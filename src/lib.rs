// sd-sys: FFI bindings to systemd for sd-id128 & sd-journal
// Copyright (C) 2020 Christian Klaue ente@ck76.de
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
//! # sd-sys
//!
//! FFI bindings for systemd API as offered by libsystemd.

/// FFI mapping for sd-id128 as defined in libsystemd
///
/// This module should rarely be used directly. Crate sd-id128 defines a low
/// level wrapper to the FFI bindings in module lli which translates each extern
/// function 1:1 into native rust. The same crate also offers a high level
/// wrapper which offers additional functionality.
pub mod id128;

/// FFI mapping for sd-journal as defined in libsystemd
///
/// This module should rarely be used directly. Crate sd-journal defines a low
/// level wrapper to the FFI bindings in module lli which translates each extern
/// function 1:1 into native rust. The same crate also offers a high level
/// wrapper which offers additional functionality.
pub mod journal;
