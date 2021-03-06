// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct PlayerVisualization(Boxed<ffi::GstPlayerVisualization>);

    match fn {
        copy => |ptr| ffi::gst_player_visualization_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_player_visualization_free(ptr),
        get_type => || ffi::gst_player_visualization_get_type(),
    }
}

unsafe impl Send for PlayerVisualization {}
unsafe impl Sync for PlayerVisualization {}
