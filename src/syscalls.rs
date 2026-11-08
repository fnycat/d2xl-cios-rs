use crate::Ioctlv;

pub type ThreadFn = unsafe extern "C" fn(arg: *mut core::ffi::c_void) -> u32;

// The types used in these functions are incredibly inconsistent, but I'm just following the
// `syscalls.h` header.
unsafe extern "C" {
    pub unsafe fn os_thread_create(
        entry: ThreadFn,
        arg: *mut core::ffi::c_void,
        stack: *mut core::ffi::c_void,
        stack_size: u32,
        priority: u32,
        autostart: u32,
    ) -> i32;

    pub unsafe fn os_thread_set_prioty(id: i32, priority: i32);
    pub unsafe fn os_thread_get_priority(id: i32) -> i32;
    pub unsafe fn os_get_thread_id() -> i32;
    pub unsafe fn os_get_parent_thread_id() -> i32;
    pub unsafe fn os_thread_continue(id: i32) -> i32;
    pub unsafe fn os_thread_stop(id: i32) -> i32;

    pub unsafe fn os_message_queue_create(ptr: *mut core::ffi::c_void, id: u32) -> i32;
    pub unsafe fn os_message_queue_destroy(queue_id: i32) -> i32;
    pub unsafe fn os_message_queue_receive(queue_id: i32, message: *mut u32, flags: u32)
    -> i32;
    pub unsafe fn os_message_queue_send(queue_id: i32, message: u32, flags: i32) -> i32;
    pub unsafe fn os_message_queue_now(queue_id: i32, message: u32, flags: i32) -> i32;
    pub unsafe fn os_message_queue_ack(message: *mut core::ffi::c_void, result: i32) -> i32;

    pub unsafe fn os_heap_create(ptr: *mut core::ffi::c_void, size: i32) -> i32;
    pub unsafe fn os_heap_destroy(heap: i32) -> i32;
    pub unsafe fn os_heap_alloc(heap: i32, size: u32) -> *mut core::ffi::c_void;
    pub unsafe fn os_heap_alloc_aligned(
        heap: i32,
        size: i32,
        align: i32,
    ) -> *mut core::ffi::c_void;
    pub unsafe fn os_heap_free(heap: i32, ptr: *mut core::ffi::c_void);

    pub unsafe fn __os_sync_before_read(ptr: *mut core::ffi::c_void, size: i32);
    pub unsafe fn __os_sync_after_write(ptr: *mut core::ffi::c_void, size: i32);

    pub unsafe fn os_set_dvd_video_mode(enable: u32);
    pub unsafe fn os_set_ahbprot(enable: u32);

    pub unsafe fn os_open(device: *const core::ffi::c_char, mode: i32) -> i32;
    pub unsafe fn os_close(fd: i32) -> i32;
    pub unsafe fn os_read(fd: i32, d: *mut core::ffi::c_void, len: i32) -> i32;
    pub unsafe fn os_write(fd: i32, s: *mut core::ffi::c_void, len: i32) -> i32;
    pub unsafe fn os_seek(fd: i32, offset: i32, mode: i32) -> i32;
    pub unsafe fn os_ioctl(
        fd: i32,
        request: i32,
        in_ptr: *mut core::ffi::c_void,
        bytes_in: i32,
        out_ptr: *mut core::ffi::c_void,
        bytes_out: i32,
    ) -> i32;
    pub unsafe fn os_ioctlv(
        fd: i32,
        request: i32,
        bytes_in: i32,
        bytes_out: i32,
        vector: *mut Ioctlv,
    ) -> i32;

    pub unsafe fn os_open_async(
        device: *const core::ffi::c_char,
        mode: i32,
        ipc_cb: *mut core::ffi::c_void,
        usrdata: *mut core::ffi::c_void,
    ) -> i32;
    pub unsafe fn os_close_async(
        fd: i32,
        ipc_cb: *mut core::ffi::c_void,
        usrdata: *mut core::ffi::c_void,
    ) -> i32;
    pub unsafe fn os_ioctl_async(
        fd: i32,
        request: i32,
        in_ptr: *mut core::ffi::c_void,
        bytes_in: i32,
        out_ptr: *mut core::ffi::c_void,
        bytes_out: i32,
        ...
    ) -> i32;
    pub unsafe fn os_ioctlv_async(
        fd: i32,
        request: i32,
        bytes_in: i32,
        bytes_out: i32,
        vector: *mut Ioctlv,
        ...
    ) -> i32;

    pub unsafe fn os_create_timer(
        time_us: i32,
        repeat_time_us: i32,
        message_queue: i32,
        message: i32,
    ) -> i32;
    pub unsafe fn os_destroy_timer(timer_id: i32) -> i32;
    pub unsafe fn os_stop_timer(timer_id: i32) -> i32;
    pub unsafe fn os_restart_timer(timer_id: i32, time_us: i32, repeat_time_us: i32) -> i32;
    pub unsafe fn os_timer_now(timer_id: i32) -> i32;

    pub unsafe fn os_device_register(
        device_name: *const core::ffi::c_char,
        queue_handle: i32,
    ) -> i32;
    pub unsafe fn os_register_event_handler(device: i32, queue: i32, message: i32) -> i32;
    pub unsafe fn os_unregister_event_handler(device: i32) -> i32;

    #[link_name = "os_software_IRQ"]
    pub unsafe fn os_software_irq(dev: i32) -> i32;

    pub unsafe fn os_ios_boot(path: *const core::ffi::c_char, flag: u32, version: u32) -> i32;
    pub unsafe fn os_kernel_set_version(version: u32) -> i32;
    pub unsafe fn os_kernel_get_version() -> i32;
    pub unsafe fn os_get_key(keyid: i32, out: *mut core::ffi::c_void);
    pub unsafe fn os_virt_to_phys(ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;

    // ARM syscalls

    pub unsafe fn svc_write(str: *const core::ffi::c_char);
}
