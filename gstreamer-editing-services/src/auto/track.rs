// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{MetaContainer, Timeline, TrackElement, TrackType};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::ptr;
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GESTrack")]
    pub struct Track(Object<ffi::GESTrack, ffi::GESTrackClass>) @extends gst::Bin, gst::Element, gst::Object, @implements gst::ChildProxy, MetaContainer;

    match fn {
        type_ => || ffi::ges_track_get_type(),
    }
}

impl Track {
    pub const NONE: Option<&'static Track> = None;

    #[doc(alias = "ges_track_new")]
    pub fn new(type_: TrackType, caps: gst::Caps) -> Track {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_track_new(type_.into_glib(), caps.into_glib_ptr())) }
    }
}

pub trait GESTrackExt: 'static {
    #[doc(alias = "ges_track_add_element")]
    fn add_element(&self, object: &impl IsA<TrackElement>) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_add_element_full")]
    fn add_element_full(&self, object: &impl IsA<TrackElement>) -> Result<(), glib::Error>;

    #[doc(alias = "ges_track_commit")]
    fn commit(&self) -> bool;

    #[doc(alias = "ges_track_get_caps")]
    #[doc(alias = "get_caps")]
    fn caps(&self) -> Option<gst::Caps>;

    #[doc(alias = "ges_track_get_elements")]
    #[doc(alias = "get_elements")]
    fn elements(&self) -> Vec<TrackElement>;

    #[doc(alias = "ges_track_get_mixing")]
    #[doc(alias = "get_mixing")]
    fn is_mixing(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_get_restriction_caps")]
    #[doc(alias = "get_restriction_caps")]
    fn restriction_caps(&self) -> Option<gst::Caps>;

    #[doc(alias = "ges_track_get_timeline")]
    #[doc(alias = "get_timeline")]
    fn timeline(&self) -> Option<Timeline>;

    #[doc(alias = "ges_track_remove_element")]
    fn remove_element(&self, object: &impl IsA<TrackElement>)
        -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_remove_element_full")]
    fn remove_element_full(&self, object: &impl IsA<TrackElement>) -> Result<(), glib::Error>;

    //#[doc(alias = "ges_track_set_create_element_for_gap_func")]
    //fn set_create_element_for_gap_func<P: Fn() -> gst::Element + 'static>(&self, func: P);

    #[doc(alias = "ges_track_set_mixing")]
    fn set_mixing(&self, mixing: bool);

    #[doc(alias = "ges_track_set_restriction_caps")]
    fn set_restriction_caps(&self, caps: &gst::Caps);

    #[doc(alias = "ges_track_set_timeline")]
    fn set_timeline(&self, timeline: &impl IsA<Timeline>);

    #[doc(alias = "ges_track_update_restriction_caps")]
    fn update_restriction_caps(&self, caps: &gst::Caps);

    fn duration(&self) -> u64;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn id(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_id(&self, id: Option<&str>);

    #[doc(alias = "restriction-caps")]
    fn get_property_restriction_caps(&self) -> Option<gst::Caps>;

    #[doc(alias = "track-type")]
    fn track_type(&self) -> TrackType;

    #[doc(alias = "commited")]
    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "track-element-added")]
    fn connect_track_element_added<F: Fn(&Self, &TrackElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "track-element-removed")]
    fn connect_track_element_removed<F: Fn(&Self, &TrackElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "duration")]
    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "mixing")]
    fn connect_mixing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "restriction-caps")]
    fn connect_restriction_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Track>> GESTrackExt for O {
    fn add_element(&self, object: &impl IsA<TrackElement>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_track_add_element(
                    self.as_ref().to_glib_none().0,
                    object.as_ref().to_glib_none().0
                ),
                "Failed to add element"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn add_element_full(&self, object: &impl IsA<TrackElement>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ges_track_add_element_full(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn commit(&self) -> bool {
        unsafe { from_glib(ffi::ges_track_commit(self.as_ref().to_glib_none().0)) }
    }

    fn caps(&self) -> Option<gst::Caps> {
        unsafe { from_glib_none(ffi::ges_track_get_caps(self.as_ref().to_glib_none().0)) }
    }

    fn elements(&self) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_track_get_elements(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_mixing(&self) -> bool {
        unsafe { from_glib(ffi::ges_track_get_mixing(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn restriction_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::ges_track_get_restriction_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn timeline(&self) -> Option<Timeline> {
        unsafe { from_glib_none(ffi::ges_track_get_timeline(self.as_ref().to_glib_none().0)) }
    }

    fn remove_element(
        &self,
        object: &impl IsA<TrackElement>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_track_remove_element(
                    self.as_ref().to_glib_none().0,
                    object.as_ref().to_glib_none().0
                ),
                "Failed to remove element"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn remove_element_full(&self, object: &impl IsA<TrackElement>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ges_track_remove_element_full(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn set_create_element_for_gap_func<P: Fn() -> gst::Element + 'static>(&self, func: P) {
    //    unsafe { TODO: call ffi:ges_track_set_create_element_for_gap_func() }
    //}

    fn set_mixing(&self, mixing: bool) {
        unsafe {
            ffi::ges_track_set_mixing(self.as_ref().to_glib_none().0, mixing.into_glib());
        }
    }

    fn set_restriction_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::ges_track_set_restriction_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            );
        }
    }

    fn set_timeline(&self, timeline: &impl IsA<Timeline>) {
        unsafe {
            ffi::ges_track_set_timeline(
                self.as_ref().to_glib_none().0,
                timeline.as_ref().to_glib_none().0,
            );
        }
    }

    fn update_restriction_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::ges_track_update_restriction_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            );
        }
    }

    fn duration(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "duration")
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn id(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "id")
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_id(&self, id: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "id", id)
    }

    fn get_property_restriction_caps(&self) -> Option<gst::Caps> {
        glib::ObjectExt::property(self.as_ref(), "restriction-caps")
    }

    fn track_type(&self) -> TrackType {
        glib::ObjectExt::property(self.as_ref(), "track-type")
    }

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn commited_trampoline<P: IsA<Track>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"commited\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    commited_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_track_element_added<F: Fn(&Self, &TrackElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn track_element_added_trampoline<
            P: IsA<Track>,
            F: Fn(&P, &TrackElement) + 'static,
        >(
            this: *mut ffi::GESTrack,
            effect: *mut ffi::GESTrackElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Track::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(effect),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"track-element-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    track_element_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_track_element_removed<F: Fn(&Self, &TrackElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn track_element_removed_trampoline<
            P: IsA<Track>,
            F: Fn(&P, &TrackElement) + 'static,
        >(
            this: *mut ffi::GESTrack,
            effect: *mut ffi::GESTrackElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Track::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(effect),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"track-element-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    track_element_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<P: IsA<Track>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<Track>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_mixing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mixing_trampoline<P: IsA<Track>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mixing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mixing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_restriction_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_restriction_caps_trampoline<
            P: IsA<Track>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTrack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::restriction-caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_restriction_caps_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
