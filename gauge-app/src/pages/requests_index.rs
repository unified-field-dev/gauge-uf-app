use gauge::types::{PermissionRequestStatusDto, PermissionRequestTargetKind};
use leptos::prelude::*;
use leptos_router::components::A;
use uf_product::components::{Body1, Caption1, Card, EmptyState, SkeletonItemSize};
use uf_product::components::{CardSectionBorder, ContentContainer, SpacingSize, Title3};
use uf_product::primitives::{
    Button, ButtonAppearance, Flex, FlexAlign, FlexGap, FlexJustify, MessageBar, MessageBarIntent,
    SkeletonItem,
};

use crate::server::{list_my_permission_requests, list_review_permission_requests};

const fn request_target_label(kind: &PermissionRequestTargetKind) -> &'static str {
    match kind {
        PermissionRequestTargetKind::Permission => "Permission",
        PermissionRequestTargetKind::Group => "Group",
    }
}

const fn request_status_label(status: &PermissionRequestStatusDto) -> &'static str {
    match status {
        PermissionRequestStatusDto::Pending => "PENDING",
        PermissionRequestStatusDto::Approved => "APPROVED",
        PermissionRequestStatusDto::Denied => "DENIED",
    }
}

/// Access request inbox: a "Needs Review" queue for eligible reviewers alongside
/// the current actor's own submitted requests.
#[component]
// Dual inbox + decision actions live here; split later if modules stay cohesive.
#[allow(clippy::too_many_lines)]
pub fn RequestsIndexPage() -> impl IntoView {
    let my_requests = Resource::new(
        || (),
        |()| async move { list_my_permission_requests().await },
    );
    let review_requests = Resource::new(
        || (),
        |()| async move { list_review_permission_requests().await },
    );

    view! {
        <ContentContainer max_width="1100px" data_testid="gauge-requests-index">
            <Flex vertical=true gap=FlexGap::Medium>
                <Card>
                    <Flex vertical=true gap=FlexGap::Small padding=SpacingSize::Size200.inset()>
                        <Title3>"Permission Requests"</Title3>
                        <Caption1>
                            "Review incoming requests and track requests you have submitted."
                        </Caption1>
                    </Flex>
                </Card>

                <Card>
                    <Flex vertical=true gap=FlexGap::Small padding=SpacingSize::Size200.inset()>
                        <Title3>"Needs Review"</Title3>
                        <Transition fallback=move || view! {
                            <Flex vertical=true gap=FlexGap::Small>
                                {(0..2).map(|_| view! {
                                    <SkeletonItem
                                        size=Signal::from(SkeletonItemSize::S32)
                                        width="100%".to_string()
                                    />
                                }).collect_view()}
                            </Flex>
                        }>
                            {move || {
                                match review_requests.get() {
                                    None => view! {
                                        <Flex vertical=true gap=FlexGap::Small>
                                            {(0..2).map(|_| view! {
                                                <SkeletonItem
                                                    size=Signal::from(SkeletonItemSize::S32)
                                                    width="100%".to_string()
                                                />
                                            }).collect_view()}
                                        </Flex>
                                    }.into_any(),
                                    Some(Ok(rows)) if rows.is_empty() => view! {
                                        <EmptyState message="No requests are waiting for your review." />
                                    }.into_any(),
                                    Some(Ok(rows)) => view! {
                                        <Flex vertical=true gap=FlexGap::Small>
                                            <For each=move || rows.clone() key=|row| row.id.clone() let:row>
                                                <>
                                                    <Flex
                                                        justify=FlexJustify::SpaceBetween
                                                        align=FlexAlign::Center
                                                        gap=FlexGap::Small
                                                        padding=SpacingSize::Size120.inset()
                                                    >
                                                        <Flex vertical=true gap=FlexGap::Small>
                                                            <Body1>{row.target_label.clone()}</Body1>
                                                            <Caption1>
                                                                {format!(
                                                                    "{} - {}",
                                                                    request_target_label(&row.target_kind),
                                                                    request_status_label(&row.status)
                                                                )}
                                                            </Caption1>
                                                        </Flex>
                                                        <A href=format!("/permission/requests/{}", row.id)>
                                                            <Button appearance=ButtonAppearance::Subtle>
                                                                "Open"
                                                            </Button>
                                                        </A>
                                                    </Flex>
                                                    <CardSectionBorder />
                                                </>
                                            </For>
                                        </Flex>
                                    }.into_any(),
                                    Some(Err(err)) => view! {
                                        <MessageBar intent=MessageBarIntent::Error>
                                            {format!("Failed to load review requests: {err}")}
                                        </MessageBar>
                                    }.into_any(),
                                }
                            }}
                        </Transition>
                    </Flex>
                </Card>

                <Card>
                    <Flex vertical=true gap=FlexGap::Small padding=SpacingSize::Size200.inset()>
                        <Title3>"My Requests"</Title3>
                        <Transition fallback=move || view! {
                            <Flex vertical=true gap=FlexGap::Small>
                                {(0..2).map(|_| view! {
                                    <SkeletonItem
                                        size=Signal::from(SkeletonItemSize::S32)
                                        width="100%".to_string()
                                    />
                                }).collect_view()}
                            </Flex>
                        }>
                            {move || {
                                match my_requests.get() {
                                    None => view! {
                                        <Flex vertical=true gap=FlexGap::Small>
                                            {(0..2).map(|_| view! {
                                                <SkeletonItem
                                                    size=Signal::from(SkeletonItemSize::S32)
                                                    width="100%".to_string()
                                                />
                                            }).collect_view()}
                                        </Flex>
                                    }.into_any(),
                                    Some(Ok(rows)) if rows.is_empty() => view! {
                                        <EmptyState message="You have not created any permission requests yet." />
                                    }.into_any(),
                                    Some(Ok(rows)) => view! {
                                        <Flex vertical=true gap=FlexGap::Small>
                                            <For each=move || rows.clone() key=|row| row.id.clone() let:row>
                                                <>
                                                    <Flex
                                                        justify=FlexJustify::SpaceBetween
                                                        align=FlexAlign::Center
                                                        gap=FlexGap::Small
                                                        padding=SpacingSize::Size120.inset()
                                                    >
                                                        <Flex vertical=true gap=FlexGap::Small>
                                                            <Body1>{row.target_label.clone()}</Body1>
                                                            <Caption1>
                                                                {format!(
                                                                    "{} - {}",
                                                                    request_target_label(&row.target_kind),
                                                                    request_status_label(&row.status)
                                                                )}
                                                            </Caption1>
                                                        </Flex>
                                                        <A href=format!("/permission/requests/{}", row.id)>
                                                            <Button appearance=ButtonAppearance::Subtle>
                                                                "Open"
                                                            </Button>
                                                        </A>
                                                    </Flex>
                                                    <CardSectionBorder />
                                                </>
                                            </For>
                                        </Flex>
                                    }.into_any(),
                                    Some(Err(err)) => view! {
                                        <MessageBar intent=MessageBarIntent::Error>
                                            {format!("Failed to load your requests: {err}")}
                                        </MessageBar>
                                    }.into_any(),
                                }
                            }}
                        </Transition>
                    </Flex>
                </Card>
            </Flex>
        </ContentContainer>
    }
}
