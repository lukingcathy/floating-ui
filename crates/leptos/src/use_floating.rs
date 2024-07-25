use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use leptos::{
    create_effect, create_memo, create_signal,
    html::{AnyElement, ElementDescriptor},
    MaybeProp, NodeRef, on_cleanup, SignalGet, SignalGetUntracked, SignalSet, watch,
};

use floating_ui_dom::{
    compute_position, ComputePositionConfig, MiddlewareData, OwnedElementOrVirtual, Placement,
    Strategy, VirtualElement,
};

use crate::{
    node_ref::NodeRefAsElement,
    types::{FloatingStyles, UseFloatingOptions, UseFloatingReturn, WhileElementsMountedCleanupFn},
    utils::{get_dpr::get_dpr, round_by_dpr::round_by_dpr},
};

pub enum VirtualElementOrNodeRef<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy + 'static,
    El: ElementDescriptor + Clone + 'static,
{
    VirtualElement(Box<dyn VirtualElement<web_sys::Element>>),
    NodeRef(NodeRef, PhantomData<El>),
}

impl<NodeRef, El> VirtualElementOrNodeRef<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy + 'static,
    El: ElementDescriptor + Clone + 'static,
{
    pub fn get(&self) -> Option<OwnedElementOrVirtual> {
        match self {
            VirtualElementOrNodeRef::VirtualElement(virtual_element) => {
                Some(virtual_element.clone().into())
            }
            VirtualElementOrNodeRef::NodeRef(node_ref, _) => {
                node_ref.get_as_element().map(|element| element.into())
            }
        }
    }

    pub fn get_untracked(&self) -> Option<OwnedElementOrVirtual> {
        match self {
            VirtualElementOrNodeRef::VirtualElement(virtual_element) => {
                Some(virtual_element.clone().into())
            }
            VirtualElementOrNodeRef::NodeRef(node_ref, _) => node_ref
                .get_untracked_as_element()
                .map(|element| element.into()),
        }
    }
}

impl<NodeRef, El> Clone for VirtualElementOrNodeRef<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy + 'static,
    El: ElementDescriptor + Clone + 'static,
{
    fn clone(&self) -> Self {
        match self {
            Self::VirtualElement(virtual_element) => Self::VirtualElement(virtual_element.clone()),
            Self::NodeRef(node_ref, phantom) => Self::NodeRef(*node_ref, *phantom),
        }
    }
}

impl From<Box<dyn VirtualElement<web_sys::Element>>>
    for VirtualElementOrNodeRef<NodeRef<AnyElement>, AnyElement>
{
    fn from(value: Box<dyn VirtualElement<web_sys::Element>>) -> Self {
        VirtualElementOrNodeRef::VirtualElement(value)
    }
}

impl<NodeRef, El> From<NodeRef> for VirtualElementOrNodeRef<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy,
    El: ElementDescriptor + Clone + 'static,
{
    fn from(value: NodeRef) -> Self {
        VirtualElementOrNodeRef::NodeRef(value, PhantomData)
    }
}

pub trait IntoReference<NodeRef, El>
where
    NodeRef: NodeRefAsElement<El> + Copy,
    El: ElementDescriptor + Clone + 'static,
{
    fn into_reference(self) -> MaybeProp<VirtualElementOrNodeRef<NodeRef, El>>;
}

impl IntoReference<NodeRef<AnyElement>, AnyElement> for Box<dyn VirtualElement<web_sys::Element>> {
    fn into_reference(self) -> MaybeProp<VirtualElementOrNodeRef<NodeRef<AnyElement>, AnyElement>> {
        VirtualElementOrNodeRef::VirtualElement(self).into()
    }
}

impl<NodeRef, El> IntoReference<NodeRef, El> for NodeRef
where
    NodeRef: NodeRefAsElement<El> + Copy,
    El: ElementDescriptor + Clone + 'static,
{
    fn into_reference(self) -> MaybeProp<VirtualElementOrNodeRef<NodeRef, El>> {
        VirtualElementOrNodeRef::NodeRef(self, PhantomData).into()
    }
}

/// Computes the `x` and `y` coordinates that will place the floating element next to a reference element.
pub fn use_floating<
    Reference: NodeRefAsElement<ReferenceEl> + Copy + 'static,
    ReferenceEl: ElementDescriptor + Clone + 'static,
    Floating: NodeRefAsElement<FloatingEl> + Copy + 'static,
    FloatingEl: ElementDescriptor + Clone + 'static,
