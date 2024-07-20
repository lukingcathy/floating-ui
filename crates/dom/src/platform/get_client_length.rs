use web_sys::Element;

use floating_ui_utils::Length;

pub fn get_client_length(element: &Element, length: Length) -> f64 {
    match length {
        Length::Width => element.client_width() as f64,
        Length::Height => element.client_height() as f64,
    }
}
