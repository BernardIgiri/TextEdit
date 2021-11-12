use gtk::prelude::*;
use glib::object::Object;

pub fn get_object<T: IsA<Object>>(builder: &gtk::Builder, name: &str) -> T {
    let error_message = format!("Failed to find: {}.", &name);
    builder
        .get_object(name)
        .expect(&error_message)
}
