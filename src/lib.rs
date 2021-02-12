// sd-sys: FFI bindings to systemd for sd-id128 & sd-journal
// Copyright (C) 2020 Christian Klaue [mail@ck76.de]
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! [![GitHub release (latest by date)](https://img.shields.io/github/v/release/ente76/sd-sys?label=github&logo=github)](https://github.com/ente76/sd-sys)  [![Crates.io](https://img.shields.io/crates/v/sd-sys)](https://crates.io/crates/sd-sys)  [![docs.rs](https://docs.rs/sd-sys/badge.svg)](https://docs.rs/sd-sys/)  ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ente76/sd-sys/test?label=test&logo=github) [![buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-or%20I%20sing-53a0d0?style=flat&logo=Buy-Me-A-Coffee)](https://www.buymeacoffee.com/ente)  [![donate@paypal](https://img.shields.io/badge/paypal-donation-53a0d0?style=flat&logo=paypal)](https://www.paypal.com/donate?hosted_button_id=CRGNTJBS4AD4G)  
//!
//! [sd-sys](https://github.com/ente76/sd-sys) defines the FFI bindings for sd-id128 & sd-journal in the systemd API of [libsystemd](https://www.freedesktop.org/software/systemd/man/sd-id128.html).  sd-sys is part of the [systemd.rs](https://github.com/ente76/systemd.rs) project, providing the FFI bindings for [sd-id128](https://github.com/ente76/sd-id128) and [sd-journal](https://github.com/ente76/sd-journal).
//!
//! ## Compatibility
//!
//! This library is developed against the latest version of libsystemd. As such there may be FFI bindings included which are not available in previous versions. Issues may arise during linking, if an unavailable function is used. There is no version check included in this library. For an example of such version check, see the feature definition of [sd-id128](https://github.com/ente76/sd-id128).

/// FFI binding for sd-id128 as defined in libsystemd used in
/// [sd-id128](https://gitlab.com/systemd.rs/sd-id128)
///
/// This module should rarely be used directly. Crate sd-id128 defines a wrapper
/// to the FFI bindings which translates each extern function 1:1 into native
/// rust.
pub mod id128;

/// FFI mapping for sd-journal as defined in libsystemd used in
/// [sd-journal](https://gitlab.com/systemd.rs/sd-journal)
///
/// This module should rarely be used directly. Crate sd-journal defines a
/// wrapper to the FFI bindings which translates each extern function into
/// native rust.
pub mod journal;
