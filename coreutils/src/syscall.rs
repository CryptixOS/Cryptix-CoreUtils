use std::arch::asm;

type SysResult = isize;
pub enum ID {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    UName = 63,
    GetTimeOfDay = 96,
    GetEUID = 107,
    SetHostName = 170,
    Time = 201,
}

#[inline(always)]
pub fn syscall(id: ID, arg1: usize, arg2: usize, arg3: usize) -> SysResult {
    let ret: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") id as usize,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            lateout("rax") ret,
        );
    }

    ret
}
#[inline(always)]
pub fn syscall3(id: ID, arg1: usize, arg2: usize, arg3: usize) -> SysResult {
    return syscall(id, arg1, arg2, arg3);
}
#[inline(always)]
pub fn syscall2(id: ID, arg1: usize, arg2: usize) -> SysResult {
    return syscall3(id, arg1, arg2, 0);
}
#[inline(always)]
pub fn syscall1(id: ID, arg1: usize) -> SysResult {
    return syscall2(id, arg1, 0);
}
#[inline(always)]
pub fn syscall0(id: ID) -> SysResult {
    return syscall1(id, 0);
}

/* ============================================================= */
/* Convenience wrappers for common syscalls                      */
/* ============================================================= */
#[inline(always)]
pub fn sys_read(fd: usize, buf: *mut u8, count: usize) -> SysResult {
    syscall3(ID::Read, fd, buf as usize, count)
}
#[inline(always)]
pub fn sys_write(fd: usize, buf: *const u8, count: usize) -> SysResult {
    syscall3(ID::Write, fd, buf as usize, count)
}
#[inline(always)]
pub fn sys_open(path: *const u8, flags: isize, mode: usize) -> SysResult {
    syscall3(ID::Open, path as usize, flags as usize, mode)
}
#[inline(always)]
pub fn sys_close(fd: usize) -> SysResult {
    syscall1(ID::Close, fd)
}
#[repr(C)]
pub struct UtsName {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}
pub fn sys_uname(buf: &mut UtsName) -> SysResult {
    return syscall1(ID::UName, buf as *mut _ as usize);
}
impl Default for UtsName {
    fn default() -> Self {
        Self {
            sysname: [0u8; 65],
            nodename: [0u8; 65],
            release: [0u8; 65],
            version: [0u8; 65],
            machine: [0u8; 65],
            domainname: [0u8; 65],
        }
    }
}
#[repr(C)]
pub struct TimeVal {
    pub tv_sec: usize,
    pub tv_usec: usize,
}
#[repr(C)]
pub struct TimeZone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}

#[inline(always)]
pub fn sys_gettimeofday(tv: &mut TimeVal, tz: &mut TimeZone) -> isize {
    syscall3(
        ID::GetTimeOfDay,
        tv as *mut _ as usize,
        tz as *mut _ as usize,
        0,
    )
}
#[inline(always)]
pub fn sys_sethostname(buf: *const u8, len: usize) -> isize {
    syscall3(ID::SetHostName, buf as usize, len, 0)
}
