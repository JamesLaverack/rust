// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tests that the tail expr in null() has its type
// unified with the type *T, and so the type variable
// in that type gets resolved.

use std::mem;

fn null<T>() -> *T {
    unsafe {
        mem::transmute(0)
    }
}

pub fn main() { null::<int>(); }
