// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPResult;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RTSPUrl(Boxed<ffi::GstRTSPUrl>);

    match fn {
        copy => |ptr| ffi::gst_rtsp_url_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_rtsp_url_free(ptr),
        get_type => || ffi::gst_rtsp_url_get_type(),
    }
}

impl RTSPUrl {
    pub fn decode_path_components(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_rtsp_url_decode_path_components(self.to_glib_none().0))
        }
    }

    pub fn get_request_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_url_get_request_uri(self.to_glib_none().0))
        }
    }

    pub fn set_port(&mut self, port: u16) -> RTSPResult {
        unsafe {
            from_glib(ffi::gst_rtsp_url_set_port(self.to_glib_none_mut().0, port))
        }
    }

    pub fn parse(urlstr: &str) -> (RTSPResult, RTSPUrl) {
        assert_initialized_main_thread!();
        unsafe {
            let mut url = ptr::null_mut();
            let ret = from_glib(ffi::gst_rtsp_url_parse(urlstr.to_glib_none().0, &mut url));
            (ret, from_glib_full(url))
        }
    }
}

unsafe impl Send for RTSPUrl {}
