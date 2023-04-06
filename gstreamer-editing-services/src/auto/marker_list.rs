// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Marker;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use crate::MarkerFlags;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GESMarkerList")]
    pub struct MarkerList(Object<ffi::GESMarkerList, ffi::GESMarkerListClass>);

    match fn {
        type_ => || ffi::ges_marker_list_get_type(),
    }
}

impl MarkerList {
    #[doc(alias = "ges_marker_list_new")]
    pub fn new() -> MarkerList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::ges_marker_list_new()) }
    }

    #[doc(alias = "ges_marker_list_add")]
    pub fn add(&self, position: impl Into<Option<gst::ClockTime>>) -> Marker {
        unsafe {
            from_glib_none(ffi::ges_marker_list_add(
                self.to_glib_none().0,
                position.into().into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_marker_list_get_markers")]
    #[doc(alias = "get_markers")]
    pub fn markers(&self) -> Vec<Marker> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_marker_list_get_markers(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_marker_list_move")]
    #[doc(alias = "move")]
    pub fn move_(&self, marker: &Marker, position: impl Into<Option<gst::ClockTime>>) -> bool {
        unsafe {
            from_glib(ffi::ges_marker_list_move(
                self.to_glib_none().0,
                marker.to_glib_none().0,
                position.into().into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_marker_list_remove")]
    pub fn remove(&self, marker: &Marker) -> bool {
        unsafe {
            from_glib(ffi::ges_marker_list_remove(
                self.to_glib_none().0,
                marker.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_marker_list_size")]
    pub fn size(&self) -> u32 {
        unsafe { ffi::ges_marker_list_size(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn flags(&self) -> MarkerFlags {
        glib::ObjectExt::property(self, "flags")
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn set_flags(&self, flags: MarkerFlags) {
        glib::ObjectExt::set_property(self, "flags", flags)
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "marker-added")]
    pub fn connect_marker_added<F: Fn(&Self, u64, &Marker) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn marker_added_trampoline<F: Fn(&MarkerList, u64, &Marker) + 'static>(
            this: *mut ffi::GESMarkerList,
            position: u64,
            marker: *mut ffi::GESMarker,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), position, &from_glib_borrow(marker))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"marker-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    marker_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "marker-moved")]
    pub fn connect_marker_moved<F: Fn(&Self, u64, u64, &Marker) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn marker_moved_trampoline<
            F: Fn(&MarkerList, u64, u64, &Marker) + 'static,
        >(
            this: *mut ffi::GESMarkerList,
            previous_position: u64,
            new_position: u64,
            marker: *mut ffi::GESMarker,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                previous_position,
                new_position,
                &from_glib_borrow(marker),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"marker-moved\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    marker_moved_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "marker-removed")]
    pub fn connect_marker_removed<F: Fn(&Self, &Marker) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn marker_removed_trampoline<F: Fn(&MarkerList, &Marker) + 'static>(
            this: *mut ffi::GESMarkerList,
            marker: *mut ffi::GESMarker,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(marker))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"marker-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    marker_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "flags")]
    pub fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&MarkerList) + 'static>(
            this: *mut ffi::GESMarkerList,
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
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl Default for MarkerList {
    fn default() -> Self {
        Self::new()
    }
}
