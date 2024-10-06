use web_sys::Element;

use crate::dom::get_document_element;
use floating_ui_utils::{Rect, Strategy};

pub fn get_viewport_rect(element: &Element, _strategy: Strategy) -> Rect {
    // let window = get_window(Some(element));
    let html = get_document_element(Some(element.into()));
    // TODO
    // let visual_viewport = window.visual_viewport;

    let x = 0.0;
    let y = 0.0;
    let width = html.client_width() as f64;
    let height = html.client_height() as f64;

    // TODO: leptos-test-visual viewport

    Rect {
        x,
        y,
        width,
        height,
    }
}
