// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    #[doc(alias = "GstVideoBufferFlags")]
    pub struct VideoBufferFlags: u32 {
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_INTERLACED")]
        const INTERLACED = ffi::GST_VIDEO_BUFFER_FLAG_INTERLACED as u32;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_TFF")]
        const TFF = ffi::GST_VIDEO_BUFFER_FLAG_TFF as u32;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_RFF")]
        const RFF = ffi::GST_VIDEO_BUFFER_FLAG_RFF as u32;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_ONEFIELD")]
        const ONEFIELD = ffi::GST_VIDEO_BUFFER_FLAG_ONEFIELD as u32;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_MULTIPLE_VIEW")]
        const MULTIPLE_VIEW = ffi::GST_VIDEO_BUFFER_FLAG_MULTIPLE_VIEW as u32;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_FIRST_IN_BUNDLE")]
        const FIRST_IN_BUNDLE = ffi::GST_VIDEO_BUFFER_FLAG_FIRST_IN_BUNDLE as u32;
        #[cfg(any(feature = "v1_16", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_TOP_FIELD")]
        const TOP_FIELD = ffi::GST_VIDEO_BUFFER_FLAG_TOP_FIELD as u32;
        #[cfg(any(feature = "v1_16", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_BOTTOM_FIELD")]
        const BOTTOM_FIELD = ffi::GST_VIDEO_BUFFER_FLAG_BOTTOM_FIELD as u32;
        #[cfg(any(feature = "v1_18", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_MARKER")]
        const MARKER = ffi::GST_VIDEO_BUFFER_FLAG_MARKER as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoBufferFlags {
    type GlibType = ffi::GstVideoBufferFlags;

    fn into_glib(self) -> ffi::GstVideoBufferFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoBufferFlags> for VideoBufferFlags {
    unsafe fn from_glib(value: ffi::GstVideoBufferFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoBufferFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_buffer_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoBufferFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoBufferFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoBufferFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstVideoChromaSite")]
    pub struct VideoChromaSite: u32 {
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_NONE")]
        const NONE = ffi::GST_VIDEO_CHROMA_SITE_NONE as u32;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_H_COSITED")]
        const H_COSITED = ffi::GST_VIDEO_CHROMA_SITE_H_COSITED as u32;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_V_COSITED")]
        const V_COSITED = ffi::GST_VIDEO_CHROMA_SITE_V_COSITED as u32;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_ALT_LINE")]
        const ALT_LINE = ffi::GST_VIDEO_CHROMA_SITE_ALT_LINE as u32;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_COSITED")]
        const COSITED = ffi::GST_VIDEO_CHROMA_SITE_COSITED as u32;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_JPEG")]
        const JPEG = ffi::GST_VIDEO_CHROMA_SITE_JPEG as u32;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_MPEG2")]
        const MPEG2 = ffi::GST_VIDEO_CHROMA_SITE_MPEG2 as u32;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_DV")]
        const DV = ffi::GST_VIDEO_CHROMA_SITE_DV as u32;
    }
}

impl VideoChromaSite {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_chroma_site_from_string")]
    pub fn from_string(s: &str) -> VideoChromaSite {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_video_chroma_site_from_string(s.to_glib_none().0)) }
    }
}

impl fmt::Display for VideoChromaSite {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

#[doc(hidden)]
impl IntoGlib for VideoChromaSite {
    type GlibType = ffi::GstVideoChromaSite;

    fn into_glib(self) -> ffi::GstVideoChromaSite {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoChromaSite> for VideoChromaSite {
    unsafe fn from_glib(value: ffi::GstVideoChromaSite) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoChromaSite {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_chroma_site_get_type()) }
    }
}

impl glib::value::ValueType for VideoChromaSite {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoChromaSite {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoChromaSite {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstVideoCodecFrameFlags")]
    pub struct VideoCodecFrameFlags: u32 {
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_DECODE_ONLY")]
        const DECODE_ONLY = ffi::GST_VIDEO_CODEC_FRAME_FLAG_DECODE_ONLY as u32;
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_SYNC_POINT")]
        const SYNC_POINT = ffi::GST_VIDEO_CODEC_FRAME_FLAG_SYNC_POINT as u32;
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME")]
        const FORCE_KEYFRAME = ffi::GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME as u32;
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME_HEADERS")]
        const FORCE_KEYFRAME_HEADERS = ffi::GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME_HEADERS as u32;
        #[cfg(any(feature = "v1_20", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_CORRUPTED")]
        const CORRUPTED = ffi::GST_VIDEO_CODEC_FRAME_FLAG_CORRUPTED as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoCodecFrameFlags {
    type GlibType = ffi::GstVideoCodecFrameFlags;

    fn into_glib(self) -> ffi::GstVideoCodecFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoCodecFrameFlags> for VideoCodecFrameFlags {
    unsafe fn from_glib(value: ffi::GstVideoCodecFrameFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl StaticType for VideoCodecFrameFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_codec_frame_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for VideoCodecFrameFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl<'a> FromValue<'a> for VideoCodecFrameFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl ToValue for VideoCodecFrameFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
bitflags! {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "GstVideoDecoderRequestSyncPointFlags")]
    pub struct VideoDecoderRequestSyncPointFlags: u32 {
        #[doc(alias = "GST_VIDEO_DECODER_REQUEST_SYNC_POINT_DISCARD_INPUT")]
        const DISCARD_INPUT = ffi::GST_VIDEO_DECODER_REQUEST_SYNC_POINT_DISCARD_INPUT as u32;
        #[doc(alias = "GST_VIDEO_DECODER_REQUEST_SYNC_POINT_CORRUPT_OUTPUT")]
        const CORRUPT_OUTPUT = ffi::GST_VIDEO_DECODER_REQUEST_SYNC_POINT_CORRUPT_OUTPUT as u32;
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for VideoDecoderRequestSyncPointFlags {
    type GlibType = ffi::GstVideoDecoderRequestSyncPointFlags;

    fn into_glib(self) -> ffi::GstVideoDecoderRequestSyncPointFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstVideoDecoderRequestSyncPointFlags> for VideoDecoderRequestSyncPointFlags {
    unsafe fn from_glib(value: ffi::GstVideoDecoderRequestSyncPointFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl StaticType for VideoDecoderRequestSyncPointFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_decoder_request_sync_point_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for VideoDecoderRequestSyncPointFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl<'a> FromValue<'a> for VideoDecoderRequestSyncPointFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl ToValue for VideoDecoderRequestSyncPointFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstVideoFlags")]
    pub struct VideoFlags: u32 {
        #[doc(alias = "GST_VIDEO_FLAG_VARIABLE_FPS")]
        const VARIABLE_FPS = ffi::GST_VIDEO_FLAG_VARIABLE_FPS as u32;
        #[doc(alias = "GST_VIDEO_FLAG_PREMULTIPLIED_ALPHA")]
        const PREMULTIPLIED_ALPHA = ffi::GST_VIDEO_FLAG_PREMULTIPLIED_ALPHA as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoFlags {
    type GlibType = ffi::GstVideoFlags;

    fn into_glib(self) -> ffi::GstVideoFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFlags> for VideoFlags {
    unsafe fn from_glib(value: ffi::GstVideoFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstVideoFormatFlags")]
    pub struct VideoFormatFlags: u32 {
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_YUV")]
        const YUV = ffi::GST_VIDEO_FORMAT_FLAG_YUV as u32;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_RGB")]
        const RGB = ffi::GST_VIDEO_FORMAT_FLAG_RGB as u32;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_GRAY")]
        const GRAY = ffi::GST_VIDEO_FORMAT_FLAG_GRAY as u32;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_ALPHA")]
        const ALPHA = ffi::GST_VIDEO_FORMAT_FLAG_ALPHA as u32;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_LE")]
        const LE = ffi::GST_VIDEO_FORMAT_FLAG_LE as u32;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_PALETTE")]
        const PALETTE = ffi::GST_VIDEO_FORMAT_FLAG_PALETTE as u32;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_COMPLEX")]
        const COMPLEX = ffi::GST_VIDEO_FORMAT_FLAG_COMPLEX as u32;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_UNPACK")]
        const UNPACK = ffi::GST_VIDEO_FORMAT_FLAG_UNPACK as u32;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_TILED")]
        const TILED = ffi::GST_VIDEO_FORMAT_FLAG_TILED as u32;
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_SUBTILES")]
        const SUBTILES = ffi::GST_VIDEO_FORMAT_FLAG_SUBTILES as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoFormatFlags {
    type GlibType = ffi::GstVideoFormatFlags;

    fn into_glib(self) -> ffi::GstVideoFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFormatFlags> for VideoFormatFlags {
    unsafe fn from_glib(value: ffi::GstVideoFormatFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_format_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoFormatFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoFormatFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFormatFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstVideoFrameFlags")]
    pub struct VideoFrameFlags: u32 {
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_INTERLACED")]
        const INTERLACED = ffi::GST_VIDEO_FRAME_FLAG_INTERLACED as u32;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_TFF")]
        const TFF = ffi::GST_VIDEO_FRAME_FLAG_TFF as u32;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_RFF")]
        const RFF = ffi::GST_VIDEO_FRAME_FLAG_RFF as u32;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_ONEFIELD")]
        const ONEFIELD = ffi::GST_VIDEO_FRAME_FLAG_ONEFIELD as u32;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_MULTIPLE_VIEW")]
        const MULTIPLE_VIEW = ffi::GST_VIDEO_FRAME_FLAG_MULTIPLE_VIEW as u32;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_FIRST_IN_BUNDLE")]
        const FIRST_IN_BUNDLE = ffi::GST_VIDEO_FRAME_FLAG_FIRST_IN_BUNDLE as u32;
        #[cfg(any(feature = "v1_16", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_TOP_FIELD")]
        const TOP_FIELD = ffi::GST_VIDEO_FRAME_FLAG_TOP_FIELD as u32;
        #[cfg(any(feature = "v1_16", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_BOTTOM_FIELD")]
        const BOTTOM_FIELD = ffi::GST_VIDEO_FRAME_FLAG_BOTTOM_FIELD as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoFrameFlags {
    type GlibType = ffi::GstVideoFrameFlags;

    fn into_glib(self) -> ffi::GstVideoFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFrameFlags> for VideoFrameFlags {
    unsafe fn from_glib(value: ffi::GstVideoFrameFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoFrameFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_frame_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoFrameFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoFrameFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFrameFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstVideoMultiviewFlags")]
    pub struct VideoMultiviewFlags: u32 {
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_VIEW_FIRST")]
        const RIGHT_VIEW_FIRST = ffi::GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_VIEW_FIRST as u32;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLIPPED")]
        const LEFT_FLIPPED = ffi::GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLIPPED as u32;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLOPPED")]
        const LEFT_FLOPPED = ffi::GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLOPPED as u32;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLIPPED")]
        const RIGHT_FLIPPED = ffi::GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLIPPED as u32;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLOPPED")]
        const RIGHT_FLOPPED = ffi::GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLOPPED as u32;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_HALF_ASPECT")]
        const HALF_ASPECT = ffi::GST_VIDEO_MULTIVIEW_FLAGS_HALF_ASPECT as u32;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_MIXED_MONO")]
        const MIXED_MONO = ffi::GST_VIDEO_MULTIVIEW_FLAGS_MIXED_MONO as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoMultiviewFlags {
    type GlibType = ffi::GstVideoMultiviewFlags;

    fn into_glib(self) -> ffi::GstVideoMultiviewFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoMultiviewFlags> for VideoMultiviewFlags {
    unsafe fn from_glib(value: ffi::GstVideoMultiviewFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoMultiviewFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_multiview_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoMultiviewFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoMultiviewFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoMultiviewFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstVideoOverlayFormatFlags")]
    pub struct VideoOverlayFormatFlags: u32 {
        #[doc(alias = "GST_VIDEO_OVERLAY_FORMAT_FLAG_PREMULTIPLIED_ALPHA")]
        const PREMULTIPLIED_ALPHA = ffi::GST_VIDEO_OVERLAY_FORMAT_FLAG_PREMULTIPLIED_ALPHA as u32;
        #[doc(alias = "GST_VIDEO_OVERLAY_FORMAT_FLAG_GLOBAL_ALPHA")]
        const GLOBAL_ALPHA = ffi::GST_VIDEO_OVERLAY_FORMAT_FLAG_GLOBAL_ALPHA as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoOverlayFormatFlags {
    type GlibType = ffi::GstVideoOverlayFormatFlags;

    fn into_glib(self) -> ffi::GstVideoOverlayFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoOverlayFormatFlags> for VideoOverlayFormatFlags {
    unsafe fn from_glib(value: ffi::GstVideoOverlayFormatFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl StaticType for VideoOverlayFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_overlay_format_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl glib::value::ValueType for VideoOverlayFormatFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl<'a> FromValue<'a> for VideoOverlayFormatFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl ToValue for VideoOverlayFormatFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstVideoPackFlags")]
    pub struct VideoPackFlags: u32 {
        #[doc(alias = "GST_VIDEO_PACK_FLAG_TRUNCATE_RANGE")]
        const TRUNCATE_RANGE = ffi::GST_VIDEO_PACK_FLAG_TRUNCATE_RANGE as u32;
        #[doc(alias = "GST_VIDEO_PACK_FLAG_INTERLACED")]
        const INTERLACED = ffi::GST_VIDEO_PACK_FLAG_INTERLACED as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoPackFlags {
    type GlibType = ffi::GstVideoPackFlags;

    fn into_glib(self) -> ffi::GstVideoPackFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoPackFlags> for VideoPackFlags {
    unsafe fn from_glib(value: ffi::GstVideoPackFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoPackFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_pack_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoPackFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoPackFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoPackFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
bitflags! {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "GstVideoTimeCodeFlags")]
    pub struct VideoTimeCodeFlags: u32 {
        #[doc(alias = "GST_VIDEO_TIME_CODE_FLAGS_DROP_FRAME")]
        const DROP_FRAME = ffi::GST_VIDEO_TIME_CODE_FLAGS_DROP_FRAME as u32;
        #[doc(alias = "GST_VIDEO_TIME_CODE_FLAGS_INTERLACED")]
        const INTERLACED = ffi::GST_VIDEO_TIME_CODE_FLAGS_INTERLACED as u32;
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
#[doc(hidden)]
impl IntoGlib for VideoTimeCodeFlags {
    type GlibType = ffi::GstVideoTimeCodeFlags;

    fn into_glib(self) -> ffi::GstVideoTimeCodeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
#[doc(hidden)]
impl FromGlib<ffi::GstVideoTimeCodeFlags> for VideoTimeCodeFlags {
    unsafe fn from_glib(value: ffi::GstVideoTimeCodeFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl StaticType for VideoTimeCodeFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_time_code_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl glib::value::ValueType for VideoTimeCodeFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl<'a> FromValue<'a> for VideoTimeCodeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl ToValue for VideoTimeCodeFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
