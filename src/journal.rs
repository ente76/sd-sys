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
use super::id128::sd_id128;
use libc::{c_char, c_int, c_void, iovec, size_t};

/// FFI data type mapping for sd-journal as defined in libsystemd
///
/// This data type should rarely be used directly. Crate sd-journal defines a
/// high level wrapper `Journal` which should be your first choice. This wrapper
/// also offers low level access to libsystemd in a Rust-native way.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct sd_journal {
    _unused: [u8; 0]
}

#[allow(clippy::identity_op)]
pub const SD_JOURNAL_LOCAL_ONLY: c_int = 1 << 0;
pub const SD_JOURNAL_RUNTIME_ONLY: c_int = 1 << 1;
pub const SD_JOURNAL_SYSTEM: c_int = 1 << 2;
pub const SD_JOURNAL_CURRENT_USER: c_int = 1 << 3;
pub const SD_JOURNAL_OS_ROOT: c_int = 1 << 4;
pub const SD_JOURNAL_ALL_NAMESPACES: c_int = 1 << 5;
pub const SD_JOURNAL_INCLUDE_DEFAULT_NAMESPACE: c_int = 1 << 6;

pub const LOG_EMERG: c_int = 0;
pub const LOG_ALERT: c_int = 1;
pub const LOG_CRIT: c_int = 2;
pub const LOG_ERR: c_int = 3;
pub const LOG_WARNING: c_int = 4;
pub const LOG_NOTICE: c_int = 5;
pub const LOG_INFO: c_int = 6;
pub const LOG_DEBUG: c_int = 7;

pub const SD_JOURNAL_NOP: c_int = 0;
pub const SD_JOURNAL_APPEND: c_int = 1;
pub const SD_JOURNAL_INVALIDATE: c_int = 2;

