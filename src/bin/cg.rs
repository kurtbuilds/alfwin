use core_graphics::display::*;
use core_foundation::string::*;

use core_foundation::base::*;
use std::ffi::{CStr, c_void};
use std::ops::Deref;
use core_foundation::number::{CFNumberGetType, CFNumberGetTypeID, CFNumberGetValue, CFNumberRef, CFNumberType};
use core_graphics::window::{kCGWindowLayer, kCGWindowName, kCGWindowOwnerName};
use objc_foundation::{INSString, NSString};
use objc_id::Id;


fn get_dict_string(dic_ref: CFDictionaryRef, key: CFStringRef) -> Option<String> {
    let mut value: *const c_void = std::ptr::null();
    if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) } != 0 {
        let type_id: CFTypeID = unsafe { CFGetTypeID(value) };
        if type_id != unsafe { CFStringGetTypeID() } {
            return None;
        }
        let c_ptr = unsafe { CFStringGetCStringPtr(value.cast(), kCFStringEncodingUTF8) };
        if c_ptr.is_null() {
            // Failed to read CFString. Try to read NSString.
            let nss: Id<NSString> = unsafe { Id::from_ptr(value as *mut NSString) };
            std::str::from_utf8(nss.deref().as_str().as_bytes()).map(|s| s.to_string()).ok()
        } else {
            let c_result = unsafe { CStr::from_ptr(c_ptr) };
            c_result.to_str().map(String::from).ok()
        }
    } else {
        None
    }
}


fn get_dict_number(dic_ref: CFDictionaryRef, key: CFStringRef) -> Option<i64> {
    let mut value: *const c_void = std::ptr::null();
    if unsafe { CFDictionaryGetValueIfPresent(dic_ref, key.to_void(), &mut value) } != 0 {
        let type_id: CFTypeID = unsafe { CFGetTypeID(value) };
        if type_id != unsafe { CFNumberGetTypeID() } {
            return None;
        }
        let value = value as CFNumberRef;
        match unsafe {CFNumberGetType(value) } {
            I64 => {
                let mut result: i64 = 0;
                let result_ref: *mut i64 = &mut result;
                if unsafe {CFNumberGetValue(value, I64, result_ref.cast()) } {
                    Some(result)
                } else {
                    None
                }
            },
            I32 => {
                let mut result: i32 = 0;
                let result_ref: *mut i32 = &mut result;
                if unsafe {CFNumberGetValue(value, I32, result_ref.cast()) } {
                    Some(result as i64)
                } else {
                    None
                }
            },
            n => {
                eprintln!("Unsupported Number of typeId: {}", n);
                None
            }
        }
    } else {
        None
    }
}

fn main() {
    const options: CGWindowListOption = kCGWindowListOptionOnScreenOnly;
    let window_list_info = unsafe { CGWindowListCopyWindowInfo(options, kCGNullWindowID) };
    let count = unsafe { CFArrayGetCount(window_list_info) };

    for i in 0..count {
        let dic_ref = unsafe { CFArrayGetValueAtIndex(window_list_info, i as isize) as CFDictionaryRef };
        let mut owner_name: *const c_void = std::ptr::null();
        let mut window_name: *const c_void = std::ptr::null();

        let app_name = get_dict_string(dic_ref, unsafe {kCGWindowOwnerName}).unwrap();
        let name = get_dict_string(dic_ref, unsafe {kCGWindowName}).unwrap();
        let layer = get_dict_number(dic_ref, unsafe {kCGWindowLayer}).unwrap();

        if layer != 0 {
            continue;
        }
        println!("{} {} {}", app_name, name, layer);
        // let r1 = unsafe { CFDictionaryGetValueIfPresent(dic_ref, kCGWindowOwnerName.to_void(), &mut owner_name) != 0 };
        // let r2 = unsafe { CFDictionaryGetValueIfPresent(dic_ref, kCGWindowName.to_void(), &mut window_name) != 0 };
        // let type_id: CFTypeID = unsafe { CFGetTypeID(owner_name) };
        // println!("{}", type_id);
        // let type_id: CFTypeID = unsafe { CFGetTypeID(window_name) };
        // println!("{}", type_id);

        // if r1 && r2 {
        //     let c_ptr = unsafe { CFStringGetCStringPtr(owner_name.cast(), kCFStringEncodingUTF8) };
        //     if !c_ptr.is_null() {
        //         let c_result = unsafe { CStr::from_ptr(c_ptr) };
        //         let result = String::from(c_result.to_str().unwrap());
        //         println!("window owner name: {}", result);
        //     }
        //     let c_ptr = unsafe { CFStringGetCStringPtr(window_name.cast(), kCFStringEncodingUTF8) };
        //     if !c_ptr.is_null() {
        //         let c_result = unsafe { CStr::from_ptr(c_ptr) };
        //         let result = String::from(c_result.to_str().unwrap());
        //         println!("CPTR window name: {}", result);
        //     } else {
        //         let nss: Id<NSString> = unsafe { Id::from_ptr(window_name as *mut NSString) };
        //         let str = std::str::from_utf8(nss.deref().as_str().as_bytes());
        //         str.map(|s| println!("NSS window name: {}", s));
        //     }
        // }

        // let cf_ref = owner_name as CFStringRef;
        // let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
        // if !c_ptr.is_null() {
        //     let c_result = unsafe { CStr::from_ptr(c_ptr) };
        //     let result = String::from(c_result.to_str().unwrap());
        //     println!("window owner name: {}", result);
    }
    //
    //     let cf_ref = window_name as CFStringRef;
    //     let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
    //     if !c_ptr.is_null() {
    //         let c_result = unsafe { CStr::from_ptr(c_ptr) };
    //         let result = String::from(c_result.to_str().unwrap());
    //         println!("window name: {}", result)
    //     } else {
    //         println!("window name str is null");
    //     }
    unsafe {
        CFRelease(window_list_info as CFTypeRef)
    }
}