import { test, expect, seedAuth, waitForHydrated, expectMutationDenied } from "./fixtures";

test.describe("e2e.perm.index", () => {
  test("e2e.perm.index.load_happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/permission", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("gauge-permissions-index")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText("CanDeploy", { exact: true })).toBeVisible({ timeout: 60_000 });
  });

  test("e2e.perm.index.search_match", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/permission/permissions", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByText("CanDeploy", { exact: true })).toBeVisible({ timeout: 60_000 });
    await page.getByPlaceholder(/Search by permission/i).fill("CanDeploy");
    await expect(page.getByText("CanDeploy", { exact: true })).toBeVisible();
  });

  test("e2e.perm.index.search_no_match", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/permission/permissions", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await page.getByPlaceholder(/Search by permission/i).fill("zz-no-such-perm");
    await expect(page.getByText(/No permissions found/i)).toBeVisible({
      timeout: 30_000,
    });
  });

  test("e2e.perm.index.open_row", async ({ page }) => {
    const { fixtures } = await seedAuth(page, "admin");
    await page.goto("/permission/permissions", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await page.locator(`a[href="/permission/permissions/${fixtures.permission_id}"]`).click();
    await expect(page).toHaveURL(new RegExp(`/permission/permissions/${fixtures.permission_id}`));
  });

  test("e2e.perm.index.create_cta", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/permission/permissions", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await page
      .getByTestId("gauge-permissions-index")
      .getByRole("link", { name: /Create Permission/i })
      .or(
        page
          .getByTestId("gauge-permissions-index")
          .getByRole("button", { name: /Create Permission/i }),
      )
      .first()
      .click();
    await expect(page).toHaveURL(/\/permission\/create-permission/);
  });
});

test.describe("e2e.domain.create", () => {
  test("e2e.domain.create.happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/permission/create-domain", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    const name = `E2E-Domain-${Date.now()}`;
    await page.getByLabel(/Domain name/i).fill(name);
    await page.getByLabel(/Description/i).fill("created by e2e");
    await page.getByRole("button", { name: "Create Domain", exact: true }).click();
    await expect(page).toHaveURL(/\/permission\/create-permission/, { timeout: 60_000 });
    await waitForHydrated(page);
    const domainSelect = page.locator("select").first();
    await expect(domainSelect.locator(`option`, { hasText: name })).toBeAttached({
      timeout: 60_000,
    });
  });

  test("e2e.domain.create.no_admin", async ({ page }) => {
    await seedAuth(page, "requestor");
    await page.goto("/permission/create-domain", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await page.getByLabel(/Domain name/i).fill("ShouldFail");
    await page.getByLabel(/Description/i).fill("no admin");
    await page.getByRole("button", { name: "Create Domain", exact: true }).click();
    await expectMutationDenied(page);
  });

  test("e2e.domain.create.empty_name", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/permission/create-domain", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await page.getByRole("button", { name: "Create Domain", exact: true }).click();
    await expect(page.getByText(/Domain name is required/i)).toBeVisible({
      timeout: 30_000,
    });
  });
});

test.describe("e2e.perm.create", () => {
  test("e2e.perm.create.happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/permission/create-permission", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    const name = `E2E-Perm-${Date.now()}`;
    await page.getByLabel(/Display name/i).fill(name);
    await page.getByLabel(/Description/i).fill("e2e permission");
    // Native/Orbital Select under Domain field
    const domainSelect = page.locator("select").first();
    await expect(domainSelect).toBeVisible({ timeout: 60_000 });
    await domainSelect.selectOption({ label: "Ops" });
    await page.getByRole("button", { name: /Create Permission/i }).click();
    await expect(page).toHaveURL(/\/permission\/permissions\//, { timeout: 60_000 });
    await expect(page.getByLabel(/^Name$/i)).toHaveValue(name, { timeout: 30_000 });
  });

  test("e2e.perm.create.domain_required_client", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/permission/create-permission", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await page.getByLabel(/Display name/i).fill("NoDomainPerm");
    await page.getByRole("button", { name: /Create Permission/i }).click();
    await expect(page.getByText(/Permission domain is required/i)).toBeVisible({
      timeout: 30_000,
    });
  });

  test("e2e.perm.create.no_domains", async ({ page }) => {
    await seedAuth(page, "admin", false, { listDomainsMode: "empty" });
    await page.goto("/permission/create-permission", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByText(/No domains available\. Create one first/i)).toBeVisible({
      timeout: 60_000,
    });
    // Product leaves Select enabled with only the placeholder when the list is empty.
    const domainSelect = page.locator("select").first();
    await expect(domainSelect.locator("option")).toHaveCount(1);
    await expect(domainSelect.locator("option").first()).toHaveText(/Select a domain/i);
    await seedAuth(page, "admin"); // clear lab override
  });

  test("e2e.perm.create.domains_error", async ({ page }) => {
    await seedAuth(page, "admin", false, { listDomainsMode: "error" });
    await page.goto("/permission/create-permission", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.locator(".orbital-message-bar--error").first()).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.locator("select").first()).toBeDisabled();
    await seedAuth(page, "admin");
  });

  test("e2e.perm.create.no_admin", async ({ page }) => {
    await seedAuth(page, "requestor");
    await page.goto("/permission/create-permission", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await page.getByLabel(/Display name/i).fill("BlockedPerm");
    const domainSelect = page.locator("select").first();
    await expect(domainSelect).toBeVisible({ timeout: 60_000 });
    await domainSelect.selectOption({ label: "Ops" });
    await page.getByRole("button", { name: /Create Permission/i }).click();
    await expectMutationDenied(page);
  });
});
