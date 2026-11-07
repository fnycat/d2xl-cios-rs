#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod ipc;
mod syscalls;
mod types;

pub use ipc::*;
pub use syscalls::*;
pub use types::*;

pub type PatcherFunc = unsafe extern "C" fn() -> i32;

#[repr(C)]
pub struct Patcher {
    pub function: PatcherFunc,
    pub status: i32,
}

/// Hack to make the `moduleName` string work.
#[repr(transparent)]
pub struct ForceSyncPtr<T>(pub *const T);

/// SAFETY: Safety is up to the user of this struct (although there's no good way to provide that in Rust).
unsafe impl<T> Sync for ForceSyncPtr<T> {}

/// The name of the cIOS module. This is referenced by an extern variable within `cios-lib`.
///
/// SAFETY: This static should never be written to.
#[unsafe(no_mangle)]
pub static moduleName: ForceSyncPtr<core::ffi::c_char> = ForceSyncPtr(c"DS4COMP".as_ptr());

unsafe extern "C" {
    #[link_name = "IOS_InitSystem"]
    pub unsafe fn ios_init_system(patchers: *const Patcher, size: cu32) -> cs32;
    #[link_name = "IOS_CheckPatches"]
    pub unsafe fn ios_check_patches(patchers: *const Patcher, size: cu32) -> cs32;
    #[link_name = "IOS_PatchModules"]
    pub unsafe fn ios_patch_modules(patchers: *const Patcher, size: cu32) -> cs32;
}
