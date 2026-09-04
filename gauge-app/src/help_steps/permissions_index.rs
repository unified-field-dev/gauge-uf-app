//! Spotlight steps for the Permissions index (`/permission/permissions`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro: key-cabinet metaphor and Permission vocabulary.
#[help_spotlight_step(
    route = "/permission/permissions",
    feature_highlight = "permission-intro",
    title = "Welcome to Permission",
    order = 10
)]
#[component]
pub fn PermissionIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-intro",
        "Permission is where this product decides who is allowed to do what. Think of it like a key cabinet.",
        Some("Anyone signed in can look around. Changing grants still needs the right ownership or admin rights. We will walk each screen one piece at a time."),
        &[
            "Permission: a named key (a capability)",
            "Group: people who can share keys",
            "Domain: a labeled shelf for related keys",
            "Request: asking for a key you do not have",
        ],
    )
}

/// Left navigation destinations.
#[help_spotlight_step(
    route = "/permission/permissions",
    feature_highlight = "permission-nav",
    title = "Finding your way",
    spotlight = "permission-left-nav",
    position = "right",
    order = 20
)]
#[component]
pub fn PermissionNavHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-nav",
        "Use the left menu to move around Permission: Permissions to browse named keys, Create Permission to add a new key, Create Domain to add a shelf label, Requests for asks waiting for a decision, Groups for people sets that share keys, and Create Group to start a new set.",
        Some("Opening another page starts that page's short tour. Help → Replay restarts this page's tour."),
        &[],
    )
}

/// Create Permission call to action.
#[help_spotlight_step(
    route = "/permission/permissions",
    feature_highlight = "permission-create-cta",
    title = "Create a permission",
    spotlight = "gauge-permissions-create",
    position = "bottom",
    order = 30
)]
#[component]
pub fn PermissionCreateCtaHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-create-cta",
        "Create Permission opens the form for a new key in the cabinet.",
        Some("You will choose a clear name, a short description, and a domain shelf. After create, you build the allow list on the detail page. You can click now, or keep touring and use the left menu later."),
        &[],
    )
}

/// Search field on the permissions list.
#[help_spotlight_step(
    route = "/permission/permissions",
    feature_highlight = "permission-search",
    title = "Find a permission",
    spotlight = "gauge-permissions-search",
    position = "bottom",
    order = 40
)]
#[component]
pub fn PermissionSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-search",
        "Use search when the catalog is long. Type part of a permission name or description; the list updates as you type. Clear the box to see every permission again.",
        Some("Tip: search is about finding a key quickly. It does not change who holds it."),
        &[],
    )
}

/// Permissions list rows.
#[help_spotlight_step(
    route = "/permission/permissions",
    feature_highlight = "permission-list",
    title = "Browse permissions",
    spotlight = "gauge-permissions-list",
    position = "top",
    order = 50
)]
#[component]
pub fn PermissionListHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-list",
        "Each row is one permission, one key. Scan the list to compare keys before you open one. If the list is empty, create a permission or widen the search.",
        None,
        &[
            "Name: how people will search for it",
            "Description: when that key should be used",
        ],
    )
}

/// Open action on a permission row.
#[help_spotlight_step(
    route = "/permission/permissions",
    feature_highlight = "permission-open",
    title = "Open a permission",
    spotlight = "gauge-permission-row-open",
    position = "left",
    order = 60
)]
#[component]
pub fn PermissionOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-open",
        "Open takes you to that permission's page. There you will see who owns the key, who is on the allow list, and (when you are allowed) edit, history, and request controls.",
        Some("Click Open on any row when you want the full story for one key."),
        &[],
    )
}
