use web_sys::Element;

use crate::dom::get_computed_style;

pub fn is_rtl(element: &Element) -> bool {
    get_computed_style(element)
        .get_property_value("direction")
        .unwrap_or("".into())
        == "rtl"
}
