#![allow(non_camel_case_types)]
use libc::{c_void, c_int};

pub enum ubertooth_t {}

#[link(name = "ubertooth")]
extern {
    
    pub fn print_version() -> c_void;
    pub fn ubertooth_init() -> *mut ubertooth_t;
    pub fn register_cleanup_handler(ut: *mut ubertooth_t, do_exit: c_int) -> c_void;
    pub fn ubertooth_connect(ut: *mut ubertooth_t, device_number: c_int) -> c_int;
}
