use floating_ui_utils::ClientRect;
use web_sys::{DomRect, DomRectList};

pub fn from_dom_rect_list(value: DomRectList) -> Vec<ClientRect> {
    (0..value.length())
        .filter_map(|i| value.item(i).map(|dom_rect| to_client_rect(dom_rect)))
        .collect()
}

pub fn to_client_rect(value: DomRect) -> ClientRect {
    ClientRect {
        x: value.x(),
        y: value.y(),
        width: value.width(),
        height: value.height(),
        top: value.top(),
        right: value.right(),
        bottom: value.bottom(),
        left: value.left(),
    }
}
