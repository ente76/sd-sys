# sd-sys

FFI bindings for systemd API as offered by libsystemd.

sd-sys is an alternative to libsystemd-sys from the [systemd-rust](https://github.com/jmesmon/rust-systemd) project. The main differences of sd-sys to libsystemd-sys are:

- sd-sys is published under the AGPL-3.0 license. Individual/commercial licenses are available upon request.
- sd-sys covers only sd-journal and sd-id128 of libsystemd. There are currently no plans to extend this coverage. Whenever there is any doubt if that is enough, [systemd-rust](https://github.com/jmesmon/rust-systemd) may be a better choice.

sd-sys serves [sd-id128](https://gitlab.com/systemd.rs/sd-id128) and [sd-journal](https://gitlab.com/systemd.rs/sd-journal) as the basic FFI binding.

Status of the library:

- Module id128 is fully stable. sd-id128 is fully implemented based on this module.
- Module journal is rather a first draft. No changes are expected anymore, but the module still has to prove against an implementation. If changes are required, they will be breaking.



## License

sd-sys: FFI bindings to systemd for sd-id128 & sd-journal
Copyright (C) 2020 Christian Klaue [mail@ck76.de]

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

Individual licenses are granted upon request.
