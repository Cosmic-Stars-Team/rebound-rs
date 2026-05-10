#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
mod bindings_gen {
    include!(concat!(env!("OUT_DIR"), "/bindings_gen.rs"));
}

pub use bindings_gen::*;
