// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstVideoSink")]
    pub struct VideoSink(Object<ffi::GstVideoSink, ffi::GstVideoSinkClass>) @extends gst_base::BaseSink, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_video_sink_get_type(),
    }
}

impl VideoSink {
    pub const NONE: Option<&'static VideoSink> = None;
}

unsafe impl Send for VideoSink {}
unsafe impl Sync for VideoSink {}

pub trait VideoSinkExt: 'static {
    #[doc(alias = "show-preroll-frame")]
    fn shows_preroll_frame(&self) -> bool;

    #[doc(alias = "show-preroll-frame")]
    fn set_show_preroll_frame(&self, show_preroll_frame: bool);

    #[doc(alias = "show-preroll-frame")]
    fn connect_show_preroll_frame_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<VideoSink>> VideoSinkExt for O {
    fn shows_preroll_frame(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-preroll-frame")
    }

    fn set_show_preroll_frame(&self, show_preroll_frame: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-preroll-frame", show_preroll_frame)
    }

    fn connect_show_preroll_frame_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_preroll_frame_trampoline<
            P: IsA<VideoSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-preroll-frame\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_preroll_frame_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
