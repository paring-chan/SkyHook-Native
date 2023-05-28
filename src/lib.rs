use std::{
    ffi::{c_char, CString},
    ptr::null,
};

use chrono::NaiveDateTime;

extern crate chrono;
extern crate skyhook;

#[repr(C)]
pub enum NativeEventType {
    KeyPressed,
    KeyReleased,
}

#[repr(C)]
pub struct NativeEvent {
    pub time_sec: i64,
    pub time_nsec: u32,
    pub event_type: NativeEventType,
    pub vkey: u16,
    pub keycode: u16,
}

static mut CALLBACK: Option<extern "C" fn(NativeEvent)> = None;

fn send_callback(ev: NativeEvent) {
    unsafe {
        if let Some(cb) = CALLBACK {
            cb(ev);
        }
    }
}

fn get_time(time: NaiveDateTime) -> (i64, u32) {
    (time.timestamp(), time.timestamp_subsec_nanos())
}

#[no_mangle]
pub extern "C" fn start_hook(callback: extern "C" fn(NativeEvent)) -> *const c_char {
    unsafe {
        CALLBACK = Some(callback);
    }

    if let Err(e) = skyhook::run(move |event| {
        let (sec, nsec) = get_time(event.time);

        let event = match event.data {
            skyhook::types::EventData::KeyPress(label, key) => NativeEvent {
                event_type: NativeEventType::KeyPressed,
                vkey: label as u16,
                keycode: key,
                time_sec: sec,
                time_nsec: nsec,
            },
            skyhook::types::EventData::KeyRelease(label, key) => NativeEvent {
                event_type: NativeEventType::KeyReleased,
                vkey: label as u16,
                keycode: key,
                time_sec: sec,
                time_nsec: nsec,
            },
        };
        send_callback(event);
    }) {
        let cstr = CString::new(e.message).unwrap();
        return cstr.into_raw();
    }

    null()
}

#[no_mangle]
pub extern "C" fn stop_hook() -> *const c_char {
    if let Err(e) = skyhook::stop() {
        let cstr = CString::new(e.message).unwrap();
        return cstr.into_raw();
    }

    null()
}

#[no_mangle]
pub extern "C" fn hook_is_running() -> bool {
    skyhook::is_running()
}