extern "C" {
    /// `int sd_journal_print(int priority, const char *format, …);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_print.html#>
    pub fn sd_journal_print(priority: c_int, message: *const c_char, ...) -> c_int;
    /// `int sd_journal_sendv(const struct iovec *iov, int n);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_print.html#>
    pub fn sd_journal_sendv(entry: *const iovec, count: c_int) -> c_int;
    // not implemented:
    // int sd_journal_printv(int priority, const char *format, va_list ap);
    // int sd_journal_send(const char *format, …);
    // int sd_journal_perror(const char *message);
    // int sd_journal_print_with_location(const char *file, const char *line,
    //                 const char *func, int priority, const char *format, …);
    // int sd_journal_printv_with_location(int priority, const char *file, const
    //          char *line, const char *func, const char *format, va_list ap);
    // int sd_journal_send_with_location(const char *file, const char *line, const
    //                                  char *func, const char *format, …);
    // int sd_journal_sendv_with_location(const char *file, const char *line,
    //                     const char *func, const struct iovec *iov, int n);
    // int sd_journal_perror_with_location(const char *file, const char *line, const
    //                                  char *func, const char *message);

    // <https://www.freedesktop.org/software/systemd/man/sd_journal_stream_fd.html#>
    // not implemented:
    // int sd_journal_stream_fd(const char *identifier, int priority, int
    //                                  level_prefix);

    /// `int sd_journal_open(sd_journal **ret, int flags);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#>
    pub fn sd_journal_open(journal: *mut *mut sd_journal, flags: c_int) -> c_int;
    /// `int sd_journal_open_namespace(sd_journal **ret, const char *namespace,
    ///                                 int flags);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#>
    pub fn sd_journal_open_namespace(journal: *mut *mut sd_journal,
                                     namespace: *const c_char,
                                     flags: c_int)
                                     -> c_int;
    /// `int sd_journal_open_directory(sd_journal **ret, const char *path,
    ///                                 int flags);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#>
    pub fn sd_journal_open_directory(journal: *mut *mut sd_journal,
                                     path: *const c_char,
                                     flags: c_int)
                                     -> c_int;
    /// `int sd_journal_open_files(sd_journal **ret, const char **paths,
    ///                                 int flags);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#>
    pub fn sd_journal_open_files(journal: *mut *mut sd_journal,
                                 paths: *const *const c_char,
                                 flags: c_int)
                                 -> c_int;
    /// `void sd_journal_close(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_open.html#>
    pub fn sd_journal_close(journal: *mut sd_journal);
    // not implemented:
    // int sd_journal_open_directory_fd(sd_journal **ret, int fd, int flags);
    // int sd_journal_open_files_fd(sd_journal **ret, int fds[], unsigned n_fds,
    //                                  int flags);
    /// `int sd_journal_next(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#>
    pub fn sd_journal_next(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_previous(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#>
    pub fn sd_journal_previous(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_next_skip(sd_journal *j, uint64_t skip);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#>
    pub fn sd_journal_next_skip(journal: *mut sd_journal, skip: u64) -> c_int;
    /// `int sd_journal_previous_skip(sd_journal *j, uint64_t skip);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_next.html#>
    pub fn sd_journal_previous_skip(journal: *mut sd_journal, skip: u64) -> c_int;
    /// `int sd_journal_get_realtime_usec(sd_journal *j, uint64_t *usec);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_realtime_usec.html#>
    pub fn sd_journal_get_realtime_usec(journal: *mut sd_journal, usec: *mut u64) -> c_int;
    /// `int sd_journal_get_monotonic_usec(sd_journal *j, uint64_t *usec,
    ///                                 sd_id128_t *boot_id);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_realtime_usec.html#>
    pub fn sd_journal_get_monotonic_usec(journal: *mut sd_journal,
                                         usec: *mut u64,
                                         boot_id: *mut sd_id128)
                                         -> c_int;
    /// `int sd_journal_add_match(sd_journal *j, const void *data,
    ///                                 size_t size);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#>
    pub fn sd_journal_add_match(journal: *mut sd_journal,
                                data: *const c_void,
                                len: size_t)
                                -> c_int;
    /// `int sd_journal_add_disjunction(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#>
    pub fn sd_journal_add_disjunction(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_add_conjunction(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#>
    pub fn sd_journal_add_conjunction(journal: *mut sd_journal) -> c_int;
    /// `void sd_journal_flush_matches(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_add_match.html#>
    pub fn sd_journal_flush_matches(journal: *mut sd_journal);
    /// `int sd_journal_seek_head(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#>
    pub fn sd_journal_seek_head(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_seek_tail(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#>
    pub fn sd_journal_seek_tail(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_seek_monotonic_usec(sd_journal *j, sd_id128_t boot_id,
    ///                                 uint64_t usec);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#>
    pub fn sd_journal_seek_monotonic_usec(journal: *mut sd_journal,
                                          boot_id: sd_id128,
                                          usec: u64)
                                          -> c_int;
    /// `int sd_journal_seek_realtime_usec(sd_journal *j, uint64_t usec);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#>
    pub fn sd_journal_seek_realtime_usec(journal: *mut sd_journal, usec: u64) -> c_int;
    /// `int sd_journal_seek_cursor(sd_journal *j, const char *cursor);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_seek_head.html#>
    pub fn sd_journal_seek_cursor(journal: *mut sd_journal, cursor: *const c_char) -> c_int;
    /// `int sd_journal_enumerate_fields(sd_journal *j, const char **field);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_enumerate_fields.html#>
    pub fn sd_journal_enumerate_fields(journal: *mut sd_journal,
                                       fields: *mut *const c_char)
                                       -> c_int;
    /// `void sd_journal_restart_fields(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_enumerate_fields.html#>
    pub fn sd_journal_restart_fields(journal: *mut sd_journal);
    /// `int sd_journal_get_cursor(sd_journal *j, char **cursor);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_cursor.html#>
    pub fn sd_journal_get_cursor(journal: *mut sd_journal, cursor: *mut *mut c_char) -> c_int;
    /// `int sd_journal_test_cursor(sd_journal *j, const char *cursor);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_cursor.html#>
    pub fn sd_journal_test_cursor(journal: *mut sd_journal, cursor: *const c_char) -> c_int;
    /// `int sd_journal_get_cutoff_realtime_usec(sd_journal *j, uint64_t *from,
    ///                                 uint64_t *to);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_cutoff_realtime_usec.html#>
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_cutoff_monotonic_usec.html#>
    pub fn sd_journal_get_cutoff_realtime_usec(journal: *mut sd_journal,
                                               from: *mut u64,
                                               to: *mut u64)
                                               -> c_int;
    /// `int sd_journal_get_cutoff_monotonic_usec(sd_journal *j,
    ///                     sd_id128_t boot_id, uint64_t *from, uint64_t *to);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_cutoff_realtime_usec.html#>
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_cutoff_monotonic_usec.html#>
    pub fn sd_journal_get_cutoff_monotonic_usec(journal: *mut sd_journal,
                                                boot_id: sd_id128,
                                                from: *mut u64,
                                                to: *mut u64)
                                                -> c_int;
    /// int sd_journal_get_usage(sd_journal *j, uint64_t *bytes);
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_usage.html#>
    pub fn sd_journal_get_usage(journal: *mut sd_journal, size: *mut u64) -> c_int;
    /// `int sd_journal_get_catalog(sd_journal *j, char **ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_catalog.html#>
    pub fn sd_journal_get_catalog(journal: *mut sd_journal, catalog: *mut *mut c_char) -> c_int;
    /// `int sd_journal_get_catalog_for_message_id(sd_id128_t id, char **ret);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_catalog.html#>
    pub fn sd_journal_get_catalog_for_message_id(id: sd_id128, catalog: *mut *mut c_char) -> c_int;
    /// `int sd_journal_get_fd(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#>
    pub fn sd_journal_get_fd(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_get_events(sd_journal *j);``
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#>
    pub fn sd_journal_get_events(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_get_timeout(sd_journal *j, uint64_t *timeout_usec);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#>
    pub fn sd_journal_get_timeout(journal: *mut sd_journal, timeout: *mut u64) -> c_int;
    /// `int sd_journal_process(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#>
    pub fn sd_journal_process(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_wait(sd_journal *j, uint64_t timeout_usec);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#>
    pub fn sd_journal_wait(journal: *mut sd_journal, timeout: u64) -> c_int;
    /// `int sd_journal_reliable_fd(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_fd.html#>
    pub fn sd_journal_reliable_fd(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_has_runtime_files(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_has_runtime_files.html#>
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_has_persistent_files.html#>
    pub fn sd_journal_has_runtime_files(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_has_persistent_files(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_has_runtime_files.html#>
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_has_persistent_files.html#>
    pub fn sd_journal_has_persistent_files(journal: *mut sd_journal) -> c_int;
    /// `int sd_journal_get_data(sd_journal *j, const char *field,
    ///                                 const void **data, size_t *length);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#>
    pub fn sd_journal_get_data(journal: *mut sd_journal,
                               field: *const c_char,
                               data: *mut *const c_void,
                               length: *mut size_t)
                               -> c_int;
    /// `int sd_journal_enumerate_data(sd_journal *j, const void **data,
    ///                                 size_t *length);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#>
    pub fn sd_journal_enumerate_data(journal: *mut sd_journal,
                                     data: *mut *const c_void,
                                     length: *mut size_t)
                                     -> c_int;
    /// `int sd_journal_enumerate_available_data(sd_journal *j,
    ///                                 const void **data, size_t *length);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#>
    pub fn sd_journal_enumerate_available_data(journal: *mut sd_journal,
                                               data: *mut *const c_void,
                                               length: *mut size_t)
                                               -> c_int;
    /// `void sd_journal_restart_data(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#>
    pub fn sd_journal_restart_data(journal: *mut sd_journal);
    /// `int sd_journal_set_data_threshold(sd_journal *j, size_t sz);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#>
    pub fn sd_journal_set_data_threshold(journal: *mut sd_journal, size: size_t) -> c_int;
    /// `int sd_journal_get_data_threshold(sd_journal *j, size_t *sz);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html#>
    pub fn sd_journal_get_data_threshold(journal: *mut sd_journal, size: *mut size_t) -> c_int;
    /// `int sd_journal_query_unique(sd_journal *j, const char *field);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#>
    pub fn sd_journal_query_unique(journal: *mut sd_journal, field: *const c_char) -> c_int;
    /// `int sd_journal_enumerate_available_unique(sd_journal *j,
    ///                                 const void **data, size_t *length);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#>
    pub fn sd_journal_enumerate_available_unique(journal: *mut sd_journal,
                                                 data: *mut *const c_void,
                                                 length: *mut size_t)
                                                 -> c_int;
    /// `int sd_journal_enumerate_unique(sd_journal *j, const void **data,
    ///                                 size_t *length);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#>
    pub fn sd_journal_enumerate_unique(journal: *mut sd_journal,
                                       data: *mut *const c_void,
                                       length: *mut size_t)
                                       -> c_int;
    /// `void sd_journal_restart_unique(sd_journal *j);`
    ///
    /// <https://www.freedesktop.org/software/systemd/man/sd_journal_query_unique.html#>
    pub fn sd_journal_restart_unique(journal: *mut sd_journal);
}
