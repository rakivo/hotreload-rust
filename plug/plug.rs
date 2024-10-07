use std::ffi::CString;

#[no_mangle]
fn greet() -> CString {
	CString::new("hello world").unwrap()
}
