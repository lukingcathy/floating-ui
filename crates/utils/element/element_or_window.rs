#[derive(Clone, Debug)]
pub enum ElementOrWindow<'a, Element, Window> {
    Element(&'a Element),
    Window(&'a Window),
}

impl<'a, Element, Window> From<&'a OwnedElementOrWindow<Element, Window>>
    for ElementOrWindow<'a, Element, Window>
{
    fn from(value: &'a OwnedElementOrWindow<Element, Window>) -> Self {
        match value {
            OwnedElementOrWindow::Element(element) => ElementOrWindow::Element(element),
            OwnedElementOrWindow::Window(window) => ElementOrWindow::Window(window),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OwnedElementOrWindow<Element, Window> {
    Element(Element),
    Window(Window),
}
