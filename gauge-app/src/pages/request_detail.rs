use gauge::types::PermissionRequestStatusDto;
use leptos::prelude::*;
use leptos::task::spawn_local_scoped;
use leptos_router::hooks::use_params_map;
use uf_product::components::{Body1, Caption1, Card};
use uf_product::components::{ContentContainer, SpacingSize, Title3};
use uf_product::primitives::{
    Button, ButtonAppearance, Flex, FlexGap, FlexJustify, MessageBar, MessageBarIntent,
};

use crate::server::{decide_permission_request, get_permission_request};

const fn status_label(status: &PermissionRequestStatusDto) -> &'static str {
    match status {
        PermissionRequestStatusDto::Pending => "PENDING",
        PermissionRequestStatusDto::Approved => "APPROVED",
        PermissionRequestStatusDto::Denied => "DENIED",
    }
}

/// Access request detail page: target/requestor/status summary, reason, and
/// approve/deny actions for eligible reviewers on pending requests.
#[component]
pub fn RequestDetailPage() -> impl IntoView {
    let params = use_params_map();
    let request_id = Memo::new(move |_| params.read().get("id").unwrap_or_default());
    let refresh = RwSignal::new(0u64);
    let error = RwSignal::new(None::<String>);

    let request = Resource::new(
        move || (request_id.get(), refresh.get()),
        move |(id, _)| async move { get_permission_request(id).await },
    );

    let decide = move |decision: gauge::types::PermissionRequestDecision| {
        let request_id = request_id.get();
        spawn_local_scoped(async move {
            match decide_permission_request(gauge::types::PermissionRequestDecisionInput {
                request_id,
                decision,
            })
            .await
            {
                Ok(_) => refresh.update(|n| *n += 1),
                Err(err) => error.set(Some(err.to_string())),
            }
        });
    };

    view! {
        <ContentContainer max_width="900px" data_testid="gauge-request-detail">
            <Transition fallback=move || view! { <Body1>"Loading request..."</Body1> }>
                {move || {
                    request.get().map(|result| match result {
                        Ok(Some(row)) => {
                            let is_pending = row.status == PermissionRequestStatusDto::Pending;
                            view! {
                                <Flex vertical=true gap=FlexGap::Medium>
                                    <Card>
                                        <Flex vertical=true gap=FlexGap::Small padding=SpacingSize::Size200.inset()>
                                            <Title3>"Permission Request"</Title3>
                                            <Caption1>{format!("ID: {}", row.id)}</Caption1>
                                        </Flex>
                                    </Card>
                                    <Card>
                                        <Flex vertical=true gap=FlexGap::Small padding=SpacingSize::Size200.inset()>
                                            <Body1>{format!("Target: {}", row.target_label)}</Body1>
                                            <Caption1>
                                                {format!("Requestor: {}", row.requestor_user_id)}
                                            </Caption1>
                                            <Caption1>
                                                {format!("Status: {}", status_label(&row.status))}
                                            </Caption1>
                                            <Caption1>
                                                {format!("Created: {}", row.created_at)}
                                            </Caption1>
                                            <Caption1>
                                                {format!("Updated: {}", row.updated_at)}
                                            </Caption1>
                                        </Flex>
                                    </Card>
                                    <Card>
                                        <Flex vertical=true gap=FlexGap::Small padding=SpacingSize::Size200.inset()>
                                            <Title3>"Reason"</Title3>
                                            <Body1>{row.reason}</Body1>
                                        </Flex>
                                    </Card>
                                    <Show when=move || row.can_review && is_pending>
                                        <Card>
                                            <Flex justify=FlexJustify::End gap=FlexGap::Small padding=SpacingSize::Size200.inset()>
                                                <Button
                                                    appearance=ButtonAppearance::Secondary
                                                    on_click=Callback::new(move |_| {
                                                        decide(gauge::types::PermissionRequestDecision::Deny)
                                                    })
                                                >
                                                    "Deny"
                                                </Button>
                                                <Button
                                                    appearance=ButtonAppearance::Primary
                                                    on_click=Callback::new(move |_| {
                                                        decide(gauge::types::PermissionRequestDecision::Approve)
                                                    })
                                                >
                                                    "Approve"
                                                </Button>
                                            </Flex>
                                        </Card>
                                    </Show>
                                    <Show when=move || error.get().is_some()>
                                        <MessageBar intent=MessageBarIntent::Error>
                                            {move || error.get().unwrap_or_default()}
                                        </MessageBar>
                                    </Show>
                                </Flex>
                            }
                            .into_any()
                        }
                        Ok(None) => view! { <Body1>"Request not found."</Body1> }.into_any(),
                        Err(err) => view! {
                            <MessageBar intent=MessageBarIntent::Error>
                                {format!("Failed to load request: {err}")}
                            </MessageBar>
                        }
                        .into_any(),
                    })
                }}
            </Transition>
        </ContentContainer>
    }
}
