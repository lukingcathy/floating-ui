use convert_case::{Case, Casing};
use leptos::{*, html::Div};

use floating_ui_leptos::{
    IntoReference, Strategy, use_floating, UseFloatingOptions, UseFloatingReturn,
};

use crate::utils::use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Node {
    ReferenceScrollParent,
    FloatingScrollParent,
    SameScrollParent,
    Body,
}

const ALL_NODES: [Node; 4] = [
    Node::ReferenceScrollParent,
    Node::FloatingScrollParent,
    Node::SameScrollParent,
    Node::Body,
];
const ALL_STRATEGIES: [Strategy; 2] = [Strategy::Absolute, Strategy::Fixed];

#[component]
pub fn Scroll() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (strategy, set_strategy) = create_signal(Strategy::Absolute);
    let (node, set_node) = create_signal(Node::ReferenceScrollParent);

    let UseFloatingReturn { x, y, update, .. } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default().strategy(strategy.into()),
    );

    let strategy_update = update.clone();
    let node_update = update.clone();

    let UseScrollReturn {
        scroll_ref,
        indicator,
        ..
    } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None::<bool>.into(),
        disable_ref_updates: Some(true),
    });

    let reference_view = move || {
        view! {
            <div
                _ref=reference_ref
                class="reference"
                style=move || match node() {
                    Node::FloatingScrollParent => "position: relative; top: -350px;",
                    _ => ""
                }
            >
                Reference
            </div>
        }
    };

    let floating_view = move || {
        view! {
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
    };

    view! {
        <h1>Scroll</h1>
        <p>
            The floating element should be positioned correctly when a certain node has been scrolled.
        </p>
        <div class="container">
            <Show
                when=move || node() != Node::Body
                fallback=move || view! {
                    {reference_view}
                    {floating_view}
                }
            >
                <div
                    _ref=scroll_ref
                    class="scroll"
                    style:position=move || match node() {
                        Node::FloatingScrollParent | Node::SameScrollParent => "relative",
                        _ => "",
                    }
                >
                    {indicator.clone()}
                    <Show when=move || node() != Node::FloatingScrollParent>
                        {reference_view}
                    </Show>
                    {floating_view}
                </div>
                <Show when=move || node() == Node::FloatingScrollParent>
                    {reference_view}
                </Show>
            </Show>
        </div>

        <h3>Strategy</h3>
        <div class="controls">
            <For
                each=|| ALL_STRATEGIES
                key=|local_strategy| format!("{:?}", local_strategy)
                children=move |local_strategy| {
                    let strategy_update = strategy_update.clone();

                    view! {
                        <button
                            data-testid=move || format!("Strategy{:?}", local_strategy).to_case(Case::Kebab)
                            style:background-color=move || match strategy() == local_strategy {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_strategy(local_strategy);
                                strategy_update();
                            }
                        >
                            {format!("{:?}", local_strategy).to_case(Case::Kebab)}
                        </button>
                    }
                }
            />
        </div>

        <h3>Node</h3>
        <div class="controls">
            <For
                each=|| ALL_NODES
                key=|local_node| format!("{:?}", local_node)
                children=move |local_node| {
                    let node_update = node_update.clone();

                    view! {
                        <button
                            data-testid=move || format!("scroll-{}", format!("{:?}", local_node).to_case(Case::Camel))
                            style:background-color=move || match node() == local_node {
                                true => "black",
                                false => ""
                            }
                            on:click=move |_| {
                                set_node(local_node);
                                node_update();
                            }
                        >
                            {format!("{:?}", local_node).to_case(Case::Camel)}
                        </button>
                    }
                }
            />
        </div>

        <Show when=move || node() == Node::Body>
            <div style:width="1px" style:height="1500px" />
        </Show>
    }
}
