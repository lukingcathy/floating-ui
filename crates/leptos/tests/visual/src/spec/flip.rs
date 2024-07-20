use convert_case::{Case, Casing};
use leptos::{*, html::Div};

use floating_ui_leptos::{
    FallbackStrategy, Flip, FlipOptions, IntoReference, MiddlewareVec, Placement, Shift,
    ShiftOptions, use_floating, UseFloatingOptions, UseFloatingReturn,
};

use crate::utils::{
    all_placements::ALL_PLACEMENTS,
    use_scroll::{use_scroll, UseScrollOptions, UseScrollReturn},
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum FallbackPlacements {
    None,
    Empty,
    All,
}

#[component]
pub fn Flip() -> impl IntoView {
    let reference_ref = create_node_ref::<Div>();
    let floating_ref = create_node_ref::<Div>();

    let (placement, set_placement) = create_signal(Placement::Bottom);
    let (main_axis, set_main_axis) = create_signal(true);
    let (cross_axis, set_cross_axis) = create_signal(true);
    let (fallback_placements, set_fallback_placements) = create_signal(FallbackPlacements::None);
    let (fallback_strategy, set_fallback_strategy) = create_signal(FallbackStrategy::BestFit);
    let (flip_alignment, set_flip_alignment) = create_signal(true);
    let (add_shift, set_add_shift) = create_signal(false);

    let UseFloatingReturn {
        x,
        y,
        strategy,
        update,
        ..
    } = use_floating(
        reference_ref.into_reference(),
        floating_ref,
        UseFloatingOptions::default()
            .placement(placement.into())
            .while_elements_mounted_auto_update()
            .middleware(MaybeProp::derive(move || {
                let mut options = FlipOptions::default()
                    .main_axis(main_axis())
                    .cross_axis(cross_axis())
                    .fallback_strategy(fallback_strategy())
                    .flip_alignment(flip_alignment());

                options = match add_shift() {
                    true => options.fallback_placements(vec![Placement::Bottom]),
                    false => match fallback_placements() {
                        FallbackPlacements::None => options,
                        FallbackPlacements::Empty => options.fallback_placements(vec![]),
                        FallbackPlacements::All => {
                            options.fallback_placements(ALL_PLACEMENTS.into())
                        }
                    },
                };

                let mut middleware: MiddlewareVec = vec![Box::new(Flip::new(options))];

                if add_shift() {
                    middleware.push(Box::new(Shift::new(ShiftOptions::default())));
                }

                Some(middleware)
            })),
    );

    let UseScrollReturn {
        scroll_ref,
        indicator,
        ..
    } = use_scroll(UseScrollOptions {
        reference_ref,
        floating_ref,
        update,
        rtl: None::<bool>.into(),
        disable_ref_updates: None,
    });

    view! {
        <h1>Flip</h1>
        <p></p>
        <div class="container">
            <div _ref=scroll_ref class="scroll" data-x="" style:position="relative">
                {indicator}
                <div _ref=reference_ref class="reference">
                    Reference
                </div>
                <div
                    _ref=floating_ref
                    class="floating"
                    style:position=move || format!("{:?}", strategy()).to_lowercase()
                    style:top=move || format!("{}px", y())
                    style:left=move || format!("{}px", x())
                    style:width=move || match add_shift() {
                        true => "400px",
                        false => ""
                    }
                >
                    Floating
                </div>
            </div>
        </div>

        <h2>placement</h2>
        <div class="controls">
            <For
                each=|| ALL_PLACEMENTS
                key=|local_placement| format!("{:?}", local_placement)
                children=move |local_placement| view! {
                    <button
                        data-testid=format!("Placement{:?}", local_placement).to_case(Case::Kebab)
                        style:background-color=move || match placement() == local_placement {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_placement(local_placement)
                    >
                        {format!("{:?}", local_placement).to_case(Case::Kebab)}
                    </button>
                }
            />
        </div>

        <h2>mainAxis</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("mainAxis-{}", value)
                        style:background-color=move || match main_axis() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_main_axis(value)
                    >
                        {format!("{}", value)}
                    </button>
                }
            />
        </div>

        <h2>crossAxis</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("crossAxis-{}", value)
                        style:background-color=move || match cross_axis() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_cross_axis(value)
                    >
                        {format!("{}", value)}
                    </button>
                }
            />
        </div>

        <h2>fallbackPlacements</h2>
        <div class="controls">
            <For
                each=|| [FallbackPlacements::None, FallbackPlacements::Empty, FallbackPlacements::All]
                key=|value| format!("{:?}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("fallbackPlacements-{}", match value {
                            FallbackPlacements::None => "undefined",
                            FallbackPlacements::Empty => "[]",
                            FallbackPlacements::All => "all",
                        })
                        style:background-color=move || match fallback_placements() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_fallback_placements(value)
                    >
                        {match value {
                            FallbackPlacements::None => "undefined".into(),
                            FallbackPlacements::Empty => "[]".into(),
                            FallbackPlacements::All => format!("[{}]", ALL_PLACEMENTS.map(|p| format!("{:?}", p).to_case(Case::Kebab)).join(", ")),
                        }}
                    </button>
                }
            />
        </div>

        <h2>fallbackStrategy</h2>
        <div class="controls">
            <For
                each=|| [FallbackStrategy::BestFit, FallbackStrategy::InitialPlacement]
                key=|local_fallback_strategy| format!("{:?}", local_fallback_strategy)
                children=move |local_fallback_strategy| view! {
                    <button
                        data-testid=format!("fallbackStrategy-{}", format!("{:?}", local_fallback_strategy).to_case(Case::Camel))
                        style:background-color=move || match fallback_strategy() == local_fallback_strategy {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_fallback_strategy(local_fallback_strategy)
                    >
                        {format!("{:?}", local_fallback_strategy).to_case(Case::Camel)}
                    </button>
                }
            />
        </div>

        <h2>flipAlignment</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("flipAlignment-{}", value)
                        style:background-color=move || match flip_alignment() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_flip_alignment(value)
                    >
                        {format!("{}", value)}
                    </button>
                }
            />
        </div>

        <h2>Add shift</h2>
        <div class="controls">
            <For
                each=|| [true, false]
                key=|value| format!("{}", value)
                children=move |value| view! {
                    <button
                        data-testid=format!("shift-{}", value)
                        style:background-color=move || match add_shift() == value {
                            true => "black",
                            false => ""
                        }
                        on:click=move |_| set_add_shift(value)
                    >
                        {format!("{}", value)}
                    </button>
                }
            />
        </div>
    }
}
