use std::arch::asm;

type SysResult = isize;
pub type FileMode = u32;

// set-sticky
pub const S_ISVTX: FileMode = 0001000;
// set-group-id
pub const S_ISGID: FileMode = 0002000;
// set-user-id
pub const S_ISUID: FileMode = 0004000;

// owner
// read
pub const S_IREAD: FileMode = 0400;
// write
pub const S_IWRITE: FileMode = 0200;
// exec
pub const S_IEXEC: FileMode = 0100;

// rwx
pub const S_IRWXU: FileMode = 00700;
// read
pub const S_IRUSR: FileMode = S_IREAD;
// write
pub const S_IWUSR: FileMode = S_IWRITE;
// exec
pub const S_IXUSR: FileMode = S_IEXEC;

// group
// rwx
pub const S_IRWXG: FileMode = 00070;
// read
pub const S_IRGRP: FileMode = 00040;
// write
pub const S_IWGRP: FileMode = 00020;
// exec
pub const S_IXGRP: FileMode = 00010;

// others
// rwx
pub const S_IRWXO: FileMode = 00007;
// read
pub const S_IROTH: FileMode = 00004;
// write
pub const S_IWOTH: FileMode = 00002;
// exec
pub const S_IXOTH: FileMode = 00001;

pub const S_IRWXUGO: FileMode = S_IRWXU | S_IRWXG | S_IRWXO;
pub const S_IALLUGO: FileMode = S_ISUID | S_ISGID | S_ISVTX | S_IRWXUGO;
pub const S_IRUGO: FileMode = S_IRUSR | S_IRGRP | S_IROTH;
pub const S_IWUGO: FileMode = S_IWUSR | S_IWGRP | S_IWOTH;
pub const S_IXUGO: FileMode = S_IXUSR | S_IXGRP | S_IXOTH;

pub enum ID {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    GetPid = 39,
    UName = 63,
    ReadLink = 89,
    ChangeMode = 90,
    UMask = 95,
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
pub const O_RDONLY: isize = 0;
#[inline(always)]
pub fn sys_open(path: *const u8, flags: isize, mode: usize) -> SysResult {
    syscall3(ID::Open, path as usize, flags as usize, mode)
}
#[inline(always)]
pub fn sys_close(fd: usize) -> SysResult {
    syscall1(ID::Close, fd)
}
#[inline(always)]
pub fn sys_getpid() -> SysResult {
    return syscall0(ID::GetPid);
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
#[inline(always)]
pub fn sys_uname(buf: &mut UtsName) -> SysResult {
    return syscall1(ID::UName, buf as *mut _ as usize);
}
#[inline(always)]
pub fn sys_readlink(path: *const u8, buf: *mut u8, bufsz: usize) -> SysResult {
    return syscall3(ID::ReadLink, path as usize, buf as usize, bufsz);
}
#[inline(always)]
pub fn sys_chmod(path: *const u8, mode: FileMode) -> SysResult {
    return syscall2(ID::ChangeMode, path as usize, mode as usize);
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
pub fn sys_umask(mode: FileMode) -> FileMode {
    return syscall1(ID::UMask, mode as usize) as FileMode;
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
