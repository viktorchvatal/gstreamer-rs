// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DiscovererStreamInfo;
use glib::translate::*;

glib::glib_wrapper! {
    pub struct DiscovererSubtitleInfo(Object<ffi::GstDiscovererSubtitleInfo>) @extends DiscovererStreamInfo;

    match fn {
        get_type => || ffi::gst_discoverer_subtitle_info_get_type(),
    }
}

impl DiscovererSubtitleInfo {
    pub fn get_language(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_subtitle_info_get_language(
                self.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for DiscovererSubtitleInfo {}
unsafe impl Sync for DiscovererSubtitleInfo {}
