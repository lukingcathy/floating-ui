//! Rust port of [Floating UI](https://floating-ui.com/).
//!
//! Utility functions shared across Floating UI crates. You may use these functions in your own projects, but are subject to breaking changes.
//!
//! See [@floating-ui/utils](https://www.npmjs.com/package/@floating-ui/utils) for the original package.

pub use crate::coordinate::*;
pub use crate::element::*;
pub use crate::length::*;
pub use crate::orientation::*;
pub use crate::padding::*;
pub use crate::rect::*;
pub use crate::strategy::*;
use dyn_clone::DynClone;

mod coordinate;
#[cfg(feature = "dom")]
pub mod dom;
mod element;
mod length;
mod orientation;
mod padding;
mod rect;
mod strategy;

/// Custom positioning reference element.
///
/// See <https://floating-ui.com/docs/virtual-elements> for the original documentation.
pub trait VirtualElement<Element>: DynClone {
    fn get_bounding_client_rect(&self) -> ClientRect;

    fn get_client_rects(&self) -> Option<Vec<ClientRect>>;

    fn context_element(&self) -> Option<Element>;
}

dyn_clone::clone_trait_object!(<Element> VirtualElement<Element>);

pub trait GetBoundingClientRectCloneable: DynClone {
    fn call(&self) -> ClientRect;
}

impl<F> GetBoundingClientRectCloneable for F
where
    F: Fn() -> ClientRect + Clone,
{
    fn call(&self) -> ClientRect {
        self()
    }
}

dyn_clone::clone_trait_object!(GetBoundingClientRectCloneable);

pub trait GetClientRectsCloneable: DynClone {
    fn call(&self) -> Vec<ClientRect>;
}

impl<F> GetClientRectsCloneable for F
where
    F: Fn() -> Vec<ClientRect> + Clone,
{
    fn call(&self) -> Vec<ClientRect> {
        self()
    }
}

dyn_clone::clone_trait_object!(GetClientRectsCloneable);

pub const ALL_PLACEMENTS: [Placement; 12] = [
    Placement::Top,
    Placement::TopStart,
    Placement::TopEnd,
    Placement::Right,
    Placement::RightStart,
    Placement::RightEnd,
    Placement::Bottom,
    Placement::BottomStart,
    Placement::BottomEnd,
    Placement::Left,
    Placement::LeftStart,
    Placement::LeftEnd,
];

pub const ALL_SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];

pub fn clamp(start: f64, value: f64, end: f64) -> f64 {
    value.min(end).max(start)
}

pub fn get_side(placement: Placement) -> Side {
    placement.side()
}

pub fn get_alignment(placement: Placement) -> Option<Alignment> {
    placement.alignment()
}

pub fn get_placement(side: Side, alignment: Option<Alignment>) -> Placement {
    (side, alignment).into()
}

pub fn get_opposite_axis(axis: Axis) -> Axis {
    axis.opposite()
}

pub fn get_axis_length(axis: Axis) -> Length {
    axis.length()
}

pub fn get_side_axis(placement: Placement) -> Axis {
    placement.side().axis()
}

pub fn get_alignment_axis(placement: Placement) -> Axis {
    get_opposite_axis(get_side_axis(placement))
}

pub fn get_alignment_sides(
    placement: Placement,
    rects: &ElementRects,
    rtl: Option<bool>,
) -> (Side, Side) {
    let alignment = get_alignment(placement);
    let alignment_axis = get_alignment_axis(placement);
    let length = get_axis_length(alignment_axis);

    let mut main_alignment_side = match (alignment_axis, alignment) {
        (Axis::X, Some(Alignment::Start)) => match rtl {
            Some(true) => Side::Left,
            _ => Side::Right,
        },
        (Axis::X, _) => match rtl {
            Some(true) => Side::Right,
            _ => Side::Left,
        },
        (Axis::Y, Some(Alignment::Start)) => Side::Bottom,
        (Axis::Y, _) => Side::Top,
    };

    if rects.reference.length(length) > rects.floating.length(length) {
        main_alignment_side = get_opposite_side(main_alignment_side);
    }

    (main_alignment_side, get_opposite_side(main_alignment_side))
}

pub fn get_expanded_placements(placement: Placement) -> Vec<Placement> {
    let opposite_placement = get_opposite_placement(placement);

    vec![
        get_opposite_alignment_placement(placement),
        opposite_placement,
        get_opposite_alignment_placement(opposite_placement),
    ]
}

pub fn get_opposite_alignment_placement(placement: Placement) -> Placement {
    placement.opposite_alignment()
}

pub fn get_side_list(side: Side, is_start: bool, rtl: Option<bool>) -> Vec<Side> {
    match side {
        Side::Top | Side::Bottom => match rtl {
            Some(true) => match is_start {
                true => vec![Side::Right, Side::Left],
                false => vec![Side::Left, Side::Right],
            },
            _ => match is_start {
                true => vec![Side::Left, Side::Right],
                false => vec![Side::Right, Side::Left],
            },
        },
        Side::Right | Side::Left => match is_start {
            true => vec![Side::Top, Side::Bottom],
            false => vec![Side::Bottom, Side::Top],
        },
    }
}

pub fn get_opposite_side(side: Side) -> Side {
    side.opposite()
}

pub fn get_opposite_axis_placements(
    placement: Placement,
    flip_alignment: bool,
    direction: Option<Alignment>,
    rtl: Option<bool>,
) -> Vec<Placement> {
    let alignment = get_alignment(placement);
    let side_list = get_side_list(
        get_side(placement),
        direction.is_some_and(|d| d == Alignment::Start),
        rtl,
    );

    let mut list: Vec<Placement> = side_list
        .into_iter()
        .map(|side| get_placement(side, alignment))
        .collect();

    if flip_alignment {
        let mut opposite_list: Vec<Placement> = list
            .clone()
            .into_iter()
            .map(get_opposite_alignment_placement)
            .collect();

        list.append(&mut opposite_list);
    }

    list
}

pub fn get_opposite_placement(placement: Placement) -> Placement {
    placement.opposite()
}

pub fn expand_padding_object(padding: PartialSideLength) -> SideLength {
    SideLength {
        top: padding.top.unwrap_or(0.0),
        right: padding.right.unwrap_or(0.0),
        bottom: padding.bottom.unwrap_or(0.0),
        left: padding.left.unwrap_or(0.0),
    }
}

pub fn get_padding_object(padding: Padding) -> SideLength {
    match padding {
        Padding::All(padding) => SideLength {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
        },
        Padding::PerSide(padding) => expand_padding_object(padding),
    }
}

pub fn rect_to_client_rect(rect: Rect) -> ClientRect {
    ClientRect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        top: rect.y,
        right: rect.x + rect.width,
        bottom: rect.y + rect.height,
        left: rect.x,
    }
}
