use leptos::prelude::*;
use leptos_router::components::A;
use uf_product::components::{Body1, Caption1, Card, EmptyState, SkeletonItemSize};
use uf_product::components::{
    CardContent, CardSectionBorder, ContentContainer, SpacingHorizontal, SpacingInset, SpacingSize,
    SpacingVertical, Title3,
};
use uf_product::primitives::{
    Button, ButtonAppearance, Field, Flex, FlexAlign, FlexGap, FlexJustify, Input, InputAppearance,
    MessageBar, MessageBarIntent, SkeletonItem,
};

use crate::server::list_groups;

/// Permission group list: search box, create-group entry point, and per-row open links.
#[component]
pub fn GroupsIndexPage() -> impl IntoView {
    let search = RwSignal::new(String::new());
    let groups = Resource::new(
        move || search.get(),
        move |query| async move {
            let q = query.trim().to_string();
            let filter = if q.is_empty() { None } else { Some(q) };
            list_groups(filter).await
        },
    );

    view! {
        <ContentContainer max_width="1100px" data_testid="gauge-groups-index">
            <Flex vertical=true gap=FlexGap::Medium>
                <Card>
                    <Flex
                        justify=FlexJustify::SpaceBetween
                        align=FlexAlign::Center
                        gap=FlexGap::Medium
                        padding=SpacingSize::Size200.inset()
                    >
                        <Flex vertical=true gap=FlexGap::Small>
                            <Title3>"Permission Groups"</Title3>
                            <Caption1>
                                "Maintain owner and membership group structures for permission control."
                            </Caption1>
                        </Flex>
                        <A href=crate::paths::CREATE_GROUP>
                            <Button appearance=ButtonAppearance::Primary>
                                "Create Group"
                            </Button>
                        </A>
                    </Flex>
                </Card>

                <Card>
                    <Flex vertical=true gap=FlexGap::Small padding=SpacingSize::Size160.inset()>
                        <Field label="Search">
                            <Input
                                bind=search
                                appearance=InputAppearance::with_placeholder("Search by group name or description")
                            />
                        </Field>
                    </Flex>
                </Card>

                <Card>
                    <Transition fallback=move || view! {
                        <CardContent>
                            <Flex vertical=true gap=FlexGap::Small>
                                {(0..3).map(|_| view! {
                                    <SkeletonItem
                                        size=Signal::from(SkeletonItemSize::S32)
                                        width="100%".to_string()
                                    />
                                }).collect_view()}
                            </Flex>
                        </CardContent>
                    }>
                        {move || {
                            match groups.get() {
                                None => view! {
                                    <CardContent>
                                        <Flex vertical=true gap=FlexGap::Small>
                                            {(0..3).map(|_| view! {
                                                <SkeletonItem
                                                    size=Signal::from(SkeletonItemSize::S32)
                                                    width="100%".to_string()
                                                />
                                            }).collect_view()}
                                        </Flex>
                                    </CardContent>
                                }.into_any(),
                                Some(Ok(items)) if items.is_empty() => {
                                    view! {
                                        <CardContent>
                                            <EmptyState
                                                message="No groups found."
                                                description="Create a group to get started."
                                            />
                                        </CardContent>
                                    }
                                        .into_any()
                                }
                                Some(Ok(items)) => {
                                    view! {
                                        <Flex vertical=true gap=FlexGap::Small padding=SpacingInset::symmetric(SpacingHorizontal::M, SpacingVertical::S)>
                                            <For each=move || items.clone() key=|row| row.id.clone() let:row>
                                                <>
                                                    <Flex
                                                        justify=FlexJustify::SpaceBetween
                                                        align=FlexAlign::Center
                                                        gap=FlexGap::Medium
                                                        padding=SpacingSize::Size120.inset()
                                                    >
                                                        <Flex vertical=true gap=FlexGap::Small>
                                                            <Body1>{row.name.clone()}</Body1>
                                                            <Caption1>{row.description.clone()}</Caption1>
                                                        </Flex>
                                                        <A href=format!("/permission/groups/{}", row.id)>
                                                            <Button appearance=ButtonAppearance::Subtle>
                                                                "Open"
                                                            </Button>
                                                        </A>
                                                    </Flex>
                                                    <CardSectionBorder />
                                                </>
                                            </For>
                                        </Flex>
                                    }.into_any()
                                }
                                Some(Err(err)) => {
                                    view! {
                                        <CardContent>
                                            <MessageBar intent=MessageBarIntent::Error>
                                                {format!("Failed to load groups: {err}")}
                                            </MessageBar>
                                        </CardContent>
                                    }
                                        .into_any()
                                }
                            }
                        }}
                    </Transition>
                </Card>
            </Flex>
        </ContentContainer>
    }
}
