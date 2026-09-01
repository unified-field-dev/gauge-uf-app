//! Host composition: wire Gauge `actor_can` into uf-product permission gates.

use std::sync::Arc;

use leptos::prelude::*;

/// Gauge-backed [`uf_product::permissions::PermissionBackend`].
pub struct GaugePermissionBackend;

#[cfg(feature = "ssr")]
#[async_trait::async_trait]
impl uf_product::permissions::PermissionBackend for GaugePermissionBackend {
    async fn has_permission(&self, permission_name: &str) -> Result<bool, ServerFnError> {
        let ctx = higgs::Higgs::from_request().await?;
        let valence = ctx
            .valence()
            .map_err(|e| ServerFnError::new(format!("Failed to build Valence: {e}")))?;
        let _caller = gauge::instrumentation::PermissionCheckCallerGuard::new("permission_backend");
        gauge::service::actor_can(&valence, permission_name)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to check permission: {e}")))
    }
}

/// Call from host shell bootstrap before serving routes (SSR).
#[cfg(feature = "ssr")]
pub fn wire_gauge_permissions() {
    uf_product::permissions::provide_permission_backend(Arc::new(GaugePermissionBackend));
}
