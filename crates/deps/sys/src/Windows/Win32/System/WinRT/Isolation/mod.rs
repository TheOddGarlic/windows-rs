#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IIsolatedEnvironmentInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsolatedEnvironmentInterop {}
impl ::core::clone::Clone for IIsolatedEnvironmentInterop {
    fn clone(&self) -> Self {
        *self
    }
}