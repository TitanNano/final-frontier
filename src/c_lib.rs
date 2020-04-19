#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

include!("bindings.rs");

pub fn c_main(argc: usize, args: Vec<String>) -> i32 {
    let argc = argc as i32;

    let args = args.into_iter().map(|string| {
        let cstring = CString::new(string).expect("CString::new failed");

        cstring.into_raw()
    }).collect::<Vec<*mut i8>>().as_mut_ptr();

    unsafe {
        lib_main(argc, args)
    }
}

pub fn c_Main_ReadParameters(argc: usize, args: Vec<String>) {
    let argc = argc as i32;

    let args = args.into_iter().map(|string| {
        let cstring = CString::new(string).expect("CString::new failed");

        cstring.into_raw()
    }).collect::<Vec<*mut i8>>().as_mut_ptr();

    unsafe {
        Main_ReadParameters(argc, args);
    }
}
