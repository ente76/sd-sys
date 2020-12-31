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
use libc::{c_char, c_int};
/// FFI data type mapping for sd-id128 as defined in libsystemd.
///
/// This data type should rarely be used directly. Crate sd-id128 defines a
/// wrapper `ID128`.
///
/// While libsystemd defines the data type as a union of [u8;16] and [u64;2]
/// currently sd-sys only supports the first.
///
/// <https://www.freedesktop.org/software/systemd/man/sd-id128.html>
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct sd_id128 {
    pub value: [u8; 16]
}

extern "C" {
    /// `char *sd_id128_to_string(sd_id128_t id, char s[33]);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_id128_to_string.html#>
    pub fn sd_id128_to_string(id: sd_id128, string: *const c_char) -> *mut c_char;
    /// `int sd_id128_from_string(const char *s, sd_id128_t *ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_id128_to_string.html#>
    pub fn sd_id128_from_string(string: *const c_char, id: *mut sd_id128) -> c_int;
    /// `int sd_id128_randomize(sd_id128_t *ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_id128_randomize.html#>
    pub fn sd_id128_randomize(id: *mut sd_id128) -> c_int;
    /// `int sd_id128_get_machine(sd_id128_t *ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#>
    pub fn sd_id128_get_machine(id: *mut sd_id128) -> c_int;
    /// `int sd_id128_get_machine_app_specific(sd_id128_t app_id,
    ///                                 sd_id128_t *ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#>
    pub fn sd_id128_get_machine_app_specific(app: sd_id128, machine: *mut sd_id128) -> c_int;
    /// `int sd_id128_get_boot(sd_id128_t *ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#>
    pub fn sd_id128_get_boot(id: *mut sd_id128) -> c_int;
    /// `int sd_id128_get_boot_app_specific(sd_id128_t app_id,
    /// sd_id128_t *ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#>
    pub fn sd_id128_get_boot_app_specific(app: sd_id128, boot: *mut sd_id128) -> c_int;
    /// `int sd_id128_get_invocation(sd_id128_t *ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#>
    pub fn sd_id128_get_invocation(id: *mut sd_id128) -> c_int;
}
