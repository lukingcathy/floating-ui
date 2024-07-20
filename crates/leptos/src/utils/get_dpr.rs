use web_sys::Element;

use floating_ui_dom::dom::get_window;

pub fn get_dpr(element: &Element) -> f64 {
    get_window(Some(element)).device_pixel_ratio()
}
