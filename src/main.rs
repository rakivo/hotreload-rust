use libloading::{Symbol, Library};

use std::{
    ffi::CString,
    thread::sleep,
    time::Duration
};

const PLUG_PATH: &str = if cfg!(target_os = "linux") {
    "./target/debug/libplug.so"
} else if cfg!(target_os = "windows") {
    ".\\target\\debug\\libplug.dll"
} else {
    "./target/debug/libplug.dylib"
};

fn main() {
    loop {
        let lib = unsafe { Library::new(PLUG_PATH).expect("failed to load the library") };
        let greet: Symbol::<fn() -> CString> = unsafe {
            lib.get(b"greet").expect("could not find `greet` function")
        };

        println!("{msg:?}", msg = greet());

        sleep(Duration::from_secs(1));
    }
}
