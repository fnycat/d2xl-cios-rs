use crate::types::*;

unsafe extern "C" {
    #[link_name = "Mem_Init"]
    pub unsafe fn mem_init(heapspace: *mut cu32, heap_size: cu32) -> cs32;

    #[link_name = "Mem_Alloc"]
    pub unsafe fn mem_alloc(size: cu32) -> *mut core::ffi::c_void;

    #[link_name = "Mem_Free"]
    pub unsafe fn mem_free(ptr: *mut core::ffi::c_void);
}
