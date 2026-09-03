//! Spotlight steps for Request detail (`/permission/requests/:id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Request detail summary.
#[help_spotlight_step(
    route = "/permission/requests/:id",
    feature_highlight = "request-detail-intro",
    title = "Reading a request",
    spotlight = "gauge-request-detail-summary",
    position = "bottom",
    order = 10
)]
#[component]
pub fn RequestDetailIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-request-detail-intro",
        "This page is one access request. Read before you decide.",
        Some(
            "If you only submitted the ask, you are here to read status, not to approve yourself.",
        ),
        &[
            "Target: permission or group asked for",
            "Requestor: who asked",
            "Status: pending, approved, or denied",
            "Created/Updated: when it changed",
            "Reason: why they asked",
        ],
    )
}

/// Approve access request.
#[help_spotlight_step(
    route = "/permission/requests/:id",
    feature_highlight = "request-detail-approve",
    title = "Approve the ask",
    spotlight = "gauge-request-approve",
    position = "top",
    order = 20
)]
#[component]
pub fn RequestDetailApproveHelp() -> impl IntoView {
    help_stack(
        "help-step-request-detail-approve",
        "Approve grants the ask: the person gets the key or joins the group.",
        Some("The button shows only when you can review and the request is still pending. After approve, status updates on this page."),
        &[],
    )
}

/// Deny access request.
#[help_spotlight_step(
    route = "/permission/requests/:id",
    feature_highlight = "request-detail-deny",
    title = "Deny the ask",
    spotlight = "gauge-request-deny",
    position = "top",
    order = 30
)]
#[component]
pub fn RequestDetailDenyHelp() -> impl IntoView {
    help_stack(
        "help-step-request-detail-deny",
        "Deny keeps access unchanged and records the choice.",
        Some("Use it when the ask should not be granted. Same visibility rules as Approve: only when you can review a pending request."),
        &[],
    )
}
