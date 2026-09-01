import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind =
  | "anonymous"
  | "admin"
  | "requestor"
  | "outsider"
  | "unverified";

export type SeedFixtures = {
  domain_a_id: string;
  domain_b_id: string;
  permission_id: string;
  permission_name: string;
  group_id: string;
  group_name: string;
  pending_perm_request_id: string;
  pending_group_request_id: string;
  child_group_id: string;
};

/** Default: do not remint pending requests (avoids wiping grants/membership mid-suite). */
export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  refreshRequests = false,
  opts?: { listDomainsMode?: "empty" | "error" },
) {
  const res = await page.request.post("/api/test/seed-data", {
    data: {
      auth,
      refresh_requests: refreshRequests,
      list_domains_mode: opts?.listDomainsMode ?? null,
    },
  });
  expect(res.ok()).toBeTruthy();
  return res.json() as Promise<{
    ok: boolean;
    auth: string;
    fixtures: SeedFixtures;
  }>;
}

/**
 * Wait for Orbital boot overlay to finish and hydrate to mark the document ready.
 */
export async function waitForHydrated(page: Page, timeoutMs = 240_000) {
  await expect
    .poll(
      async () =>
        page.evaluate(() => {
          const html = document.documentElement;
          if (html.getAttribute("data-orbital-boot-state") === "error") {
            return "error";
          }
          if (html.getAttribute("data-orbital-hydrated") === "true") {
            return "ready";
          }
          return "loading";
        }),
      { timeout: timeoutMs },
    )
    .not.toBe("error");
  await expect
    .poll(
      async () =>
        page.evaluate(
          () => document.documentElement.getAttribute("data-orbital-hydrated") === "true",
        ),
      { timeout: timeoutMs },
    )
    .toBe(true);
  await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0, {
    timeout: 60_000,
  });
  await expect(page.getByTestId("e2e-auth-bootstrap")).toBeAttached({
    timeout: 30_000,
  });
}

/** Higgs / server-fn deny surfaces as an Orbital error MessageBar. */
export async function expectMutationDenied(page: Page) {
  await expect(page.locator(".orbital-message-bar--error").first()).toBeVisible({
    timeout: 60_000,
  });
}

export const test = base;
export { expect };
