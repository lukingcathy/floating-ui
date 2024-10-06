use web_sys::Element;

use crate::dom::get_computed_style;

pub fn is_static_positioned(element: &Element) -> bool {
    get_computed_style(element)
        .get_property_value("position")
        .expect("Computed style should have position.")
        == "static"
}
