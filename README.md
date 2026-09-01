# Gauge UF App

[GitHub](https://github.com/unified-field-dev/gauge-uf-app) ·
`cargo doc -p gauge-app --features ssr --open`

## About

Gauge UF App is the Unified Field **operations UI** for permissions, groups,
domains, and request/review under `/permission`. Domain rules, Valence schemas,
and `actor_can` / `user_can` live in the sibling
[gauge](https://github.com/unified-field-dev/gauge) crate; this repo mounts the
Orbital pages and Higgs `#[server]` wrappers operators use.

- **UI (`gauge-app`)** — pages, Higgs wrappers, `PermissionRoutes`, `uf_app!`
  registration at app id `permission` / path `/permission`

Hosts supply Valence + auth, enable `ssr` / hydrate to match the host, and mount
`PermissionRoutes`. Crate-root rustdoc owns the **Features** index and mount
guide.

## Where things live

| Concern | Location |
|---------|----------|
| Mount routes, Features guide | `gauge-app` crate root (`PermissionRoutes`) |
| Higgs server wrappers / DTOs | `gauge-app/src/server.rs` |
| Orbital pages | `gauge-app/src/pages/` |
| Shell / app bar | `gauge-app/src/shell/`, `layout.rs` |
| Domain service, schemas, `actor_can` | sibling [gauge](https://github.com/unified-field-dev/gauge) (`gauge::service`) |
| Playwright host + scenario catalog | `gauge-uf-app-e2e/` |
| Local + CI gates | [`docs/VERIFICATION.md`](docs/VERIFICATION.md) |

There is no `service/` tree in this repo — call into `gauge` for ACL and persistence.

## Getting started

```toml
[dependencies]
# Pin a release tag or commit SHA — do not use branch = "main".
gauge-app = { git = "https://github.com/unified-field-dev/gauge-uf-app", package = "gauge-app", rev = "<tag-or-sha>", default-features = false }
gauge = { git = "https://github.com/unified-field-dev/gauge", package = "gauge", rev = "<tag-or-sha>", default-features = false }
```

```rust,ignore
use gauge_app::PermissionRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <PermissionRoutes />
    </Routes>
}
```

Wire Valence + session extractors in host bootstrap, sync permission manifests,
then mount the routes above. Domain-only smoke (bootstrap owner +
deny→grant→allow without the UI graph) lives in gauge's
[`embedded-gauge-host`](https://github.com/unified-field-dev/gauge/tree/main/examples/embedded-gauge-host).

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-gauge-uf-app
cargo check -p gauge-app --features ssr
```

## Security

Admin mutations require a session and `GaugeAdmin` (Higgs
`#[uf_product_macros::server(permission = "GaugeAdmin")]`). Owner / Super User
checks and Valence policy notes for the domain live in
[gauge `SECURITY.md`](https://github.com/unified-field-dev/gauge/blob/main/SECURITY.md).
Report vulnerabilities privately — do not open a public issue for
security-sensitive reports.

## Verify

Full Layer 1 (fmt / clippy / test / check / rustdoc) and Layer 2 Playwright live in
[`docs/VERIFICATION.md`](docs/VERIFICATION.md). GitHub Actions runs the same gates on
every PR and push to `main`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-gauge-uf-app
cargo check -p gauge-app --features ssr
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p gauge-app --features ssr --no-deps
```

Domain contracts (service integ, embedded teaching host) stay in the
[gauge](https://github.com/unified-field-dev/gauge) repo.

## FAQ

**Is this a standalone server?** No. `gauge-app` mounts under a host `<Routes>`
tree. Persistence and grant resolution live in `gauge`; hosts supply Valence and
session chrome.

**Do I need this crate for backend checks?** No. Call `gauge::service::actor_can`
(or related APIs) from the domain crate alone. Depend on `gauge-app` when
operators need `/permission`.

## License

MIT (see workspace `Cargo.toml` `license` field).
