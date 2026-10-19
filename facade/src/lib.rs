// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # hotel8 - Hotel8 workspace namespace facade
//!
//! Every hotel8 crate is named `hotel8::{leaf}` and is imported through this
//! facade; service-scoped crates nest under their service module. See
//! `sentin8/design/sentin8-architecture-principles.md` §2.1.

pub mod pms {
    pub use hotel8_pms_client as client;
    pub use hotel8_pms_contracts as contracts;
}
pub mod pos {
    pub use hotel8_pos_client as client;
    pub use hotel8_pos_contracts as contracts;
}
pub mod distrib {
    pub use hotel8_distrib_client as client;
    pub use hotel8_distrib_contracts as contracts;
}
pub mod loyalty {
    pub use hotel8_loyalty_client as client;
    pub use hotel8_loyalty_contracts as contracts;
}
pub mod meta {
    pub use hotel8_meta_client as client;
    pub use hotel8_meta_contracts as contracts;
}
pub mod upsell {
    pub use hotel8_upsell_client as client;
    pub use hotel8_upsell_contracts as contracts;
}
