// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use LinkerFlavor;
use std::default::Default;
use target::{LinkArgs, TargetOptions};

pub fn opts() -> TargetOptions {
    let mut args = LinkArgs::new();

    args.insert(LinkerFlavor::Gcc, vec![
        // We want to be able to strip as much executable code as possible
        // from the linker command line, and this flag indicates to the
        // linker that it can avoid linking in dynamic libraries that don't
        // actually satisfy any symbols up to that point (as with many other
        // resolutions the linker does). This option only applies to all
        // following libraries so we're sure to pass it as one of the first
        // arguments.
        "-Wl,--as-needed".to_string(),
    ]);

    TargetOptions {
        dynamic_linking: false,
        executables: true,
        linker_is_gnu: true,
        has_rpath: false,
        pre_link_args: args,
        position_independent_executables: true,
        .. Default::default()
    }
}