//! Spotlight steps for Create Permission (`/permission/create-permission`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Create-permission page intro.
#[help_spotlight_step(
    route = "/permission/create-permission",
    feature_highlight = "create-permission-intro",
    title = "Creating a permission",
    spotlight = "gauge-create-perm-page",
    position = "top",
    order = 10
)]
#[component]
pub fn CreatePermissionIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-create-permission-intro",
        "You are defining a new named capability, a new key in the cabinet.",
        Some("Walk top to bottom: name the key, describe when to use it, pick a domain shelf, then create. After create, you choose who may use it on that permission's detail page."),
        &[],
    )
}

/// Create-permission form fields.
#[help_spotlight_step(
    route = "/permission/create-permission",
    feature_highlight = "create-permission-form",
    title = "Name, description, domain",
    spotlight = "gauge-create-perm-form",
    position = "bottom",
    order = 20
)]
#[component]
pub fn CreatePermissionFormHelp() -> impl IntoView {
    help_stack(
        "help-step-create-permission-form",
        "Fill the form top to bottom.",
        Some("If the domain list is empty, create a domain first. An owner group is assigned automatically for new permissions; you do not pick owners on this form."),
        &[
            "Display name: clear label people search for",
            "Description: when this key should be used",
            "Domain: which shelf this key sits on",
        ],
    )
}

/// Cancel create permission.
#[help_spotlight_step(
    route = "/permission/create-permission",
    feature_highlight = "create-permission-cancel",
    title = "Cancel create",
    spotlight = "gauge-create-perm-cancel",
    position = "top",
    order = 30
)]
#[component]
pub fn CreatePermissionCancelHelp() -> impl IntoView {
    help_stack(
        "help-step-create-permission-cancel",
        "Cancel leaves this form without creating a key and returns you to the permissions list.",
        Some("Use it when you opened Create by mistake or want to discard a half-filled draft."),
        &[],
    )
}

/// Submit create permission.
#[help_spotlight_step(
    route = "/permission/create-permission",
    feature_highlight = "create-permission-submit",
    title = "Save the new permission",
    spotlight = "gauge-create-perm-submit",
    position = "top",
    order = 40
)]
#[component]
pub fn CreatePermissionSubmitHelp() -> impl IntoView {
    help_stack(
        "help-step-create-permission-submit",
        "Create Permission saves the new key and opens its detail page so you can build the allow list next.",
        Some("You need the rights to create permissions; otherwise the action is denied and the form explains why."),
        &[],
    )
}
