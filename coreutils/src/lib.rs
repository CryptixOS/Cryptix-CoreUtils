pub mod clear;
pub mod syscall;
pub mod uname;
pub use clear::clear_main;
pub use uname::uname_main;

pub mod ls;
pub mod yes;

pub use yes::yes;
