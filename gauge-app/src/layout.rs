use lepton_shell::AppBarUserMenu;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use uf_integrations::{
    ShellAppBar, ShellAuthMenu, ShellLeftNav, UnifiedFieldAppBar, UnifiedFieldShellLayout,
};
use uf_product::components::{
    Navigation, NavigationBody, NavigationConfig, NavigationLink, NavigationMaterial,
};
use uf_product::routes::RequireAuthenticated;

use crate::paths;
use crate::AppMetadata;

/// Shell layout for the Permission app: app bar + left nav wrapping the routed page [`Outlet`].
#[component]
pub fn PermissionLayout() -> impl IntoView {
    let app_name = AppMetadata::name().to_string();
    let selected_value = RwSignal::new(None::<String>);
    let open_categories = RwSignal::new(Vec::<String>::new());

    view! {
        <div data-testid="permission-app-root">
        <UnifiedFieldShellLayout>
            <ShellAppBar slot>
                <UnifiedFieldAppBar
                    app_name=app_name
                    app_id=AppMetadata::id()
                    homepage_url="/".to_string()
                >
                    <ShellAuthMenu slot:auth_menu>
                        <AppBarUserMenu />
                    </ShellAuthMenu>
                </UnifiedFieldAppBar>
            </ShellAppBar>
            <ShellLeftNav slot>
                <Navigation config=NavigationConfig::new().with_selected_value(selected_value).with_open_categories(open_categories)>
                    <NavigationMaterial slot />
                    <NavigationBody slot>
                        <div id="permission-left-nav">
                            <NavigationLink path=paths::PERMISSIONS value=paths::PERMISSIONS icon=icondata::AiSafetyCertificateOutlined exact=true test_id="nav-permissions">"Permissions"</NavigationLink>
                            <NavigationLink path=paths::CREATE_PERMISSION value=paths::CREATE_PERMISSION icon=icondata::AiPlusCircleOutlined exact=true test_id="nav-create-permission">"Create Permission"</NavigationLink>
                            <NavigationLink path="/permission/create-domain" value="/permission/create-domain" icon=icondata::AiPlusCircleOutlined exact=true test_id="nav-create-domain">"Create Domain"</NavigationLink>
                            <NavigationLink path="/permission/requests" value="/permission/requests" icon=icondata::AiBellOutlined exact=true test_id="nav-requests">"Requests"</NavigationLink>
                            <NavigationLink path=paths::GROUPS value=paths::GROUPS icon=icondata::AiTeamOutlined exact=true test_id="nav-groups">"Groups"</NavigationLink>
                            <NavigationLink path=paths::CREATE_GROUP value=paths::CREATE_GROUP icon=icondata::AiUsergroupAddOutlined exact=true test_id="nav-create-group">"Create Group"</NavigationLink>
                        </div>
                    </NavigationBody>
                </Navigation>
            </ShellLeftNav>
            <RequireAuthenticated requires_email_verification=true>
                <Outlet />
            </RequireAuthenticated>
        </UnifiedFieldShellLayout>
        </div>
    }
}
