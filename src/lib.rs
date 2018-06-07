#![allow(dead_code)]
#![allow(renamed_and_removed_lints)]
#![recursion_limit = "1024"]
extern crate libc;
#[macro_use]
extern crate error_chain;

mod native;
mod errors {
    error_chain! {
        errors {
            UbertoothNativeError(t: i32) {
                description("Ubertooth native library returned an error")
                display("Ubertooth error {}", t)
            }
        }
    }
}
use errors::*;

pub struct Ubertooth {
    ptr: *mut native::ubertooth_t,
}

pub enum UbertoothDeviceNumber {
    ZERO = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7
}

impl Ubertooth {
    fn from_ptr(ptr: *mut native::ubertooth_t) -> Self {
        Self {
            ptr
        }
    }

    pub fn connect(&self, device_number : UbertoothDeviceNumber) -> Result<()> {
        let result : i32;
        unsafe {
            result = native::ubertooth_connect(self.ptr, device_number as i32);
            if result < 0 {
                Err(ErrorKind::UbertoothNativeError(result))?
            } else {
                Ok(())
            }
        }
    }
}

fn print_version() {
    unsafe{ native::print_version(); }
}

fn init() -> Option<Ubertooth> {
    unsafe { 
        let ptr = native::ubertooth_init();
        if ptr.is_null() {
            None
        } else {
            native::register_cleanup_handler(ptr, 1);
            Some(Ubertooth::from_ptr(ptr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_returns_some() {
        init().expect("Init should not be null");
    }

    #[test]
    fn print_version_works() {
        print_version();
    }

    #[test]
    #[should_panic(expected = "Error(UbertoothNativeError(-1), State { next_error: None, backtrace: None })")]
    fn start_on_unavailable_device_fails() {
        let ubertooth = init().expect("Ubertooth subsystem could not initialize");
        ubertooth.connect(UbertoothDeviceNumber::SEVEN).unwrap();
    }
}
