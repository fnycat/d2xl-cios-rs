#[derive(Copy, Clone)]
pub enum IpcError {
    NoEntry = -6,
    NoMemory = -22,
    Invalid = -101,
    Access = -102,
    Exists = -105,
    /// Not sure why cios-lib includes a second copy of this error.
    NoEntry2 = -106,
}

impl TryFrom<i32> for IpcError {
    type Error = i32;

    /// Returns the original input as an error if it does not match a variant.
    fn try_from(value: i32) -> Result<Self, i32> {
        Ok(match value {
            -6 => Self::NoEntry,
            -22 => Self::NoMemory,
            -101 => Self::Invalid,
            -102 => Self::Access,
            -105 => Self::Exists,
            -106 => Self::NoEntry2,
            _ => return Err(value),
        })
    }
}

#[derive(Copy, Clone)]
pub enum IosCommand {
    Open = 0x01,
    Close = 0x02,
    Read = 0x03,
    Write = 0x04,
    Seek = 0x05,
    Ioctl = 0x06,
    Ioctlv = 0x07,
}

impl TryFrom<u32> for IosCommand {
    type Error = u32;

    /// Returns the original input as an error if it does not match a variant.
    fn try_from(value: u32) -> Result<Self, u32> {
        Ok(match value {
            0x01 => Self::Open,
            0x02 => Self::Close,
            0x03 => Self::Read,
            0x04 => Self::Write,
            0x05 => Self::Seek,
            0x06 => Self::Ioctl,
            0x07 => Self::Ioctlv,
            _ => return Err(value),
        })
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageOpen {
    pub device: *mut core::ffi::c_char,
    pub mode: u32,
    pub resultfd: u32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageReadWrite {
    pub data: *mut core::ffi::c_void,
    pub len: u32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageSeek {
    pub offset: i32,
    pub origin: i32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageIoctl {
    pub command: u32,
    pub buf_in: *mut u32,
    pub len_in: u32,
    pub buf_io: *mut u32,
    pub len_io: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ioctlv {
    pub data: *mut core::ffi::c_void,
    pub len: u32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IpcMessageIoctlv {
    pub command: u32,
    pub num_in: u32,
    pub num_io: u32,
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
    pub command: u32,
    pub result: u32,
    pub fd: u32,
    pub data: IpcMessageData,
}

unsafe extern "C" {
    #[link_name = "InvalidateVector"]
    pub unsafe fn invalidate_vector(vector: *mut Ioctlv, in_len: u32, io_len: u32);

    #[link_name = "FlushVector"]
    pub unsafe fn flush_vector(vector: *mut Ioctlv, in_len: u32, io_len: u32);
}
