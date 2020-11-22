// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::VideoCodecFrame;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::StaticType;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::Value;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::boxed::Box as Box_;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct VideoDecoder(Object<ffi::GstVideoDecoder, ffi::GstVideoDecoderClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || ffi::gst_video_decoder_get_type(),
    }
}

unsafe impl Send for VideoDecoder {}
unsafe impl Sync for VideoDecoder {}

pub const NONE_VIDEO_DECODER: Option<&VideoDecoder> = None;

pub trait VideoDecoderExt: 'static {
    fn add_to_frame(&self, n_bytes: i32);

    fn allocate_output_buffer(&self) -> Result<gst::Buffer, glib::BoolError>;

    fn get_buffer_pool(&self) -> Option<gst::BufferPool>;

    fn get_estimate_rate(&self) -> i32;

    fn get_max_decode_time(&self, frame: &VideoCodecFrame) -> gst::ClockTimeDiff;

    fn get_max_errors(&self) -> i32;

    fn get_needs_format(&self) -> bool;

    fn get_packetized(&self) -> bool;

    fn get_pending_frame_size(&self) -> usize;

    fn get_qos_proportion(&self) -> f64;

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode);

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps;

    fn set_estimate_rate(&self, enabled: bool);

    fn set_max_errors(&self, num: i32);

    fn set_needs_format(&self, enabled: bool);

    fn set_packetized(&self, packetized: bool);

    fn set_use_default_pad_acceptcaps(&self, use_: bool);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_property_qos(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_property_qos(&self, qos: bool);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_max_errors_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<VideoDecoder>> VideoDecoderExt for O {
    fn add_to_frame(&self, n_bytes: i32) {
        unsafe {
            ffi::gst_video_decoder_add_to_frame(self.as_ref().to_glib_none().0, n_bytes);
        }
    }

    fn allocate_output_buffer(&self) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_video_decoder_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib::glib_bool_error!("Failed to allocate output buffer"))
        }
    }

    fn get_buffer_pool(&self) -> Option<gst::BufferPool> {
        unsafe {
            from_glib_full(ffi::gst_video_decoder_get_buffer_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_estimate_rate(&self) -> i32 {
        unsafe { ffi::gst_video_decoder_get_estimate_rate(self.as_ref().to_glib_none().0) }
    }

    fn get_max_decode_time(&self, frame: &VideoCodecFrame) -> gst::ClockTimeDiff {
        unsafe {
            ffi::gst_video_decoder_get_max_decode_time(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
            )
        }
    }

    fn get_max_errors(&self) -> i32 {
        unsafe { ffi::gst_video_decoder_get_max_errors(self.as_ref().to_glib_none().0) }
    }

    fn get_needs_format(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_decoder_get_needs_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_packetized(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_decoder_get_packetized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pending_frame_size(&self) -> usize {
        unsafe { ffi::gst_video_decoder_get_pending_frame_size(self.as_ref().to_glib_none().0) }
    }

    fn get_qos_proportion(&self) -> f64 {
        unsafe { ffi::gst_video_decoder_get_qos_proportion(self.as_ref().to_glib_none().0) }
    }

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            ffi::gst_video_decoder_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_video_decoder_proxy_getcaps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn set_estimate_rate(&self, enabled: bool) {
        unsafe {
            ffi::gst_video_decoder_set_estimate_rate(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_max_errors(&self, num: i32) {
        unsafe {
            ffi::gst_video_decoder_set_max_errors(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_needs_format(&self, enabled: bool) {
        unsafe {
            ffi::gst_video_decoder_set_needs_format(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_packetized(&self, packetized: bool) {
        unsafe {
            ffi::gst_video_decoder_set_packetized(
                self.as_ref().to_glib_none().0,
                packetized.to_glib(),
            );
        }
    }

    fn set_use_default_pad_acceptcaps(&self, use_: bool) {
        unsafe {
            ffi::gst_video_decoder_set_use_default_pad_acceptcaps(
                self.as_ref().to_glib_none().0,
                use_.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_property_qos(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"qos\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `qos` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_property_qos(&self, qos: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"qos\0".as_ptr() as *const _,
                Value::from(&qos).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_max_errors_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_errors_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstVideoDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<VideoDecoder>,
        {
            let f: &F = &*(f as *const F);
            f(&VideoDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-errors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_errors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstVideoDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<VideoDecoder>,
        {
            let f: &F = &*(f as *const F);
            f(&VideoDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_qos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
