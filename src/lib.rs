use std::{
    ffi::{c_char, CString},
    ptr::null,
    time::{SystemTime, UNIX_EPOCH},
};

extern crate skyhook;

#[repr(C)]
pub enum NativeEventType {
    KeyPressed,
    KeyReleased,
}

#[repr(C)]
pub struct NativeEvent {
    pub time: usize,
    pub event_type: NativeEventType,
    pub label: u16,
    pub key: u16,
}

static mut CALLBACK: Option<extern "C" fn(NativeEvent)> = None;

fn send_callback(ev: NativeEvent) {
    unsafe {
        if let Some(cb) = CALLBACK {
            cb(ev);
        }
    }
}

fn get_time(time: SystemTime) -> usize {
    time.duration_since(UNIX_EPOCH).unwrap().as_nanos() as usize
}

#[no_mangle]
pub extern "C" fn start_hook(callback: extern "C" fn(NativeEvent)) -> *const c_char {
    unsafe {
        CALLBACK = Some(callback);
    }

    if let Err(e) = skyhook::run(move |event| {
        let event = match event.data {
            skyhook::types::EventData::KeyPress(label, key) => NativeEvent {
                time: get_time(event.time),
                event_type: NativeEventType::KeyPressed,
                label: label as u16,
                key,
            },
            skyhook::types::EventData::KeyRelease(label, key) => NativeEvent {
                time: get_time(event.time),
                event_type: NativeEventType::KeyReleased,
                label: label as u16,
                key,
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
