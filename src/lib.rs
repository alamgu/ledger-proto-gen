#![feature(cfg_version)]
#![cfg_attr(version("1.71"), feature(impl_trait_in_assoc_type))]
mod proto {
    include!(concat!(env!("OUT_DIR"), "/google/mod.rs"));
}
pub mod generate;
