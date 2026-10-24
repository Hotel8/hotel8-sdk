# hotel8-sdk - Hotel8 Namespace Facade
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](../LICENSE)
[![Built with Flutter](https://img.shields.io/badge/Built%20with-Flutter-blue.svg)](https://flutter.dev/)
[![Docs](https://img.shields.io/badge/docs/read-3aa757.svg)](https://hotel8.dev/docs/services/sdk)
[![Website](https://img.shields.io/badge/website-hotel8.dev-1a73e8.svg)](https://hotel8.dev/docs/sdk)

## 1. Summary

The Hotel8 vertical's SDK workspace: one cargo workspace holding the
six hotel services' client + contracts crates and the `hotel8`
facade crate. Imports go through the facade -
`use hotel8::pms::client::…` (namespace rule, architecture-principles
§2.1).

## 2. Crates

| Crate | Purpose |
|---|---|
| `facade` (package `hotel8`) | The namespace facade |
| `crates/{pms,pos,distrib,loyalty,meta,upsell}/contracts` | Wire types per service |
| `crates/{pms,pos,distrib,loyalty,meta,upsell}/client` | HTTP clients per service (M2M-authenticated) |

## 3. Consumption

```toml
hotel8 = { git = "https://github.com/Hotel8/hotel8-sdk", tag = "hotel8-v0.1.0" }
```

The facade pins the platform SDK (`sentin8` - shared plane only,
SDD §2.2); consumers inherit it transitively.

---

**Architecture & Design:** hotel8:sdk and its underlying concepts were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).
