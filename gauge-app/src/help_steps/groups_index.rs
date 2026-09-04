//! Spotlight steps for the Groups index (`/permission/groups`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Groups index intro.
#[help_spotlight_step(
    route = "/permission/groups",
    feature_highlight = "groups-intro",
    title = "What a group is",
    spotlight = "gauge-groups-page",
    position = "top",
    order = 10
)]
#[component]
pub fn GroupsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-groups-intro",
        "A group is a named set of people (and sometimes other groups).",
        Some("Put a group on an allow list once instead of adding each person. Groups also have owners who manage membership and answer join requests."),
        &[],
    )
}

/// Create Group call to action.
#[help_spotlight_step(
    route = "/permission/groups",
    feature_highlight = "groups-create-cta",
    title = "Create a group",
    spotlight = "gauge-groups-create",
    position = "bottom",
    order = 20
)]
#[component]
pub fn GroupsCreateCtaHelp() -> impl IntoView {
    help_stack(
        "help-step-groups-create-cta",
        "Create Group opens the form for a new set of people.",
        Some("After create, you add members and owners on the detail page. You can click now or keep touring."),
        &[],
    )
}

/// Search field on the groups list.
#[help_spotlight_step(
    route = "/permission/groups",
    feature_highlight = "groups-search",
    title = "Find a group",
    spotlight = "gauge-groups-search",
    position = "bottom",
    order = 30
)]
#[component]
pub fn GroupsSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-groups-search",
        "Search by group name or description when the list is long.",
        Some("Type part of a label to narrow results; clear the box to see every group again."),
        &[],
    )
}

/// Groups list rows.
#[help_spotlight_step(
    route = "/permission/groups",
    feature_highlight = "groups-list",
    title = "Browse groups",
    spotlight = "gauge-groups-list",
    position = "top",
    order = 40
)]
#[component]
pub fn GroupsListHelp() -> impl IntoView {
    help_stack(
        "help-step-groups-list",
        "Each row is one group. Scan before you open. An empty list means create a group or widen the search.",
        None,
        &[
            "Name: how people find the set",
            "Description: who belongs here",
        ],
    )
}

/// Open action on a group row.
#[help_spotlight_step(
    route = "/permission/groups",
    feature_highlight = "groups-open",
    title = "Open a group",
    spotlight = "gauge-group-row-open",
    position = "left",
    order = 50
)]
#[component]
pub fn GroupsOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-groups-open",
        "Open takes you to that group's page: owners, members, history, and edit controls when you are allowed.",
        Some("Click Open on any row for the full story."),
        &[],
    )
}
