use std::marker::PhantomData;

use leptos::html::ElementDescriptor;

use floating_ui_dom::{
    Arrow as CoreArrow, ARROW_NAME, ArrowOptions as CoreArrowOptions, Middleware,
    MiddlewareReturn, MiddlewareState, Padding,
};

use crate::node_ref::NodeRefAsElement;

/// Options for [`Arrow`].
#[derive(Clone)]
pub struct ArrowOptions<Ref, RefEl>
where
    Ref: NodeRefAsElement<RefEl> + Copy + 'static,
    RefEl: ElementDescriptor + Clone + 'static,
{
    /// The arrow element to be positioned.
    pub element: Ref,

    /// The padding between the arrow element and the floating element edges.
    /// Useful when the floating element has rounded corners.
    ///
    /// Defaults to `0` on all sides.
    pub padding: Option<Padding>,

    phantom: PhantomData<RefEl>,
}

impl<Ref, RefEl> ArrowOptions<Ref, RefEl>
where
    Ref: NodeRefAsElement<RefEl> + Copy + 'static,
    RefEl: ElementDescriptor + Clone + 'static,
{
    pub fn new(element: Ref) -> Self {
        ArrowOptions {
            element,
            padding: None,
            phantom: PhantomData,
        }
    }

    /// Set `element` option.
    pub fn element(mut self, value: Ref) -> Self {
        self.element = value;
        self
    }

    /// Set `padding` option.
    pub fn padding(mut self, value: Padding) -> Self {
        self.padding = Some(value);
        self
    }
}

/// Provides data to position an inner element of the floating element so that it appears centered to the reference element.
///
/// See <https://floating-ui.com/docs/arrow> for the original documentation.
#[derive(Clone)]
pub struct Arrow<Ref, RefEl>
where
    Ref: NodeRefAsElement<RefEl> + Copy + 'static,
    RefEl: ElementDescriptor + Clone + 'static,
{
    options: ArrowOptions<Ref, RefEl>,
}

impl<Ref, RefEl> Arrow<Ref, RefEl>
where
    Ref: NodeRefAsElement<RefEl> + Copy + 'static,
    RefEl: ElementDescriptor + Clone + 'static,
{
    pub fn new(options: ArrowOptions<Ref, RefEl>) -> Self {
        Arrow { options }
    }
}

impl<Ref, RefEl> Middleware<web_sys::Element, web_sys::Window> for Arrow<Ref, RefEl>
where
    Ref: NodeRefAsElement<RefEl> + Copy + 'static,
    RefEl: ElementDescriptor + Clone + 'static,
{
    fn name(&self) -> &'static str {
        ARROW_NAME
    }

    fn compute(
        &self,
        state: MiddlewareState<web_sys::Element, web_sys::Window>,
    ) -> MiddlewareReturn {
        let element = self.options.element.get_untracked_as_element();

        if let Some(element) = element {
            let element: &web_sys::Element = &element;

            CoreArrow::new(CoreArrowOptions {
                element: element.clone(),
                padding: self.options.padding.clone(),
            })
            .compute(state)
        } else {
            MiddlewareReturn {
                x: None,
                y: None,
                data: None,
                reset: None,
            }
        }
    }
}
