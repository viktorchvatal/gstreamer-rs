// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use crate::AppLeakyType;
use crate::AppStreamType;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstAppSrc")]
    pub struct AppSrc(Object<ffi::GstAppSrc, ffi::GstAppSrcClass>) @extends gst::Element, gst::Object, @implements gst::URIHandler;

    match fn {
        type_ => || ffi::gst_app_src_get_type(),
    }
}

impl AppSrc {
    #[doc(alias = "gst_app_src_end_of_stream")]
    pub fn end_of_stream(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe { try_from_glib(ffi::gst_app_src_end_of_stream(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_app_src_get_caps")]
    #[doc(alias = "get_caps")]
    pub fn caps(&self) -> Option<gst::Caps> {
        unsafe { from_glib_full(ffi::gst_app_src_get_caps(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_src_get_current_level_buffers")]
    #[doc(alias = "get_current_level_buffers")]
    pub fn current_level_buffers(&self) -> u64 {
        unsafe { ffi::gst_app_src_get_current_level_buffers(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_app_src_get_current_level_bytes")]
    #[doc(alias = "get_current_level_bytes")]
    pub fn current_level_bytes(&self) -> u64 {
        unsafe { ffi::gst_app_src_get_current_level_bytes(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_src_get_current_level_time")]
    #[doc(alias = "get_current_level_time")]
    pub fn current_level_time(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::gst_app_src_get_current_level_time(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "gst_app_src_get_duration")]
    #[doc(alias = "get_duration")]
    pub fn duration(&self) -> Option<gst::ClockTime> {
        unsafe { from_glib(ffi::gst_app_src_get_duration(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_src_get_leaky_type")]
    #[doc(alias = "get_leaky_type")]
    pub fn leaky_type(&self) -> AppLeakyType {
        unsafe { from_glib(ffi::gst_app_src_get_leaky_type(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_src_get_max_buffers")]
    #[doc(alias = "get_max_buffers")]
    pub fn max_buffers(&self) -> u64 {
        unsafe { ffi::gst_app_src_get_max_buffers(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_app_src_get_max_bytes")]
    #[doc(alias = "get_max_bytes")]
    pub fn max_bytes(&self) -> u64 {
        unsafe { ffi::gst_app_src_get_max_bytes(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_src_get_max_time")]
    #[doc(alias = "get_max_time")]
    pub fn max_time(&self) -> Option<gst::ClockTime> {
        unsafe { from_glib(ffi::gst_app_src_get_max_time(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_app_src_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> i64 {
        unsafe { ffi::gst_app_src_get_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_app_src_get_stream_type")]
    #[doc(alias = "get_stream_type")]
    pub fn stream_type(&self) -> AppStreamType {
        unsafe { from_glib(ffi::gst_app_src_get_stream_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_app_src_push_sample")]
    pub fn push_sample(&self, sample: &gst::Sample) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_app_src_push_sample(
                self.to_glib_none().0,
                sample.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_app_src_set_callbacks")]
    //pub fn set_callbacks(&self, callbacks: /*Ignored*/&mut AppSrcCallbacks, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_app_src_set_callbacks() }
    //}

    #[doc(alias = "gst_app_src_set_caps")]
    pub fn set_caps(&self, caps: Option<&gst::Caps>) {
        unsafe {
            ffi::gst_app_src_set_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "gst_app_src_set_duration")]
    pub fn set_duration(&self, duration: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::gst_app_src_set_duration(self.to_glib_none().0, duration.into().into_glib());
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_src_set_leaky_type")]
    pub fn set_leaky_type(&self, leaky: AppLeakyType) {
        unsafe {
            ffi::gst_app_src_set_leaky_type(self.to_glib_none().0, leaky.into_glib());
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_src_set_max_buffers")]
    pub fn set_max_buffers(&self, max: u64) {
        unsafe {
            ffi::gst_app_src_set_max_buffers(self.to_glib_none().0, max);
        }
    }

    #[doc(alias = "gst_app_src_set_max_bytes")]
    pub fn set_max_bytes(&self, max: u64) {
        unsafe {
            ffi::gst_app_src_set_max_bytes(self.to_glib_none().0, max);
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_src_set_max_time")]
    pub fn set_max_time(&self, max: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::gst_app_src_set_max_time(self.to_glib_none().0, max.into().into_glib());
        }
    }

    #[doc(alias = "gst_app_src_set_size")]
    pub fn set_size(&self, size: i64) {
        unsafe {
            ffi::gst_app_src_set_size(self.to_glib_none().0, size);
        }
    }

    #[doc(alias = "gst_app_src_set_stream_type")]
    pub fn set_stream_type(&self, type_: AppStreamType) {
        unsafe {
            ffi::gst_app_src_set_stream_type(self.to_glib_none().0, type_.into_glib());
        }
    }

    pub fn is_block(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"block\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `block` getter")
        }
    }

    pub fn set_block(&self, block: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"block\0".as_ptr() as *const _,
                block.to_value().to_glib_none().0,
            );
        }
    }

    pub fn format(&self) -> gst::Format {
        unsafe {
            let mut value = glib::Value::from_type(<gst::Format as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"format\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `format` getter")
        }
    }

    pub fn set_format(&self, format: gst::Format) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"format\0".as_ptr() as *const _,
                format.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "handle-segment-change")]
    pub fn is_handle_segment_change(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"handle-segment-change\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `handle-segment-change` getter")
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "handle-segment-change")]
    pub fn set_handle_segment_change(&self, handle_segment_change: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"handle-segment-change\0".as_ptr() as *const _,
                handle_segment_change.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "is-live")]
    pub fn is_live(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"is-live\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `is-live` getter")
        }
    }

    #[doc(alias = "is-live")]
    pub fn set_is_live(&self, is_live: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"is-live\0".as_ptr() as *const _,
                is_live.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "max-latency")]
    pub fn max_latency(&self) -> i64 {
        unsafe {
            let mut value = glib::Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-latency\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-latency` getter")
        }
    }

    #[doc(alias = "max-latency")]
    pub fn set_max_latency(&self, max_latency: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"max-latency\0".as_ptr() as *const _,
                max_latency.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "min-latency")]
    pub fn min_latency(&self) -> i64 {
        unsafe {
            let mut value = glib::Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-latency\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-latency` getter")
        }
    }

    #[doc(alias = "min-latency")]
    pub fn set_min_latency(&self, min_latency: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-latency\0".as_ptr() as *const _,
                min_latency.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "min-percent")]
    pub fn min_percent(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-percent\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-percent` getter")
        }
    }

    #[doc(alias = "min-percent")]
    pub fn set_min_percent(&self, min_percent: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"min-percent\0".as_ptr() as *const _,
                min_percent.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "block")]
    pub fn connect_block_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_block_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::block\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_block_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "caps")]
    pub fn connect_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "current-level-buffers")]
    pub fn connect_current_level_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_level_buffers_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-level-buffers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_current_level_buffers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "current-level-bytes")]
    pub fn connect_current_level_bytes_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_level_bytes_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-level-bytes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_current_level_bytes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "current-level-time")]
    pub fn connect_current_level_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_level_time_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-level-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_current_level_time_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "duration")]
    pub fn connect_duration_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "format")]
    pub fn connect_format_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_format_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::format\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_format_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "handle-segment-change")]
    pub fn connect_handle_segment_change_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_handle_segment_change_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::handle-segment-change\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_handle_segment_change_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-live")]
    pub fn connect_is_live_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_live_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-live\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_live_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "leaky-type")]
    pub fn connect_leaky_type_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_leaky_type_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::leaky-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_leaky_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "max-buffers")]
    pub fn connect_max_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_buffers_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-buffers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_buffers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-bytes")]
    pub fn connect_max_bytes_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_bytes_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-bytes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_bytes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-latency")]
    pub fn connect_max_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_latency_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_latency_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "max-time")]
    pub fn connect_max_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_time_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_time_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-latency")]
    pub fn connect_min_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_latency_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_latency_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-percent")]
    pub fn connect_min_percent_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_percent_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-percent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_percent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "size")]
    pub fn connect_size_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<F: Fn(&AppSrc) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stream-type")]
    pub fn connect_stream_type_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stream_type_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stream-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stream_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for AppSrc {}
unsafe impl Sync for AppSrc {}
