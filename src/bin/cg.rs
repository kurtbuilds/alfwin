#![allow(non_snake_case)]
use core_foundation::string::*;
use core_graphics::display::*;

use core_foundation::base::*;
use core_foundation::number::{
    kCFNumberSInt32Type, kCFNumberSInt64Type, CFNumberGetType, CFNumberGetTypeID, CFNumberGetValue,
    CFNumberRef,
};
use core_graphics::window::{kCGWindowLayer, kCGWindowName, kCGWindowOwnerName};
use objc_foundation::{INSString, NSString};
use objc_id::Id;
use std::ffi::{c_void, CStr};
use std::ops::Deref;

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
            std::str::from_utf8(nss.deref().as_str().as_bytes())
                .map(|s| s.to_string())
                .ok()
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
        match unsafe { CFNumberGetType(value) } {
            I64 if I64 == kCFNumberSInt64Type => {
                let mut result: i64 = 0;
                let result_ref: *mut i64 = &mut result;
                if unsafe { CFNumberGetValue(value, I64, result_ref.cast()) } {
                    Some(result)
                } else {
                    None
                }
            }
            I32 if I32 == kCFNumberSInt32Type => {
                let mut result: i32 = 0;
                let result_ref: *mut i32 = &mut result;
                if unsafe { CFNumberGetValue(value, I32, result_ref.cast()) } {
                    Some(result as i64)
                } else {
                    None
                }
            }
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
    const OPTIONS: CGWindowListOption = kCGWindowListOptionOnScreenOnly;
    let window_list_info = unsafe { CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) };
    let count = unsafe { CFArrayGetCount(window_list_info) };

    for i in 0..count {
        let dic_ref =
            unsafe { CFArrayGetValueAtIndex(window_list_info, i as isize) as CFDictionaryRef };
        let mut _owner_name: *const c_void = std::ptr::null();
        let mut _window_name: *const c_void = std::ptr::null();

        let app_name = get_dict_string(dic_ref, unsafe { kCGWindowOwnerName }).unwrap();
        let name = get_dict_string(dic_ref, unsafe { kCGWindowName }).unwrap();
        let layer = get_dict_number(dic_ref, unsafe { kCGWindowLayer });

        if layer.is_none() || layer.unwrap() != 0 {
            continue;
        }
        println!("{} {} {}", app_name, name, layer.unwrap());
    }
    unsafe { CFRelease(window_list_info as CFTypeRef) }
}
