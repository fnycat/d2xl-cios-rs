unsafe extern "C" {
    #[link_name = "Mem_Init"]
    pub unsafe fn mem_init(heapspace: *mut u32, heap_size: u32) -> i32;

    #[link_name = "Mem_Alloc"]
    pub unsafe fn mem_alloc(size: u32) -> *mut core::ffi::c_void;

    #[link_name = "Mem_Free"]
    pub unsafe fn mem_free(ptr: *mut core::ffi::c_void);
}
