// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use std::mem;

pub fn type_find_helper_for_data<P: IsA<gst::Object>, R: AsRef<[u8]>>(
    obj: Option<&P>,
    data: R,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let data = data.as_ref();
        let (ptr, len) = (data.as_ptr(), data.len());
        let ret = ffi::gst_type_find_helper_for_data(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(ptr),
            len,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::glib_bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
pub fn type_find_helper_for_data_with_extension<P: IsA<gst::Object>, R: AsRef<[u8]>>(
    obj: Option<&P>,
    data: R,
    extension: Option<&str>,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let data = data.as_ref();
        let (ptr, len) = (data.as_ptr(), data.len());
        let ret = ffi::gst_type_find_helper_for_data_with_extension(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(ptr),
            len,
            extension.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::glib_bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

pub fn type_find_helper_for_buffer<P: IsA<gst::Object>>(
    obj: Option<&P>,
    buf: &gst::Buffer,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let ret = ffi::gst_type_find_helper_for_buffer(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            buf.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::glib_bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[cfg(any(feature = "v1_16", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_16")))]
pub fn type_find_helper_for_buffer_with_extension<P: IsA<gst::Object>>(
    obj: Option<&P>,
    buf: &gst::Buffer,
    extension: Option<&str>,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let ret = ffi::gst_type_find_helper_for_buffer_with_extension(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            buf.to_glib_none().0,
            extension.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::glib_bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}
