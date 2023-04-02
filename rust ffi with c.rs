use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn greet(name: *const c_char) {
    let c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let name = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => "World",
    };
    println!("Hello, {}!", name);
}
