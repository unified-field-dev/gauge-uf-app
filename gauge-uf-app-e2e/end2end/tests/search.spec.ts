import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("e2e.search.principals", () => {
  test("e2e.search.principals_initial", async ({ page }) => {
    const { fixtures } = await seedAuth(page, "admin");
    await page.goto(`/permission/permissions/${fixtures.permission_id}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    const picker = page.getByPlaceholder(/Search users or groups/i).first();
    await picker.click();
    const listbox = page.getByRole("listbox");
    await expect(listbox).toBeVisible({ timeout: 30_000 });
    await expect(listbox.getByRole("option").first()).toBeVisible({ timeout: 30_000 });
  });

  test("e2e.search.principals_query_user", async ({ page }) => {
    const { fixtures } = await seedAuth(page, "admin");
    await page.goto(`/permission/permissions/${fixtures.permission_id}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    const picker = page.getByPlaceholder(/Search users or groups/i).first();
    await picker.click();
    await page.keyboard.type("requestor");
    const listbox = page.getByRole("listbox");
    await expect(listbox.getByRole("option", { name: /requestor/i })).toBeVisible({
      timeout: 30_000,
    });
  });

  test("e2e.search.principals_query_group", async ({ page }) => {
    const { fixtures } = await seedAuth(page, "admin");
    await page.goto(`/permission/permissions/${fixtures.permission_id}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    const picker = page.getByPlaceholder(/Search users or groups/i).first();
    await picker.click();
    await page.keyboard.type("Deployers");
    const listbox = page.getByRole("listbox");
    await expect(
      listbox.getByRole("option", { name: /^Deployers\b/i }).first(),
    ).toBeVisible({
      timeout: 30_000,
    });
  });

  test("e2e.search.principals_no_admin", async ({ page }) => {
    const { fixtures } = await seedAuth(page, "requestor");
    await page.goto(`/permission/permissions/${fixtures.permission_id}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    const picker = page.getByPlaceholder(/Search users or groups/i).first();
    await picker.click();
    await page.keyboard.type("admin");
    await expect(page.locator(".orbital-message-bar--error").first()).toBeVisible({
      timeout: 60_000,
    });
  });
});
