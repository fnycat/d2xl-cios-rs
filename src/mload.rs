#[derive(Copy, Clone)]
pub enum MloadCommand {
    GetIosInfo = 0x4D4C4401,
    GetMloadVersion = 0x4D4C402,
    LoadElf = 0x4D4C4480,
    RunElf = 0x4D4C4481,
    RunThread = 0x4D4C4482,
    StopThread = 0x4D4C4484,
    ContinueThread = 0x4D4C4485,
    GetLoadBase = 0x4D4C4490,
    Memset = 0x4D4C4491,
    SetLogMode = 0x4D4C44D0,
    GetLogBuffer = 0x4D4C44D1,
    GetLogBufferAndEmpty = 0x4D4C44D2,
    SetLogRingBuffer = 0x4D4C44D3,
    SetStealthMode = 0x4D4C44E0,
}

#[derive(Copy, Clone)]
pub enum LogMode {
    None = 0x00,
    Buffer = 0x01,
    Gecko = 0x02,
    RingBuffer = 0x03,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DataElf {
    pub start: *mut core::ffi::c_void,
    pub prio: core::ffi::c_int,
    pub stack: *mut core::ffi::c_void,
    pub stack_size: core::ffi::c_int,
}

// pub fn mload_init() {}

// pub fn mload_close() {}

// pub fn mload_get_load_base(starlet_base: *mut u32, size: *mut core::ffi::c_int) -> i32 {}

// pub fn mload_module(addr: *mut core::ffi::c_void) -> i32 {}

// pub fn mload_elf(my_elf: *const core::ffi::c_void, data_elf: *mut DataElf) -> i32 {}

// pub fn mload_run_thread(starlet_addr: *mut core::ffi::c_void, starlet_stack_top: *mut core::ffi::c_void, stack_size: core::ffi::c_int, priority: core::ffi::c_int); return 0; }
