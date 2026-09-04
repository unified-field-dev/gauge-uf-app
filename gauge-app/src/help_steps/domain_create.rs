//! Spotlight steps for Create Domain (`/permission/create-domain`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Create-domain page intro.
#[help_spotlight_step(
    route = "/permission/create-domain",
    feature_highlight = "create-domain-intro",
    title = "What a domain is",
    spotlight = "gauge-create-domain-page",
    position = "top",
    order = 10
)]
#[component]
pub fn CreateDomainIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-create-domain-intro",
        "A domain is a folder label for permissions, a shelf in the key cabinet.",
        Some("It does not grant access by itself. It keeps related keys easy to find when you create or edit permissions."),
        &[],
    )
}

/// Create-domain form fields.
#[help_spotlight_step(
    route = "/permission/create-domain",
    feature_highlight = "create-domain-form",
    title = "Name the shelf",
    spotlight = "gauge-create-domain-form",
    position = "bottom",
    order = 20
)]
#[component]
pub fn CreateDomainFormHelp() -> impl IntoView {
    help_stack(
        "help-step-create-domain-form",
        "Fill in the shelf label and what belongs on it. After you create it, pick this domain when making or editing permissions.",
        Some("Prefer a name others will recognize (for example \"Billing\")."),
        &[
            "Domain name: short shelf label",
            "Description: what belongs on this shelf",
        ],
    )
}

/// Cancel create domain.
#[help_spotlight_step(
    route = "/permission/create-domain",
    feature_highlight = "create-domain-cancel",
    title = "Cancel domain",
    spotlight = "gauge-create-domain-cancel",
    position = "top",
    order = 30
)]
#[component]
pub fn CreateDomainCancelHelp() -> impl IntoView {
    help_stack(
        "help-step-create-domain-cancel",
        "Cancel discards this domain draft and returns you to the permissions list.",
        Some("Nothing is saved until you press Create Domain."),
        &[],
    )
}

/// Submit create domain.
#[help_spotlight_step(
    route = "/permission/create-domain",
    feature_highlight = "create-domain-submit",
    title = "Create the domain",
    spotlight = "gauge-create-domain-submit",
    position = "top",
    order = 40
)]
#[component]
pub fn CreateDomainSubmitHelp() -> impl IntoView {
    help_stack(
        "help-step-create-domain-submit",
        "Create Domain saves the shelf. You can then assign it when you create or edit permissions.",
        Some("The app may take you toward creating a permission next."),
        &[],
    )
}
