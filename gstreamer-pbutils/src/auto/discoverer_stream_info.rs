// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;

glib::glib_wrapper! {
    pub struct DiscovererStreamInfo(Object<ffi::GstDiscovererStreamInfo>);

    match fn {
        get_type => || ffi::gst_discoverer_stream_info_get_type(),
    }
}

unsafe impl Send for DiscovererStreamInfo {}
unsafe impl Sync for DiscovererStreamInfo {}

pub const NONE_DISCOVERER_STREAM_INFO: Option<&DiscovererStreamInfo> = None;

pub trait DiscovererStreamInfoExt: 'static {
    fn get_caps(&self) -> Option<gst::Caps>;

    fn get_misc(&self) -> Option<gst::Structure>;

    fn get_next(&self) -> Option<DiscovererStreamInfo>;

    fn get_previous(&self) -> Option<DiscovererStreamInfo>;

    fn get_stream_id(&self) -> Option<glib::GString>;

    fn get_stream_type_nick(&self) -> glib::GString;

    fn get_tags(&self) -> Option<gst::TagList>;

    fn get_toc(&self) -> Option<gst::Toc>;
}

impl<O: IsA<DiscovererStreamInfo>> DiscovererStreamInfoExt for O {
    fn get_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_misc(&self) -> Option<gst::Structure> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_misc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_next(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_next(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_previous(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_previous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_stream_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_stream_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_stream_type_nick(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_stream_type_nick(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_tags(&self) -> Option<gst::TagList> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_tags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_toc(&self) -> Option<gst::Toc> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_toc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}
