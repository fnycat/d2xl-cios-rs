unsafe extern "C" {
    pub unsafe fn svc_printf(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
