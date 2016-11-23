//! FFI bindings for `he-profiler.h`.

#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_uint, uint64_t, c_char};

#[repr(C)]
/// Representation of native C struct `he_profiler_event`.
pub struct he_profiler_event {
    pub start_time: uint64_t,
    pub start_energy: uint64_t,
    pub end_time: uint64_t,
    pub end_energy: uint64_t,
}

extern "C" {
    pub fn he_profiler_init(num_profilers: c_uint,
                            profiler_names: *const *const c_char,
                            window_sizes: *const uint64_t,
                            default_window_size: uint64_t,
                            app_profiler_id: c_uint,
                            app_profiler_min_sleep_us: uint64_t,
                            log_path: *const c_char) -> c_int;

    pub fn he_profiler_event_begin(event: *mut he_profiler_event) -> c_int;

    pub fn he_profiler_event_end(event: *mut he_profiler_event,
                                 profiler: c_uint,
                                 id: uint64_t,
                                 work: uint64_t) -> c_int;

    pub fn he_profiler_event_end_begin(event: *mut he_profiler_event,
                                       profiler: c_uint,
                                       id: uint64_t,
                                       work: uint64_t) -> c_int;

    pub fn he_profiler_finish() -> c_int;
}
