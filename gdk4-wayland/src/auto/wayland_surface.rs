// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gtk4-gir-files
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    pub struct WaylandSurface(Object<ffi::GdkWaylandSurface>) @extends gdk::Surface;

    match fn {
        get_type => || ffi::gdk_wayland_surface_get_type(),
    }
}

impl WaylandSurface {}

pub const NONE_WAYLAND_SURFACE: Option<&WaylandSurface> = None;

impl fmt::Display for WaylandSurface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WaylandSurface")
    }
}
