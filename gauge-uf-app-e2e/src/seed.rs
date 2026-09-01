//! Harness-only seed endpoint for Playwright.

use axum::http::StatusCode;
use axum::Json;
use gauge::service;
use gauge::types::{PermissionRequestCreateInput, PermissionRequestTargetKind};
use serde::Deserialize;
use valence::Actor;

use crate::e2e_valence::{e2e_fixtures, e2e_system_valence, store_fixtures, FixtureIds};
use crate::gate_demos::{write_e2e_auth_kind, E2eAuthKind};

#[derive(Debug, Deserialize)]
pub struct SeedRequest {
    /// `anonymous` | `admin` | `requestor` | `outsider` | `unverified`
    #[serde(default = "default_auth")]
    pub auth: String,
    /// When true, mint fresh pending requests for isolation.
    #[serde(default = "default_refresh")]
    pub refresh_requests: bool,
    /// Lab-only: `empty` | `error` forces `list_domains` via `GAUGE_E2E_LIST_DOMAINS`.
    /// Omit or null clears the override.
    #[serde(default)]
    pub list_domains_mode: Option<String>,
}

fn default_auth() -> String {
    E2eAuthKind::Anonymous.as_str().to_string()
}

fn default_refresh() -> bool {
    false
}

pub async fn seed_data(
    session: tower_sessions::Session,
    Json(body): Json<SeedRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let kind = E2eAuthKind::parse(&body.auth);
    write_e2e_auth_kind(&session, kind)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match body.list_domains_mode.as_deref() {
        Some("empty") | Some("error") => {
            gauge_app::e2e_lab::set_list_domains_override(body.list_domains_mode.as_deref());
        }
        _ => gauge_app::e2e_lab::set_list_domains_override(None),
    }

    let mut fixtures = e2e_fixtures();
    if body.refresh_requests {
        fixtures = refresh_pending_requests(fixtures).await.map_err(|e| {
            log::error!("seed refresh_requests failed: {e:#}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        store_fixtures(fixtures.clone());
    }

    Ok(Json(serde_json::json!({
        "ok": true,
        "auth": kind.as_str(),
        "fixtures": {
            "domain_a_id": fixtures.domain_a_id,
            "domain_b_id": fixtures.domain_b_id,
            "permission_id": fixtures.permission_id,
            "permission_name": fixtures.permission_name,
            "group_id": fixtures.group_id,
            "group_name": fixtures.group_name,
            "pending_perm_request_id": fixtures.pending_perm_request_id,
            "pending_group_request_id": fixtures.pending_group_request_id,
            "child_group_id": fixtures.child_group_id,
        }
    })))
}

async fn refresh_pending_requests(mut fixtures: FixtureIds) -> anyhow::Result<FixtureIds> {
    let system = e2e_system_valence();
    let admin_ctx = system.with_actor(Actor::User {
        user_id: "admin".to_string(),
    });
    let requestor_ctx = system.with_actor(Actor::User {
        user_id: "requestor".to_string(),
    });

    // Grant/membership/owner specs leave requestor on CanDeploy / Deployers;
    // clear those so create_permission_request can mint fresh PENDING rows.
    if let Err(e) =
        service::revoke_permission_from_user(&fixtures.permission_id, "requestor", &admin_ctx).await
    {
        log::debug!("refresh revoke perm user (ok if absent): {e}");
    }
    if let Err(e) = service::revoke_permission_from_group(
        &fixtures.permission_id,
        &fixtures.group_id,
        &admin_ctx,
    )
    .await
    {
        log::debug!("refresh revoke perm group (ok if absent): {e}");
    }
    if let Err(e) =
        service::remove_group_member_user(&fixtures.group_id, "requestor", &admin_ctx).await
    {
        log::debug!("refresh remove group member (ok if absent): {e}");
    }
    if let Err(e) =
        service::remove_group_owner_user(&fixtures.group_id, "requestor", &admin_ctx).await
    {
        log::debug!("refresh remove group owner (ok if absent): {e}");
    }

    // Mint as requestor so outsider unauthorized_viewer can assert deny-by-viewer.
    let perm_row = service::create_permission_request(
        PermissionRequestCreateInput {
            target_kind: PermissionRequestTargetKind::Permission,
            target_id: fixtures.permission_id.clone(),
            reason: format!("e2e refresh perm {}", chrono::Utc::now().timestamp_millis()),
        },
        &requestor_ctx,
    )
    .await
    .map_err(|e| anyhow::anyhow!("refresh perm request failed: {e}"))?;
    fixtures.pending_perm_request_id = perm_row.id;

    let group_row = service::create_permission_request(
        PermissionRequestCreateInput {
            target_kind: PermissionRequestTargetKind::Group,
            target_id: fixtures.group_id.clone(),
            reason: format!(
                "e2e refresh group {}",
                chrono::Utc::now().timestamp_millis()
            ),
        },
        &requestor_ctx,
    )
    .await
    .map_err(|e| anyhow::anyhow!("refresh group request failed: {e}"))?;
    fixtures.pending_group_request_id = group_row.id;

    Ok(fixtures)
}
