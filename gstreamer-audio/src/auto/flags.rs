// This file was generated by gir (3294959) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

bitflags! {
    pub struct AudioFlags: u32 {
        const AUDIO_FLAG_NONE = 0;
        const AUDIO_FLAG_UNPOSITIONED = 1;
    }
}

#[doc(hidden)]
impl ToGlib for AudioFlags {
    type GlibType = ffi::GstAudioFlags;

    fn to_glib(&self) -> ffi::GstAudioFlags {
        ffi::GstAudioFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFlags> for AudioFlags {
    fn from_glib(value: ffi::GstAudioFlags) -> AudioFlags {
        skip_assert_initialized!();
        AudioFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for AudioFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstAudioFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for AudioFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct AudioFormatFlags: u32 {
        const AUDIO_FORMAT_FLAG_INTEGER = 1;
        const AUDIO_FORMAT_FLAG_FLOAT = 2;
        const AUDIO_FORMAT_FLAG_SIGNED = 4;
        const AUDIO_FORMAT_FLAG_COMPLEX = 16;
        const AUDIO_FORMAT_FLAG_UNPACK = 32;
    }
}

#[doc(hidden)]
impl ToGlib for AudioFormatFlags {
    type GlibType = ffi::GstAudioFormatFlags;

    fn to_glib(&self) -> ffi::GstAudioFormatFlags {
        ffi::GstAudioFormatFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioFormatFlags> for AudioFormatFlags {
    fn from_glib(value: ffi::GstAudioFormatFlags) -> AudioFormatFlags {
        skip_assert_initialized!();
        AudioFormatFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for AudioFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_format_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioFormatFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioFormatFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstAudioFormatFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for AudioFormatFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct AudioPackFlags: u32 {
        const AUDIO_PACK_FLAG_NONE = 0;
        const AUDIO_PACK_FLAG_TRUNCATE_RANGE = 1;
    }
}

#[doc(hidden)]
impl ToGlib for AudioPackFlags {
    type GlibType = ffi::GstAudioPackFlags;

    fn to_glib(&self) -> ffi::GstAudioPackFlags {
        ffi::GstAudioPackFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioPackFlags> for AudioPackFlags {
    fn from_glib(value: ffi::GstAudioPackFlags) -> AudioPackFlags {
        skip_assert_initialized!();
        AudioPackFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for AudioPackFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_pack_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AudioPackFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AudioPackFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstAudioPackFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for AudioPackFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

