#![cfg_attr(feature = "nightly",
    feature(doc_cfg, external_doc, trivial_bounds)
)]
#![cfg_attr(not(feature = "std"),
    no_std,
)]

#![allow(nonstandard_style)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
)]
#![deny(
    bare_trait_objects,
    elided_lifetimes_in_paths,
    unconditional_recursion,
    unused_must_use,
)]

#[cfg(feature = "proc_macros")]
#[macro_use]
extern crate require_unsafe_in_body;

#[doc(hidden)] pub
extern crate paste;

#[macro_use]
#[path = "utils/_mod.rs"]
pub(in crate)
mod utils;

extern crate proc_macro;
pub use ::proc_macro::{ffi_export, cfg_headers};

#[macro_use]
#[path = "layout/_mod.rs"]
pub mod layout;

__cfg_headers__! {
    #[doc(hidden)] pub
    use ::inventory;

    // #[doc(hidden)] /** Not yet part of the public API **/
    #[cfg_attr(feature = "nightly",
        doc(cfg(feature = "headers")),
    )]
    pub
    mod headers;

    #[doc(hidden)] pub
    struct FfiExport(
        pub
        fn (&'_ mut dyn layout::Definer)
          -> ::std::io::Result<()>
        ,
    );

    ::inventory::collect!(FfiExport);
}

cfg_alloc! {
    extern crate alloc;
}

cfg_alloc! {
    #[doc(inline)]
    pub use boxed::{Box, slice_boxed, str_boxed};
    pub mod boxed;
}

use self::c_char_module::c_char;
#[path = "c_char.rs"]
mod c_char_module;

pub mod char_p;

pub mod closure;

const _: () = {
    #[path = "ffi_export.rs"]
    mod ffi_export;
};

pub
mod ptr;

#[doc(inline)]
pub use slice::{slice_ref, slice_mut};
pub mod slice;


#[doc(inline)]
pub use string::str_ref;

#[path = "string/_mod.rs"]
pub mod string;

pub
mod tuple;

cfg_alloc! {

    #[doc(inline)]
    pub use string::String;

    #[doc(inline)]
    pub use vec::Vec;
    pub mod vec;
}

macro_rules! reexport_primitive_types {(
    $($ty:ident)*
) => (
    $(
        #[doc(hidden)]
        pub use $ty;
    )*
)} reexport_primitive_types! {
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
    char
    bool
    str
}
#[doc(hidden)] pub use ::core;
cfg_std! {
    #[doc(hidden)] pub use ::std;
}

#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
#[doc(hidden)] pub
struct NotZeroSized;


pub
mod prelude {
    pub
    mod repr_c {
        cfg_alloc! {
            #[doc(no_inline)]
            pub use crate::{
                Box,
                Vec,
                String,
            };
        }
        #[doc(no_inline)]
        pub use crate::tuple::*;
    }
    #[doc(no_inline)]
    pub use crate::{
        char_p::{
            char_p_raw,
            char_p_ref,
        },
        closure::*,
        ffi_export,
        layout::ReprC,
        slice::{
            slice_raw,
            slice_ref,
            slice_mut,
        },
        string::{
            str_ref,
        },
    };
    cfg_alloc! {
        #[doc(no_inline)]
        pub use crate::{
            char_p::char_p_boxed,
            slice::slice_boxed,
            string::str_boxed,
        };
    }
    cfg_proc_macros! {
        #[doc(no_inline)]
        pub use crate::layout::derive_ReprC;
    }
    #[doc(no_inline)]
    pub use ::core::{
        convert::{
            TryFrom as _,
            TryInto as _,
        },
        ops::Not as _,
    };
}
