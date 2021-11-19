use core_graphics::display::*;
use core_foundation::string::*;

use core_foundation::base::*;
use std::ffi::{ CStr, c_void };
use core_graphics::window::{kCGWindowName, kCGWindowOwnerName};

fn main() {
    const OPTIONS: CGWindowListOption = kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;
    let window_list_info = unsafe { CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) };
    let count = unsafe { CFArrayGetCount(window_list_info) };

    for i in 0..count {

        let dic_ref = unsafe { CFArrayGetValueAtIndex(window_list_info, i as isize) as CFDictionaryRef };
        let mut owner_name: *const c_void = std::ptr::null();
        let mut window_name: *const c_void = std::ptr::null();

        let r1 = unsafe { CFDictionaryGetValueIfPresent(dic_ref, kCGWindowOwnerName.to_void(), &mut owner_name) != 0 };
        let r2 = unsafe { CFDictionaryGetValueIfPresent(dic_ref, kCGWindowName.to_void(), &mut window_name) != 0 };
        if  r1 && r2 {
            let cf_ref = owner_name as CFStringRef;
            let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
            if !c_ptr.is_null() {
                let c_result = unsafe { CStr::from_ptr(c_ptr) };
                let result = String::from(c_result.to_str().unwrap());
                println!("window owner name: {}", result)
            }

            let cf_ref = window_name as CFStringRef;
            let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
            if !c_ptr.is_null() {
                let c_result = unsafe { CStr::from_ptr(c_ptr) };
                let result = String::from(c_result.to_str().unwrap());
                println!("window name: {}", result)
            } else {
                println!("window name str is null");
            }
        }
    }

    unsafe {
        CFRelease(window_list_info as CFTypeRef)
    }
}