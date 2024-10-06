use leptos::{html::Div, *};

use floating_ui_leptos::{
    client_rect::to_client_rect, use_floating, DefaultVirtualElement, Strategy, UseFloatingOptions,
    UseFloatingReturn, VirtualElement,
};

use crate::utils::use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn};

#[component]
pub fn VirtualElement() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();
    let virtual_element = MaybeProp::derive(move || {
        let context_element = reference_ref.get();
        context_element.map(|context_element| {
            let context_element_clone = context_element.clone();
            let element: &web_sys::Element = context_element.as_ref();
            (Box::new(
                DefaultVirtualElement::new(Box::new(move || {
                    let dom_rect = context_element_clone.get_bounding_client_rect();
                    to_client_rect(dom_rect)
                }))
                .context_element(element.clone()),
            ) as Box<dyn VirtualElement<web_sys::Element>>)
                .into()
        })
    });

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        virtual_element,
        floating_ref,
        UseFloatingOptions::default()
            .strategy(Strategy::Fixed.into())
            .while_elements_mounted_auto_update(),
    );

    let UseScrollReturn { scroll_ref, .. } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None::<bool>.into(),
        disable_ref_updates: None,
    });

    view! {
        <h1>Virtual Element</h1>
        <p></p>
        <div class="container">
            <div _ref=scroll_ref class="scroll" data-x="" style:position="relative">
                <div _ref=reference_ref class="reference">
                    Reference
                </div>
            </div>
        </div>

        <div
            _ref=floating_ref
            class="floating"
            style:position=move || format!("{:?}", strategy()).to_lowercase()
            style:top=move || format!("{}px", y())
            style:left=move || format!("{}px", x())
        >
            Floating
        </div>
    }
}