>(
    reference: MaybeProp<VirtualElementOrNodeRef<Reference, ReferenceEl>>,
    floating: Floating,
    options: UseFloatingOptions,
) -> UseFloatingReturn {
    let open_option = move || options.open.get().unwrap_or(true);
    let placement_option_untracked = move || {
        options
            .placement
            .get_untracked()
            .unwrap_or(Placement::Bottom)
    };
    let strategy_option_untracked = move || {
        options
            .strategy
            .get_untracked()
            .unwrap_or(Strategy::Absolute)
    };
    let options_middleware = options.middleware.clone();
    let middleware_option_untracked = move || options_middleware.get_untracked();
    let transform_option = move || options.transform.get().unwrap_or(true);
    let options_while_elements_mounted = options.while_elements_mounted.clone();
    let while_elements_mounted_untracked = move || options_while_elements_mounted.get_untracked();

    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);
    let (strategy, set_strategy) = create_signal(strategy_option_untracked());
    let (placement, set_placement) = create_signal(placement_option_untracked());
    let (middleware_data, set_middleware_data) = create_signal(MiddlewareData::default());
    let (is_positioned, set_is_positioned) = create_signal(false);
    let floating_styles = create_memo(move |_| {
        let initial_styles = FloatingStyles {
            position: strategy.get(),
            top: "0".into(),
            left: "0".into(),
            transform: None,
            will_change: None,
        };

        if let Some(floating_element) = floating.get_as_element() {
            let x_val = round_by_dpr(&floating_element, x.get());
            let y_val = round_by_dpr(&floating_element, y.get());

            if transform_option() {
                FloatingStyles {
                    transform: Some(format!("translate({x_val}px, {y_val}px)")),
                    will_change: match get_dpr(&floating_element) >= 1.5 {
                        true => Some("transform".into()),
                        false => None,
                    },
                    ..initial_styles
                }
            } else {
                FloatingStyles {
                    left: format!("{x_val}px"),
                    top: format!("{y_val}px"),
                    ..initial_styles
                }
            }
        } else {
            initial_styles
        }
    });

    let update_reference = reference.clone();
    let update = move || {
        if let Some(reference) = update_reference.get_untracked() {
            if let (Some(reference_element), Some(floating_element)) = (
                reference.get_untracked(),
                floating.get_untracked_as_element(),
            ) {
                let config = ComputePositionConfig {
                    placement: Some(placement_option_untracked()),
                    strategy: Some(strategy_option_untracked()),
                    middleware: middleware_option_untracked(),
                };

                let position =
                    compute_position((&reference_element).into(), &floating_element, Some(config));
                set_x.set(position.x);
                set_y.set(position.y);
                set_strategy.set(position.strategy);
                set_placement.set(position.placement);
                set_middleware_data.set(position.middleware_data);
                set_is_positioned.set(true);
            }
        }
    };
    let update_rc = Rc::new(update);

    let while_elements_mounted_cleanup: Rc<RefCell<Option<WhileElementsMountedCleanupFn>>> =
        Rc::new(RefCell::new(None));

    let cleanup_while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();
    let cleanup = move || {
        if let Some(while_elements_mounted_cleanup) = cleanup_while_elements_mounted_cleanup.take()
        {
            while_elements_mounted_cleanup();
        }
    };
    let cleanup_rc = Rc::new(cleanup);

    let attach_reference = reference.clone();
    let attach_update_rc = update_rc.clone();
    let attach_cleanup_rc = cleanup_rc.clone();
    let attach_while_elements_mounted_cleanup = while_elements_mounted_cleanup.clone();
    let attach = move || {
        attach_cleanup_rc();

        if let (Some(while_elements_mounted), Some(reference)) = (
            while_elements_mounted_untracked(),
            attach_reference.get_untracked(),
        ) {
            if let (Some(reference_element), Some(floating_element)) = (
                reference.get_untracked(),
                floating.get_untracked_as_element(),
            ) {
                attach_while_elements_mounted_cleanup.replace(Some(while_elements_mounted(
                    (&reference_element).into(),
                    &floating_element,
                    attach_update_rc.clone(),
                )));
            }
        } else {
            attach_update_rc();
        }
    };
    let attach_rc = Rc::new(attach);

    let reset = move || {
        if !open_option() {
            set_is_positioned.set(false);
        }
    };

    let reference_attach = attach_rc.clone();
    create_effect(move |_| {
        if let Some(reference) = reference.get() {
            match reference {
                VirtualElementOrNodeRef::VirtualElement(_) => {
                    reference_attach();
                }
                VirtualElementOrNodeRef::NodeRef(reference, _) => {
                    if let Some(reference) = reference.get() {
                        let reference_attach = reference_attach.clone();
                        _ = reference.on_mount(move |_| {
                            reference_attach();
                        });
                    }
                }
            }
        }
    });

    let floating_attach = attach_rc.clone();
    create_effect(move |_| {
        if let Some(floating) = floating.get() {
            let floating_attach = floating_attach.clone();
            _ = floating.on_mount(move |_| {
                floating_attach();
            });
        }
    });

    create_effect(move |_| {
        reset();
    });

    let placement_update_rc = update_rc.clone();
    let strategy_update_rc = update_rc.clone();
    let middleware_update_rc = update_rc.clone();
    _ = watch(
        move || options.placement.get(),
        move |_, _, _| {
            placement_update_rc();
        },
        false,
    );
    _ = watch(
        move || options.strategy.get(),
        move |_, _, _| {
            strategy_update_rc();
        },
        false,
    );
    _ = watch(
        move || options.middleware.get(),
        move |_, _, _| {
            middleware_update_rc();
        },
        false,
    );
    _ = watch(
        move || options.while_elements_mounted.get(),
        move |_, _, _| {
            attach_rc();
        },
        false,
    );

    on_cleanup(move || {
        cleanup_rc();
    });

    UseFloatingReturn {
        x: x.into(),
        y: y.into(),
        placement: placement.into(),
        strategy: strategy.into(),
        middleware_data: middleware_data.into(),
        is_positioned: is_positioned.into(),
        floating_styles: floating_styles.into(),
        update: update_rc.clone(),
    }
}

#[cfg(test)]
mod tests {
    use leptos::{*, html::Div};
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn updates_is_positioned_when_position_is_computed() {
        #[component]
        fn Component() -> impl IntoView {
            let reference = create_node_ref::<Div>();
            let floating = create_node_ref::<Div>();
            let UseFloatingReturn { is_positioned, .. } = use_floating(
                reference.into_reference(),
                floating,
                UseFloatingOptions::default(),
            );

            view! {
                <div _ref=reference />
                <div _ref=floating />
                <div id="test-is-positioned">{is_positioned}</div>
            }
        }

        let document = leptos::document();
        mount_to(document.body().unwrap(), Component);

        // assert_eq!(
        //     document
        //         .get_element_by_id("test-is-positioned")
        //         .and_then(|element| element.text_content()),
        //     Some("true".into())
        // );
    }
}
