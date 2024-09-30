use crate::{ClientRect, GetBoundingClientRectCloneable, GetClientRectsCloneable, VirtualElement};

#[derive(Clone)]
pub struct DefaultVirtualElement<Element: Clone> {
    pub get_bounding_client_rect: Box<dyn GetBoundingClientRectCloneable>,
    pub get_client_rects: Option<Box<dyn GetClientRectsCloneable>>,
    pub context_element: Option<Element>,
}

impl<Element: Clone> DefaultVirtualElement<Element> {
    pub fn new(get_bounding_client_rect: Box<dyn GetBoundingClientRectCloneable>) -> Self {
        DefaultVirtualElement {
            get_bounding_client_rect,
            get_client_rects: None,
            context_element: None,
        }
    }

    pub fn get_bounding_client_rect(
        mut self,
        get_bounding_client_rect: Box<dyn GetBoundingClientRectCloneable>,
    ) -> Self {
        self.get_bounding_client_rect = get_bounding_client_rect;
        self
    }

    pub fn get_client_rects(mut self, get_client_rects: Box<dyn GetClientRectsCloneable>) -> Self {
        self.get_client_rects = Some(get_client_rects);
        self
    }

    pub fn context_element(mut self, context_element: Element) -> Self {
        self.context_element = Some(context_element);
        self
    }
}

impl<Element: Clone> VirtualElement<Element> for DefaultVirtualElement<Element> {
    fn get_bounding_client_rect(&self) -> ClientRect {
        self.get_bounding_client_rect.call()
    }

    fn get_client_rects(&self) -> Option<Vec<ClientRect>> {
        self.get_client_rects
            .as_ref()
            .map(|get_client_rects| get_client_rects.call())
    }

    fn context_element(&self) -> Option<Element> {
        self.context_element.clone()
    }
}
