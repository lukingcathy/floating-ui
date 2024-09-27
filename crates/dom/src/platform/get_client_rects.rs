use floating_ui_utils::ClientRect;

use crate::types::ElementOrVirtual;

pub fn get_client_rects(element: ElementOrVirtual) -> Vec<ClientRect> {
    match element {
        ElementOrVirtual::Element(element) => {
            ClientRect::from_dom_rect_list(element.get_client_rects())
        }
        ElementOrVirtual::VirtualElement(virtual_element) => virtual_element
            .get_client_rects()
            .expect("Virtual element must implement `get_client_rects`."),
    }
}
