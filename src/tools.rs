pub unsafe fn dc_write8(addr: *mut core::ffi::c_void, value: u8) {
    unsafe {
        core::ptr::write_volatile(addr.cast::<u8>(), value);
        dc_flush_range(addr, size_of::<u8>() as i32);
    }
}

pub unsafe fn dc_write16(addr: *mut core::ffi::c_void, value: u16) {
    unsafe {
        core::ptr::write_volatile(addr.cast::<u16>(), value);
        dc_flush_range(addr, size_of::<u16>() as i32);
    }
}

pub unsafe fn dc_write32(addr: *mut core::ffi::c_void, value: u32) {
    unsafe {
        core::ptr::write_volatile(addr.cast::<u32>(), value);
        dc_flush_range(addr, size_of::<u32>() as i32);
    }
}

unsafe extern "C" {
    #[link_name = "DCInvalidateRange"]
    pub unsafe fn dc_invalidate_range(ptr: *mut core::ffi::c_void, size: core::ffi::c_int);

    #[link_name = "DCFlushRange"]
    pub unsafe fn dc_flush_range(ptr: *mut core::ffi::c_void, size: core::ffi::c_int);

    #[link_name = "ICInvalidate"]
    pub unsafe fn ic_invalidate();

    #[link_name = "Swi_Mload"]
    pub unsafe fn swi_mload(arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> i32;

    #[link_name = "Perms_Read"]
    pub unsafe fn perms_read() -> u32;

    #[link_name = "Perms_Write"]
    pub unsafe fn perms_write(flags: u32);

    #[link_name = "MEM2_Prot"]
    pub unsafe fn mem2_prot(flag: u32);

    #[link_name = "VirtToPhys"]
    pub unsafe fn virt_to_phys(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;

    #[link_name = "PhysToVirt"]
    pub unsafe fn phys_to_virt(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}
