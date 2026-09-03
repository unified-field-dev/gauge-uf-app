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

/** All Permission Help inventory keys — seed as seen so non-tour specs stay quiet. */
const GAUGE_HELP_STEPS_SEEN = [
  {
    route: "/permission/create-domain",
    feature_highlight: "create-domain-intro",
    spotlight: "gauge-create-domain-page",
    replay: false,
  },
  {
    route: "/permission/create-domain",
    feature_highlight: "create-domain-form",
    spotlight: "gauge-create-domain-form",
    replay: false,
  },
  {
    route: "/permission/create-domain",
    feature_highlight: "create-domain-cancel",
    spotlight: "gauge-create-domain-cancel",
    replay: false,
  },
  {
    route: "/permission/create-domain",
    feature_highlight: "create-domain-submit",
    spotlight: "gauge-create-domain-submit",
    replay: false,
  },
  {
    route: "/permission/create-group",
    feature_highlight: "create-group-intro",
    spotlight: "gauge-create-group-page",
    replay: false,
  },
  {
    route: "/permission/create-group",
    feature_highlight: "create-group-form",
    spotlight: "gauge-create-group-form",
    replay: false,
  },
  {
    route: "/permission/create-group",
    feature_highlight: "create-group-cancel",
    spotlight: "gauge-create-group-cancel",
    replay: false,
  },
  {
    route: "/permission/create-group",
    feature_highlight: "create-group-submit",
    spotlight: "gauge-create-group-submit",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-intro",
    spotlight: "gauge-group-detail-header",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-request",
    spotlight: "gauge-group-request-access",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-owners-add",
    spotlight: "gauge-group-owners-picker",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-owners-remove",
    spotlight: "gauge-group-owner-remove",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-edit",
    spotlight: "gauge-group-edit-form",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-history",
    spotlight: "gauge-group-show-history",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-delete",
    spotlight: "gauge-group-delete",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-save",
    spotlight: "gauge-group-save",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-members-add",
    spotlight: "gauge-group-members-picker",
    replay: false,
  },
  {
    route: "/permission/groups/:id",
    feature_highlight: "group-detail-members-remove",
    spotlight: "gauge-group-member-remove",
    replay: false,
  },
  {
    route: "/permission/groups",
    feature_highlight: "groups-intro",
    spotlight: "gauge-groups-page",
    replay: false,
  },
  {
    route: "/permission/groups",
    feature_highlight: "groups-create-cta",
    spotlight: "gauge-groups-create",
    replay: false,
  },
  {
    route: "/permission/groups",
    feature_highlight: "groups-search",
    spotlight: "gauge-groups-search",
    replay: false,
  },
  {
    route: "/permission/groups",
    feature_highlight: "groups-list",
    spotlight: "gauge-groups-list",
    replay: false,
  },
  {
    route: "/permission/groups",
    feature_highlight: "groups-open",
    spotlight: "gauge-group-row-open",
    replay: false,
  },
  {
    route: "/permission/create-permission",
    feature_highlight: "create-permission-intro",
    spotlight: "gauge-create-perm-page",
    replay: false,
  },
  {
    route: "/permission/create-permission",
    feature_highlight: "create-permission-form",
    spotlight: "gauge-create-perm-form",
    replay: false,
  },
  {
    route: "/permission/create-permission",
    feature_highlight: "create-permission-cancel",
    spotlight: "gauge-create-perm-cancel",
    replay: false,
  },
  {
    route: "/permission/create-permission",
    feature_highlight: "create-permission-submit",
    spotlight: "gauge-create-perm-submit",
    replay: false,
  },
  {
    route: "/permission/permissions/:id",
    feature_highlight: "permission-detail-intro",
    spotlight: "gauge-perm-detail-header",
    replay: false,
  },
  {
    route: "/permission/permissions/:id",
    feature_highlight: "permission-detail-request",
    spotlight: "gauge-perm-request-access",
    replay: false,
  },
  {
    route: "/permission/permissions/:id",
    feature_highlight: "permission-detail-edit",
    spotlight: "gauge-perm-edit-form",
    replay: false,
  },
  {
    route: "/permission/permissions/:id",
    feature_highlight: "permission-detail-history",
    spotlight: "gauge-show-history",
    replay: false,
  },
  {
    route: "/permission/permissions/:id",
    feature_highlight: "permission-detail-delete",
    spotlight: "gauge-perm-delete",
    replay: false,
  },
  {
    route: "/permission/permissions/:id",
    feature_highlight: "permission-detail-save",
    spotlight: "gauge-perm-save",
    replay: false,
  },
  {
    route: "/permission/permissions/:id",
    feature_highlight: "permission-detail-allow-add",
    spotlight: "gauge-perm-allow-picker",
    replay: false,
  },
  {
    route: "/permission/permissions/:id",
    feature_highlight: "permission-detail-allow-remove",
    spotlight: "gauge-perm-allow-remove",
    replay: false,
  },
  {
    route: "/permission/permissions",
    feature_highlight: "permission-intro",
    spotlight: null,
    replay: false,
  },
  {
    route: "/permission/permissions",
    feature_highlight: "permission-nav",
    spotlight: "permission-left-nav",
    replay: false,
  },
  {
    route: "/permission/permissions",
    feature_highlight: "permission-create-cta",
    spotlight: "gauge-permissions-create",
    replay: false,
  },
  {
    route: "/permission/permissions",
    feature_highlight: "permission-search",
    spotlight: "gauge-permissions-search",
    replay: false,
  },
  {
    route: "/permission/permissions",
    feature_highlight: "permission-list",
    spotlight: "gauge-permissions-list",
    replay: false,
  },
  {
    route: "/permission/permissions",
    feature_highlight: "permission-open",
    spotlight: "gauge-permission-row-open",
    replay: false,
  },
  {
    route: "/permission/requests/:id",
    feature_highlight: "request-detail-intro",
    spotlight: "gauge-request-detail-summary",
    replay: false,
  },
  {
    route: "/permission/requests/:id",
    feature_highlight: "request-detail-approve",
    spotlight: "gauge-request-approve",
    replay: false,
  },
  {
    route: "/permission/requests/:id",
    feature_highlight: "request-detail-deny",
    spotlight: "gauge-request-deny",
    replay: false,
  },
  {
    route: "/permission/requests",
    feature_highlight: "requests-intro",
    spotlight: "gauge-requests-page",
    replay: false,
  },
  {
    route: "/permission/requests",
    feature_highlight: "requests-needs-review",
    spotlight: "gauge-requests-needs-review",
    replay: false,
  },
  {
    route: "/permission/requests",
    feature_highlight: "requests-review-open",
    spotlight: "gauge-requests-review-open",
    replay: false,
  },
  {
    route: "/permission/requests",
    feature_highlight: "requests-mine",
    spotlight: "gauge-requests-mine",
    replay: false,
  },
  {
    route: "/permission/requests",
    feature_highlight: "requests-mine-open",
    spotlight: "gauge-requests-mine-open",
    replay: false,
  },
] as const;

