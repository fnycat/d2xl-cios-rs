use crate::types::*;

unsafe extern "C" {
    #[link_name = "DCInvalidateRange"]
    pub unsafe fn dc_invalidate_range(ptr: *mut core::ffi::c_void, size: core::ffi::c_int);

    #[link_name = "DCFlushRange"]
    pub unsafe fn dc_flush_range(ptr: *mut core::ffi::c_void, size: core::ffi::c_int);

    #[link_name = "ICInvalidate"]
    pub unsafe fn ic_invalidate();

    #[link_name = "Swi_Mload"]
    pub unsafe fn swi_mload(arg0: cu32, arg1: cu32, arg2: cu32, arg3: cu32) -> cs32;

    #[link_name = "Perms_Read"]
    pub unsafe fn perms_read() -> cu32;

    #[link_name = "Perms_Write"]
    pub unsafe fn perms_write(flags: cu32);

    #[link_name = "MEM2_Prot"]
    pub unsafe fn mem2_prot(flag: cu32);

    #[link_name = "VirtToPhys"]
    pub unsafe fn virt_to_phys(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;

    #[link_name = "PhysToVirt"]
    pub unsafe fn phys_to_virt(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}
