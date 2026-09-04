//! Spotlight steps for Group detail (`/permission/groups/:id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Group detail header intro.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-intro",
    title = "This group",
    spotlight = "gauge-group-detail-header",
    position = "bottom",
    order = 10
)]
#[component]
pub fn GroupDetailIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-intro",
        "This page is one group, one named set of people. Owners manage the set (edits, owners, members, and join requests). Members are who belongs.",
        Some("When this group is on a permission allow list, members share that key."),
        &[],
    )
}

/// Request Access on group detail.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-request",
    title = "Ask to join",
    spotlight = "gauge-group-request-access",
    position = "bottom",
    order = 20
)]
#[component]
pub fn GroupDetailRequestHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-request",
        "Request Access asks the owners to add you to this group.",
        Some("Track the ask under Requests. The button shows only when you are allowed to ask."),
        &[],
    )
}

/// Add an owner to the group.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-owners-add",
    title = "Add an owner",
    spotlight = "gauge-group-owners-picker",
    position = "bottom",
    order = 30
)]
#[component]
pub fn GroupDetailOwnersAddHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-owners-add",
        "Owners can edit this group, manage owners, and manage members.",
        Some("Use the owners picker to add a user (only users can be owners). Prefer a small owner set so it stays clear who can change membership."),
        &[],
    )
}

/// Remove an owner from the group.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-owners-remove",
    title = "Remove an owner",
    spotlight = "gauge-group-owner-remove",
    position = "left",
    order = 40
)]
#[component]
pub fn GroupDetailOwnersRemoveHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-owners-remove",
        "Each owner row has a menu (⋮). Remove Owner takes that person off the owner list after you confirm.",
        Some("They lose management rights on this group."),
        &[],
    )
}

/// Edit group name and description.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-edit",
    title = "Edit the group",
    spotlight = "gauge-group-edit-form",
    position = "top",
    order = 50
)]
#[component]
pub fn GroupDetailEditHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-edit",
        "These fields describe the set. Edits stay local until Save Changes. If you cannot edit, you can still read the labels.",
        None,
        &[
            "Name: how people find this set",
            "Description: who belongs",
        ],
    )
}

/// Show History on group detail.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-history",
    title = "Group history",
    spotlight = "gauge-group-show-history",
    position = "top",
    order = 60
)]
#[component]
pub fn GroupDetailHistoryHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-history",
        "Show History lists past membership and owner changes when you are allowed to view them.",
        Some("Denied viewers see a clear \"not authorized\" message instead of rows."),
        &[],
    )
}

/// Delete group action.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-delete",
    title = "Delete this group",
    spotlight = "gauge-group-delete",
    position = "top",
    order = 70
)]
#[component]
pub fn GroupDetailDeleteHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-delete",
        "Delete Group removes this set. Treat it as permanent.",
        Some("You return to the groups list afterward. Only use it when the set should no longer exist."),
        &[],
    )
}

/// Save group edits.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-save",
    title = "Save group changes",
    spotlight = "gauge-group-save",
    position = "top",
    order = 80
)]
#[component]
pub fn GroupDetailSaveHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-save",
        "Save Changes writes your name and description edits to this group.",
        Some("Membership and owner changes from the pickers apply when you add or remove; they do not wait on this Save button."),
        &[],
    )
}

/// Add a member to the group.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-members-add",
    title = "Add a member",
    spotlight = "gauge-group-members-picker",
    position = "bottom",
    order = 90
)]
#[component]
pub fn GroupDetailMembersAddHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-members-add",
        "Members are who belongs in this set. When this group is on a permission allow list, these members share that key.",
        Some("Use the picker to add a user or a nested group."),
        &[],
    )
}

/// Remove a member from the group.
#[help_spotlight_step(
    route = "/permission/groups/:id",
    feature_highlight = "group-detail-members-remove",
    title = "Remove a member",
    spotlight = "gauge-group-member-remove",
    position = "left",
    order = 100
)]
#[component]
pub fn GroupDetailMembersRemoveHelp() -> impl IntoView {
    help_stack(
        "help-step-group-detail-members-remove",
        "Each member row has a menu (⋮). Remove takes that person or nested group out of the set after you confirm.",
        Some("They lose shared access that came only through this group."),
        &[],
    )
}
