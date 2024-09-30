use crate::VirtualElement;

#[derive(Clone)]
pub enum OwnedElementOrVirtual<Element> {
    Element(Element),
    VirtualElement(Box<dyn VirtualElement<Element>>),
}

impl<Element> OwnedElementOrVirtual<Element> {
    pub fn resolve(self) -> Option<Element> {
        match self {
            OwnedElementOrVirtual::Element(element) => Some(element),
            OwnedElementOrVirtual::VirtualElement(virtual_element) => {
                virtual_element.context_element()
            }
        }
    }
}

impl<Element> From<Element> for OwnedElementOrVirtual<Element> {
    fn from(value: Element) -> Self {
        OwnedElementOrVirtual::Element(value)
    }
}

impl<Element> From<Box<dyn VirtualElement<Element>>> for OwnedElementOrVirtual<Element> {
    fn from(value: Box<dyn VirtualElement<Element>>) -> Self {
        OwnedElementOrVirtual::VirtualElement(value)
    }
}
