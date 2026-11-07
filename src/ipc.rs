use crate::types::*;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageOpen {
    pub device: *mut core::ffi::c_char,
    pub mode: cu32,
    pub resultfd: cu32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageReadWrite {
    pub data: *mut core::ffi::c_void,
    pub len: cu32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageSeek {
    pub offset: cs32,
    pub origin: cs32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageIoctl {
    pub command: cu32,
    pub buf_in: *mut cu32,
    pub len_in: cu32,
    pub buf_io: *mut cu32,
    pub len_io: cu32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ioctlv {
    pub data: *mut core::ffi::c_void,
    pub len: cu32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageIoctlv {
    pub command: cu32,
    pub num_in: cu32,
    pub num_io: cu32,
    pub vector: *mut Ioctlv,
}

#[repr(C, packed)]
pub union IpcMessageData {
    pub open: IpcMessageOpen,
    pub read: IpcMessageReadWrite,
    pub write: IpcMessageReadWrite,
    pub seek: IpcMessageSeek,
    pub ioctl: IpcMessageIoctl,
    pub ioctlv: IpcMessageIoctlv,
}

#[repr(C, packed)]
pub struct IpcMessage {
    pub command: cu32,
    pub result: cu32,
    pub fd: cu32,
    pub data: IpcMessageData,
}

unsafe extern "C" {
    #[link_name = "InvalidateVector"]
    pub unsafe fn invalidate_vector(vector: *mut Ioctlv, in_len: cu32, io_len: cu32);

    #[link_name = "FlushVector"]
    pub unsafe fn flush_vector(vector: *mut Ioctlv, in_len: cu32, io_len: cu32);
}
