// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;

glib::wrapper! {
    pub struct PlayerVideoRenderer(Interface<ffi::GstPlayerVideoRenderer, ffi::GstPlayerVideoRendererInterface>);

    match fn {
        get_type => || ffi::gst_player_video_renderer_get_type(),
    }
}

unsafe impl Send for PlayerVideoRenderer {}
unsafe impl Sync for PlayerVideoRenderer {}

pub const NONE_PLAYER_VIDEO_RENDERER: Option<&PlayerVideoRenderer> = None;

pub trait PlayerVideoRendererExt: 'static {}

impl<O: IsA<PlayerVideoRenderer>> PlayerVideoRendererExt for O {}
