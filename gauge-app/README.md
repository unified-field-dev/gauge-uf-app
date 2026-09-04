# gauge-app

Orbital admin UI for permissions and permission groups (`PermissionRoutes` at
`/permission`).

## Host integration

Mount `<PermissionRoutes />` under the host `<Routes>`. Domain rules and
`actor_can` live in the `gauge` crate; this package wraps them for operators.

## Documentation

- Crate rustdoc: `cargo doc -p gauge-app --features ssr --open`
- Root [`README.md`](../README.md)
- Domain crate: [gauge](https://github.com/unified-field-dev/gauge)
