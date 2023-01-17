#![allow(non_snake_case, non_camel_case_types)]
#![cfg_attr(test, allow(deref_nullptr))] // https://github.com/rust-lang/rust-bindgen/issues/2066

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm32_unknown_unknown;

// force linking to openssl
#[cfg(feature = "bundled-sqlcipher-vendored-openssl")]
extern crate openssl_sys;

#[cfg(all(windows, feature = "winsqlite3", target_pointer_width = "32"))]
compile_error!("The `libsqlite3-sys/winsqlite3` feature is not supported on 32 bit targets.");

pub use self::error::*;

use std::default::Default;
use std::mem;

mod error;

#[must_use]
pub fn SQLITE_STATIC() -> sqlite3_destructor_type {
    None
}

#[must_use]
pub fn SQLITE_TRANSIENT() -> sqlite3_destructor_type {
    Some(unsafe { mem::transmute(-1_isize) })
}

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
}
pub use bindings::*;

pub type sqlite3_index_constraint = sqlite3_index_info_sqlite3_index_constraint;
pub type sqlite3_index_constraint_usage = sqlite3_index_info_sqlite3_index_constraint_usage;

impl Default for sqlite3_vtab {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Default for sqlite3_vtab_cursor {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub struct Rand {
    pub seed: u128,
    pub a: u128,
    pub c: u128,
    pub m: u128,
}

impl Rand {
    pub fn new() -> Rand{
        let now = if cfg!(all(target_arch = "wasm32", target_os = "unknown")) {
            (ic_cdk::api::time() / 1000000000 ) as u128
        } else {
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u128
        };
        Rand{seed: now, a: 0xBC8F, c: 0xB, m: (1 << 31) - 1}
    }

    pub fn rand(&mut self) {
        self.seed = (self.seed * self.a + self.c) & self.m;
    }

    pub fn next(&mut self, bound: usize) -> usize {
        self.rand();
        (self.seed % (bound as u128)) as usize
    }

    pub fn fill_i8(&mut self, dest: &mut [i8]) {
        let data = (0..dest.len()).map(|_| {
            self.rand();
            (self.seed % 256) as i8
        }).collect::<Vec<_>>();
        dest.copy_from_slice(&data);
    }

    pub fn fill_u8(&mut self, dest: &mut [u8]) {
        let data = (0..dest.len()).map(|_| {
            self.rand();
            (self.seed % 256) as u8
        }).collect::<Vec<_>>();
        dest.copy_from_slice(&data);
    }
}
