use crate::{Ioctlv, types::*};

pub type ThreadFn = unsafe extern "C" fn(arg: *mut core::ffi::c_void) -> cu32;

// The types used in these functions are incredibly inconsistent, but I'm just following the
// `syscalls.h` header.
unsafe extern "C" {
    pub unsafe fn os_thread_create(
        entry: ThreadFn,
        arg: *mut core::ffi::c_void,
        stack: *mut core::ffi::c_void,
        stack_size: cu32,
        priority: cu32,
        autostart: cu32,
    ) -> cs32;

    pub unsafe fn os_thread_set_prioty(id: cs32, priority: cs32);
    pub unsafe fn os_thread_get_priority(id: cs32) -> cs32;
    pub unsafe fn os_get_thread_id() -> cs32;
    pub unsafe fn os_get_parent_thread_id() -> cs32;
    pub unsafe fn os_thread_continue(id: cs32) -> cs32;
    pub unsafe fn os_thread_stop(id: cs32) -> cs32;

    pub unsafe fn os_message_queue_create(ptr: *mut core::ffi::c_void, id: cu32) -> cs32;
    pub unsafe fn os_message_queue_destroy(queue_id: cs32) -> cs32;
    pub unsafe fn os_message_queue_receive(queue_id: cs32, message: *mut cu32, flags: cu32)
    -> cs32;
    pub unsafe fn os_message_queue_send(queue_id: cs32, message: cu32, flags: cs32) -> cs32;
    pub unsafe fn os_message_queue_now(queue_id: cs32, message: cu32, flags: cs32) -> cs32;
    pub unsafe fn os_message_queue_ack(message: *mut core::ffi::c_void, result: cs32) -> cs32;

    pub unsafe fn os_heap_create(ptr: *mut core::ffi::c_void, size: cs32) -> cs32;
    pub unsafe fn os_heap_destroy(heap: cs32) -> cs32;
    pub unsafe fn os_heap_alloc(heap: cs32, size: cu32) -> *mut core::ffi::c_void;
    pub unsafe fn os_heap_alloc_aligned(
        heap: cs32,
        size: cs32,
        align: cs32,
    ) -> *mut core::ffi::c_void;
    pub unsafe fn os_heap_free(heap: cs32, ptr: *mut core::ffi::c_void);

    pub unsafe fn __os_sync_before_read(ptr: *mut core::ffi::c_void, size: cs32);
    pub unsafe fn __os_sync_after_write(ptr: *mut core::ffi::c_void, size: cs32);

    pub unsafe fn os_set_dvd_video_mode(enable: cu32);
    pub unsafe fn os_set_ahbprot(enable: cu32);

    pub unsafe fn os_open(device: *const core::ffi::c_char, mode: cs32) -> cs32;
    pub unsafe fn os_close(fd: cs32) -> cs32;
    pub unsafe fn os_read(fd: cs32, d: *mut core::ffi::c_void, len: cs32) -> cs32;
    pub unsafe fn os_write(fd: cs32, s: *mut core::ffi::c_void, len: cs32) -> cs32;
    pub unsafe fn os_seek(fd: cs32, offset: cs32, mode: cs32) -> cs32;
    pub unsafe fn os_ioctl(
        fd: cs32,
        request: cs32,
        in_ptr: *mut core::ffi::c_void,
        bytes_in: cs32,
        out_ptr: *mut core::ffi::c_void,
        bytes_out: cs32,
    ) -> cs32;
    pub unsafe fn os_ioctlv(
        fd: cs32,
        request: cs32,
        bytes_in: cs32,
        bytes_out: cs32,
        vector: *mut Ioctlv,
    ) -> cs32;

    pub unsafe fn os_open_async(
        device: *const core::ffi::c_char,
        mode: cs32,
        ipc_cb: *mut core::ffi::c_void,
        usrdata: *mut core::ffi::c_void,
    ) -> cs32;
    pub unsafe fn os_close_async(
        fd: cs32,
        ipc_cb: *mut core::ffi::c_void,
        usrdata: *mut core::ffi::c_void,
    ) -> cs32;
    pub unsafe fn os_ioctl_async(
        fd: cs32,
        request: cs32,
        in_ptr: *mut core::ffi::c_void,
        bytes_in: cs32,
        out_ptr: *mut core::ffi::c_void,
        bytes_out: cs32,
        ...
    ) -> cs32;
    pub unsafe fn os_ioctlv_async(
        fd: cs32,
        request: cs32,
        bytes_in: cs32,
        bytes_out: cs32,
        vector: *mut Ioctlv,
        ...
    ) -> cs32;

    pub unsafe fn os_create_timer(
        time_us: cs32,
        repeat_time_us: cs32,
        message_queue: cs32,
        message: cs32,
    ) -> cs32;
    pub unsafe fn os_destroy_timer(timer_id: cs32) -> cs32;
    pub unsafe fn os_stop_timer(timer_id: cs32) -> cs32;
    pub unsafe fn os_restart_timer(timer_id: cs32, time_us: cs32, repeat_time_us: cs32) -> cs32;
    pub unsafe fn os_timer_now(timer_id: cs32) -> cs32;

    pub unsafe fn os_device_register(
        device_name: *const core::ffi::c_char,
        queue_handle: cs32,
    ) -> cs32;
    pub unsafe fn os_register_event_handler(device: cs32, queue: cs32, message: cs32) -> cs32;
    pub unsafe fn os_unregister_event_handler(device: cs32) -> cs32;

    #[link_name = "os_software_IRQ"]
    pub unsafe fn os_software_irq(dev: cs32) -> cs32;

    pub unsafe fn os_ios_boot(path: *const core::ffi::c_char, flag: cu32, version: cu32) -> cs32;
    pub unsafe fn os_kernel_set_version(version: cu32) -> cs32;
    pub unsafe fn os_kernel_get_version() -> cs32;
    pub unsafe fn os_get_key(keyid: cs32, out: *mut core::ffi::c_void);
    pub unsafe fn os_virt_to_phys(ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void;

    // ARM syscalls

    pub unsafe fn svc_write(str: *const core::ffi::c_char);
}
