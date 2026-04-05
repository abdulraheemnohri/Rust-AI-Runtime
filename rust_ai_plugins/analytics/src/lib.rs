#[no_mangle]
pub extern "C" fn run_analytics(input: *const u8, len: usize) -> *mut i8 {
    let text = unsafe { std::str::from_utf8(std::slice::from_raw_parts(input, len)).unwrap() };
    let output = format!("Analytics response: {}", text);
    std::ffi::CString::new(output).unwrap().into_raw()
}
