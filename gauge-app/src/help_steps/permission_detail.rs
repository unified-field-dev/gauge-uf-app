//! Spotlight steps for Permission detail (`/permission/permissions/:id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Permission detail header intro.
#[help_spotlight_step(
    route = "/permission/permissions/:id",
    feature_highlight = "permission-detail-intro",
    title = "This permission",
    spotlight = "gauge-perm-detail-header",
    position = "bottom",
    order = 10
)]
#[component]
pub fn PermissionDetailIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-detail-intro",
        "This page is one permission, one key. Here you see who owns it, who may use it (the allow list), and how it changed over time.",
        Some("Editing needs owner or admin rights; everyone signed in can still learn the layout."),
        &[],
    )
}

/// Request Access on permission detail.
#[help_spotlight_step(
    route = "/permission/permissions/:id",
    feature_highlight = "permission-detail-request",
    title = "Ask for access",
    spotlight = "gauge-perm-request-access",
    position = "bottom",
    order = 20
)]
#[component]
pub fn PermissionDetailRequestHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-detail-request",
        "If you need this key and cannot change the allow list, Request Access sends a written ask to the owners.",
        Some("They approve or deny it under Requests. The button appears only when you are allowed to ask."),
        &[],
    )
}

/// Edit permission definition fields.
#[help_spotlight_step(
    route = "/permission/permissions/:id",
    feature_highlight = "permission-detail-edit",
    title = "Edit the definition",
    spotlight = "gauge-perm-edit-form",
    position = "top",
    order = 30
)]
#[component]
pub fn PermissionDetailEditHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-detail-edit",
        "These fields describe the key. Edits stay local until you press Save Changes. If you cannot edit, the fields still show how the key is defined.",
        None,
        &[
            "Name: search label",
            "Description: when to use it",
            "Owners group: who maintains this key",
            "Domain: which shelf it sits on",
        ],
    )
}

/// Show History on permission detail.
#[help_spotlight_step(
    route = "/permission/permissions/:id",
    feature_highlight = "permission-detail-history",
    title = "Change history",
    spotlight = "gauge-show-history",
    position = "top",
    order = 40
)]
#[component]
pub fn PermissionDetailHistoryHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-detail-history",
        "Show History opens a log of past edits and allow-list changes for this permission.",
        Some("If you are not allowed to view it, you see a clear \"not authorized\" message instead of rows. The dialog stays closed until you open it."),
        &[],
    )
}

/// Delete permission action.
#[help_spotlight_step(
    route = "/permission/permissions/:id",
    feature_highlight = "permission-detail-delete",
    title = "Delete this permission",
    spotlight = "gauge-perm-delete",
    position = "top",
    order = 50
)]
#[component]
pub fn PermissionDetailDeleteHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-detail-delete",
        "Delete Permission removes this key from the cabinet. Treat it as permanent.",
        Some("You need owner or admin rights. After delete you return to the permissions list."),
        &[],
    )
}

/// Save permission edits.
#[help_spotlight_step(
    route = "/permission/permissions/:id",
    feature_highlight = "permission-detail-save",
    title = "Save changes",
    spotlight = "gauge-perm-save",
    position = "top",
    order = 60
)]
#[component]
pub fn PermissionDetailSaveHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-detail-save",
        "Save Changes writes your edits to this permission. Nothing on the form is live until you save.",
        Some("Use it after you change name, description, owners, or domain."),
        &[],
    )
}

/// Add principal to allow list.
#[help_spotlight_step(
    route = "/permission/permissions/:id",
    feature_highlight = "permission-detail-allow-add",
    title = "Add to the allow list",
    spotlight = "gauge-perm-allow-picker",
    position = "bottom",
    order = 70
)]
#[component]
pub fn PermissionDetailAllowAddHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-detail-allow-add",
        "The allow list is who holds this key: people or groups that inherit the capability.",
        Some("Use the picker to search and add a principal. Adding someone changes real access; only do it when you mean to grant the key."),
        &[],
    )
}

/// Remove principal from allow list.
#[help_spotlight_step(
    route = "/permission/permissions/:id",
    feature_highlight = "permission-detail-allow-remove",
    title = "Remove from the allow list",
    spotlight = "gauge-perm-allow-remove",
    position = "left",
    order = 80
)]
#[component]
pub fn PermissionDetailAllowRemoveHelp() -> impl IntoView {
    help_stack(
        "help-step-permission-detail-allow-remove",
        "Each allow-list row has a menu (⋮). Remove takes that person or group off the key after you confirm.",
        Some("Use it when someone should no longer hold this capability."),
        &[],
    )
}
