//! Permission manifest for the Permission (gauge) admin app.

use uf_product_macros::UfPermissionManifest;

/// Admin permission for gauge-app mutating server functions and privileged search.
///
/// Synced into the `gauge` domain; server functions gate with
/// `#[uf_product_macros::server(permission = "GaugeAdmin")]`.
#[allow(clippy::expl_impl_clone_on_copy)]
#[derive(UfPermissionManifest)]
#[permission_manifest(
    domain_key = "gauge",
    domain_name = "Gauge",
    domain_description = "Permission administration"
)]
pub enum GaugePermission {
    /// Administer permissions, groups, domains, and principal search.
    #[permission(description = "Administer permissions, groups, and domains")]
    GaugeAdmin,
}