/** Default: do not remint pending requests (avoids wiping grants/membership mid-suite). */
export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  refreshRequests = false,
  opts?: { listDomainsMode?: "empty" | "error"; help_tour?: boolean },
) {
  const helpTour = opts?.help_tour ?? false;
  await page.addInitScript(
    ([enableTour, seenSteps]) => {
      try {
        if (enableTour) {
          if (!sessionStorage.getItem("uf.help.e2e_tour_cleared")) {
            localStorage.removeItem("uf.help.tour_steps");
            sessionStorage.setItem("uf.help.e2e_tour_cleared", "1");
          }
          return;
        }
        localStorage.setItem("uf.help.tour_steps", JSON.stringify(seenSteps));
      } catch {
        /* ignore */
      }
    },
    [helpTour, GAUGE_HELP_STEPS_SEEN] as const,
  );

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

async function bootState(page: Page): Promise<"ready" | "error" | "loading"> {
  return page.evaluate(() => {
    const html = document.documentElement;
    if (html.getAttribute("data-orbital-boot-state") === "error") {
      return "error";
    }
    if (html.getAttribute("data-orbital-hydrated") === "true") {
      return "ready";
    }
    return "loading";
  });
}

/**
 * Wait for Orbital hydrate to mark the document ready, then clear the boot overlay.
 *
 * Large WASM graphs can fail the first fetch on CI. Reload once when boot enters
 * `error` — never reload while still `loading` (that aborts in-flight `.wasm`
 * and sticks boot-state on error).
 */
export async function waitForHydrated(page: Page, timeoutMs = 120_000) {
  for (let attempt = 0; attempt < 2; attempt++) {
    try {
      await expect.poll(async () => bootState(page), { timeout: timeoutMs }).toBe("ready");
      break;
    } catch (err) {
      const state = await bootState(page).catch(() => "loading" as const);
      if (state === "error" && attempt === 0) {
        await page.reload({ waitUntil: "load" });
        continue;
      }
      throw err;
    }
  }
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
