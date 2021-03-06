// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use pango_sys;
use AttrClass;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Attribute(Boxed<pango_sys::PangoAttribute>);

    match fn {
        copy => |ptr| pango_sys::pango_attribute_copy(mut_override(ptr)),
        free => |ptr| pango_sys::pango_attribute_destroy(ptr),
    }
}

impl Attribute {
    fn equal(&self, attr2: &Attribute) -> bool {
        unsafe {
            from_glib(pango_sys::pango_attribute_equal(
                self.to_glib_none().0,
                attr2.to_glib_none().0,
            ))
        }
    }

    pub fn init(&mut self, klass: &AttrClass) {
        unsafe {
            pango_sys::pango_attribute_init(self.to_glib_none_mut().0, klass.to_glib_none().0);
        }
    }
}

impl PartialEq for Attribute {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Attribute {}
