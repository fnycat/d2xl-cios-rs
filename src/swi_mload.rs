use crate::IosInfo;

pub type SwiFunc =
    unsafe extern "C" fn(in_ptr: *mut core::ffi::c_void, out_ptr: *mut core::ffi::c_void) -> i32;

unsafe extern "C" {
    #[link_name = "Swi_Memcpy"]
    pub unsafe fn swi_memcpy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, len: i32);

    #[link_name = "Swi_uMemcpy"]
    pub unsafe fn swi_umemcpy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, len: i32);

    #[link_name = "Swi_CallFunc"]
    pub unsafe fn swi_callfunc(
        func: SwiFunc,
        in_ptr: *mut core::ffi::c_void,
        out_ptr: *mut core::ffi::c_void,
    ) -> i32;

    #[link_name = "Swi_GetIosInfo"]
    pub unsafe fn swi_get_ios_info(buf: *mut IosInfo) -> u32;
}
