use crate::syscall::syscall;
use core::mem::MaybeUninit;
use std::process::exit;

#[repr(C)]
pub struct UtsName {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65],
}
#[cfg(target_arch = "x86_64")]
const SYS_UNAME: usize = 63;

pub fn uname_main() -> i32 {
    let mut uts = MaybeUninit::<UtsName>::uninit();

    let res = syscall(SYS_UNAME, uts.as_mut_ptr() as usize, 0, 0);
    if res < 0 {
        eprintln!("uname syscall failed: {}", -res);
        exit(1);
    }

    let uts = unsafe { uts.assume_init() };

    let sysname_bytes = &uts.sysname;
    let len = sysname_bytes
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(sysname_bytes.len());
    let sysname = str::from_utf8(&sysname_bytes[..len]).unwrap_or("Unknown");

    println!("{}", sysname);

    0
}
