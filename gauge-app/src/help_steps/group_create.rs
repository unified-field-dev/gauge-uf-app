//! Spotlight steps for Create Group (`/permission/create-group`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Create-group page intro.
#[help_spotlight_step(
    route = "/permission/create-group",
    feature_highlight = "create-group-intro",
    title = "Creating a group",
    spotlight = "gauge-create-group-page",
    position = "top",
    order = 10
)]
#[component]
pub fn CreateGroupIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-create-group-intro",
        "You are starting an empty set of people.",
        Some("After create, open the group to add members and owners. Groups can contain users and nested groups."),
        &[],
    )
}

/// Create-group form fields.
#[help_spotlight_step(
    route = "/permission/create-group",
    feature_highlight = "create-group-form",
    title = "Name and description",
    spotlight = "gauge-create-group-form",
    position = "bottom",
    order = 20
)]
#[component]
pub fn CreateGroupFormHelp() -> impl IntoView {
    help_stack(
        "help-step-create-group-form",
        "Give the set a clear label and a short note about who belongs.",
        Some("Prefer \"Support on-call\" over \"Group 3\". A clear name makes later allow-list picks safer."),
        &[
            "Display name: something people recognize",
            "Description: who belongs in this set",
        ],
    )
}

/// Cancel create group.
#[help_spotlight_step(
    route = "/permission/create-group",
    feature_highlight = "create-group-cancel",
    title = "Cancel create group",
    spotlight = "gauge-create-group-cancel",
    position = "top",
    order = 30
)]
#[component]
pub fn CreateGroupCancelHelp() -> impl IntoView {
    help_stack(
        "help-step-create-group-cancel",
        "Cancel discards the draft and returns to the Groups list.",
        Some("Nothing is saved until Create."),
        &[],
    )
}

/// Submit create group.
#[help_spotlight_step(
    route = "/permission/create-group",
    feature_highlight = "create-group-submit",
    title = "Save the new group",
    spotlight = "gauge-create-group-submit",
    position = "top",
    order = 40
)]
#[component]
pub fn CreateGroupSubmitHelp() -> impl IntoView {
    help_stack(
        "help-step-create-group-submit",
        "Create Group saves the set and opens its detail page so you can add owners and members next.",
        None,
        &[],
    )
}
