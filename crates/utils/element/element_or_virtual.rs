use crate::{OwnedElementOrVirtual, VirtualElement};

#[derive(Clone)]
pub enum ElementOrVirtual<'a, Element: Clone> {
    Element(&'a Element),
    VirtualElement(Box<dyn VirtualElement<Element>>),
}

impl<'a, Element: Clone> ElementOrVirtual<'a, Element> {
    pub fn resolve(self) -> Option<Element> {
        match self {
            ElementOrVirtual::Element(element) => Some(element.clone()),
            ElementOrVirtual::VirtualElement(virtal_element) => virtal_element.context_element(),
        }
    }
}

impl<'a, Element: Clone> From<&'a Element> for ElementOrVirtual<'a, Element> {
    fn from(value: &'a Element) -> Self {
        ElementOrVirtual::Element(value)
    }
}

impl<'a, Element: Clone> From<Box<dyn VirtualElement<Element>>> for ElementOrVirtual<'a, Element> {
    fn from(value: Box<dyn VirtualElement<Element>>) -> Self {
        ElementOrVirtual::VirtualElement(value)
    }
}

impl<'a, Element: Clone> From<&'a OwnedElementOrVirtual<Element>>
    for ElementOrVirtual<'a, Element>
{
    fn from(value: &'a OwnedElementOrVirtual<Element>) -> Self {
        match value {
            OwnedElementOrVirtual::Element(element) => ElementOrVirtual::Element(element),
            OwnedElementOrVirtual::VirtualElement(virtual_element) => {
                ElementOrVirtual::VirtualElement(virtual_element.clone())
            }
        }
    }
}
