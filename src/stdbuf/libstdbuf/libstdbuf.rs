extern crate libc;
#[macro_use]
extern crate cpp;

#[macro_use]
extern crate uucore;

use libc::{c_int, size_t, c_char, FILE, _IOFBF, _IONBF, _IOLBF};
use std::env;
use std::ptr;

cpp!{{
    #include <cstdio>

    extern "C" {
        void __stdbuf(void);

        void __attribute((constructor))
        __stdbuf_init(void) {
            __stdbuf();
        }

        FILE *__stdbuf_get_stdin() { return stdin; }
        FILE *__stdbuf_get_stdout() { return stdout; }
        FILE *__stdbuf_get_stderr() { return stderr; }
    }
}}

extern {
    fn __stdbuf_get_stdin() -> *mut FILE;
    fn __stdbuf_get_stdout() -> *mut FILE;
    fn __stdbuf_get_stderr() -> *mut FILE;
}

fn set_buffer(stream: *mut FILE, value: &str) {
    let (mode, size): (c_int, size_t) = match value {
        "0" => (_IONBF, 0 as size_t),
        "L" => (_IOLBF, 0 as size_t),
        input => {
            let buff_size: usize = match input.parse() {
                Ok(num) => num,
                Err(e) => crash!(1, "incorrect size of buffer!: {}", e)
            };
            (_IOFBF, buff_size as size_t)
        }
    };
    let res: c_int;
    unsafe {
        let buffer: *mut c_char = ptr::null_mut();
        assert!(buffer.is_null());
        res = libc::setvbuf(stream, buffer, mode, size);
    }
    if res != 0 {
        crash!(res, "error while calling setvbuf!");
    }
}

#[no_mangle]
pub unsafe extern "C" fn __stdbuf() {
    if let Ok(val) = env::var("_STDBUF_E") {
        set_buffer(__stdbuf_get_stderr(), &val);
    }
    if let Ok(val) = env::var("_STDBUF_I") {
        set_buffer(__stdbuf_get_stdin(), &val);
    }
    if let Ok(val) = env::var("_STDBUF_O") {
        set_buffer(__stdbuf_get_stdout(), &val);
    }
}
