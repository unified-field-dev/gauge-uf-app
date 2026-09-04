//! Spotlight steps for the Requests index (`/permission/requests`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Requests index intro.
#[help_spotlight_step(
    route = "/permission/requests",
    feature_highlight = "requests-intro",
    title = "Access requests",
    spotlight = "gauge-requests-page",
    position = "top",
    order = 10
)]
#[component]
pub fn RequestsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-requests-intro",
        "A request is a written ask for a permission key or group membership.",
        Some("Owners approve or deny. Nothing changes until someone decides. This page has two queues: asks waiting on you, and asks you submitted."),
        &[],
    )
}

/// Needs Review queue.
#[help_spotlight_step(
    route = "/permission/requests",
    feature_highlight = "requests-needs-review",
    title = "Needs Review",
    spotlight = "gauge-requests-needs-review",
    position = "top",
    order = 20
)]
#[component]
pub fn RequestsNeedsReviewHelp() -> impl IntoView {
    help_stack(
        "help-step-requests-needs-review",
        "Needs Review lists asks you can decide (as an owner or eligible reviewer).",
        Some("If the list is empty, nothing is waiting on you."),
        &[
            "Target: which permission or group",
            "Kind: permission vs group",
            "Status: usually pending here",
        ],
    )
}

/// Open a Needs Review row.
#[help_spotlight_step(
    route = "/permission/requests",
    feature_highlight = "requests-review-open",
    title = "Open a review item",
    spotlight = "gauge-requests-review-open",
    position = "left",
    order = 30
)]
#[component]
pub fn RequestsReviewOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-requests-review-open",
        "Open a Needs Review row to read the reason and, when you can review, choose Approve or Deny on the detail page.",
        Some("Click Open when you are ready to decide."),
        &[],
    )
}

/// My Requests queue.
#[help_spotlight_step(
    route = "/permission/requests",
    feature_highlight = "requests-mine",
    title = "My Requests",
    spotlight = "gauge-requests-mine",
    position = "top",
    order = 40
)]
#[component]
pub fn RequestsMineHelp() -> impl IntoView {
    help_stack(
        "help-step-requests-mine",
        "My Requests is your outbox: pending, approved, or denied.",
        Some("Same columns as Needs Review, for asks you submitted. Use it to track whether owners have answered yet."),
        &[],
    )
}

/// Open a My Requests row.
#[help_spotlight_step(
    route = "/permission/requests",
    feature_highlight = "requests-mine-open",
    title = "Open your request",
    spotlight = "gauge-requests-mine-open",
    position = "left",
    order = 50
)]
#[component]
pub fn RequestsMineOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-requests-mine-open",
        "Open a My Requests row to read the full story and status.",
        Some("You do not approve your own request here; you are reading the outcome of an ask you sent."),
        &[],
    )
}
