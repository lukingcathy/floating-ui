//! Rust port of [Floating UI](https://floating-ui.com/).
//!
//! Utility functions shared across Floating UI crates. You may use these functions in your own projects, but are subject to breaking changes.
//!
//! See [@floating-ui/utils](https://www.npmjs.com/package/@floating-ui/utils) for the original package.

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

#[cfg(feature = "dom")]
pub mod dom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alignment {
    Start,
    End,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    pub fn opposite(&self) -> Side {
        use Side::{Bottom, Left, Right, Top};
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
        }
    }

    pub fn axis(&self) -> Axis {
        use Side::{Bottom, Left, Right, Top};
        match self {
            Top | Bottom => Axis::Y,
            Right | Left => Axis::X,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AlignedPlacement {
    TopStart,
    TopEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
    LeftStart,
    LeftEnd,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Placement {
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

impl Placement {
    pub fn alignment(&self) -> Option<Alignment> {
        use Placement::{
            Bottom, BottomEnd, BottomStart, Left, LeftEnd, LeftStart, Right, RightEnd, RightStart,
            Top, TopEnd, TopStart,
        };
        match self {
            Top | Bottom | Left | Right => None,
            TopStart | BottomStart | LeftStart | RightStart => Some(Alignment::Start),
            TopEnd | BottomEnd | LeftEnd | RightEnd => Some(Alignment::End),
        }
    }

    pub fn side(&self) -> Side {
        use Placement::{
            Bottom, BottomEnd, BottomStart, Left, LeftEnd, LeftStart, Right, RightEnd, RightStart,
            Top, TopEnd, TopStart,
        };
        match self {
            Top | TopStart | TopEnd => Side::Top,
            Right | RightStart | RightEnd => Side::Right,
            Bottom | BottomStart | BottomEnd => Side::Bottom,
            Left | LeftStart | LeftEnd => Side::Left,
        }
    }

    pub fn opposite(&self) -> Placement {
        use Placement::{
            Bottom, BottomEnd, BottomStart, Left, LeftEnd, LeftStart, Right, RightEnd, RightStart,
            Top, TopEnd, TopStart,
        };
        match self {
            Top => Bottom,
            TopStart => BottomStart,
            TopEnd => BottomEnd,
            Right => Left,
            RightStart => LeftStart,
            RightEnd => LeftEnd,
            Bottom => Top,
            BottomStart => TopStart,
            BottomEnd => TopEnd,
            Left => Right,
            LeftStart => RightStart,
            LeftEnd => RightEnd,
        }
    }

    pub fn opposite_alignment(&self) -> Placement {
        use Placement::{
            Bottom, BottomEnd, BottomStart, Left, LeftEnd, LeftStart, Right, RightEnd, RightStart,
            Top, TopEnd, TopStart,
        };
        match self {
            Top => Top,
            TopStart => TopEnd,
            TopEnd => TopStart,
            Right => Right,
            RightStart => RightEnd,
            RightEnd => RightStart,
            Bottom => Bottom,
            BottomStart => BottomEnd,
            BottomEnd => BottomStart,
            Left => Left,
            LeftStart => LeftEnd,
            LeftEnd => LeftStart,
        }
    }
}

impl From<(Side, Option<Alignment>)> for Placement {
    fn from(value: (Side, Option<Alignment>)) -> Self {
        match value {
            (Side::Top, None) => Placement::Top,
            (Side::Top, Some(Alignment::Start)) => Placement::TopStart,
            (Side::Top, Some(Alignment::End)) => Placement::TopEnd,
            (Side::Right, None) => Placement::Right,
            (Side::Right, Some(Alignment::Start)) => Placement::RightStart,
            (Side::Right, Some(Alignment::End)) => Placement::RightEnd,
            (Side::Bottom, None) => Placement::Bottom,
            (Side::Bottom, Some(Alignment::Start)) => Placement::BottomStart,
            (Side::Bottom, Some(Alignment::End)) => Placement::BottomEnd,
            (Side::Left, None) => Placement::Left,
            (Side::Left, Some(Alignment::Start)) => Placement::LeftStart,
            (Side::Left, Some(Alignment::End)) => Placement::LeftEnd,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Strategy {
    Absolute,
    Fixed,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn opposite(&self) -> Axis {
        use Axis::{X, Y};
        match self {
            X => Y,
            Y => X,
        }
    }

    pub fn length(&self) -> Length {
        use Axis::{X, Y};
        match self {
            X => Length::Width,
            Y => Length::Height,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

impl Coords {
    pub fn new(value: f64) -> Self {
        Self { x: value, y: value }
    }

    pub fn axis(&self, axis: Axis) -> f64 {
        use Axis::{X, Y};
        match axis {
            X => self.x,
            Y => self.y,
        }
    }

    pub fn update_axis<F>(&mut self, axis: Axis, update: F)
    where
        F: Fn(f64) -> f64,
    {
        match axis {
            Axis::X => {
                self.x = update(self.x);
            }
            Axis::Y => {
                self.y = update(self.y);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Length {
    Width,
    Height,
}

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

impl Dimensions {
    pub fn length(&self, length: Length) -> f64 {
        use Length::{Height, Width};
        match length {
            Width => self.width,
            Height => self.height,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SideObject {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl SideObject {
    pub fn side(&self, side: Side) -> f64 {
        use Side::{Bottom, Left, Right, Top};
        match side {
            Top => self.top,
            Right => self.right,
            Bottom => self.bottom,
            Left => self.left,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PartialSideObject {
    pub top: Option<f64>,
    pub right: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn axis(&self, axis: Axis) -> f64 {
        use Axis::{X, Y};
        match axis {
            X => self.x,
            Y => self.y,
        }
    }

    pub fn length(&self, length: Length) -> f64 {
        use Length::{Height, Width};
        match length {
            Width => self.width,
            Height => self.height,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Padding {
    All(f64),
    PerSide(PartialSideObject),
}

#[derive(Clone, Debug)]
pub struct ClientRectObject {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl From<Rect> for ClientRectObject {
    fn from(value: Rect) -> Self {
        ClientRectObject {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
            top: value.y,
            right: value.x + value.width,
            bottom: value.y + value.height,
            left: value.x,
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "dom")] {
        impl ClientRectObject {
            pub fn from_dom_rect_list(value: web_sys::DomRectList) -> Vec<Self> {
                (0..value.length())
                    .filter_map(|i| value.item(i).map(ClientRectObject::from))
                    .collect()
            }
        }

        impl From<web_sys::DomRect> for ClientRectObject {
            fn from(value: web_sys::DomRect) -> Self {
                Self {
                    x: value.x(),
                    y: value.y(),
                    width: value.width(),
                    height: value.height(),
                    top: value.top(),
                    right: value.right(),
                    bottom: value.bottom(),
                    left: value.left(),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ElementRects {
    pub reference: Rect,
    pub floating: Rect,
}

/// Custom positioning reference element.
///
/// See <https://floating-ui.com/docs/virtual-elements> for the original documentation.
pub trait VirtualElement<Element>: DynClone {
    fn get_bounding_client_rect(&self) -> ClientRectObject;

    fn get_client_rects(&self) -> Option<Vec<ClientRectObject>>;

    fn context_element(&self) -> Option<Element>;
}

dyn_clone::clone_trait_object!(<Element> VirtualElement<Element>);

pub trait GetBoundingClientRectCloneable: DynClone {
    fn call(&self) -> ClientRectObject;
}

impl<F> GetBoundingClientRectCloneable for F
where
    F: Fn() -> ClientRectObject + Clone,
{
    fn call(&self) -> ClientRectObject {
        self()
    }
}

dyn_clone::clone_trait_object!(GetBoundingClientRectCloneable);

pub trait GetClientRectsCloneable: DynClone {
    fn call(&self) -> Vec<ClientRectObject>;
}

impl<F> GetClientRectsCloneable for F
where
    F: Fn() -> Vec<ClientRectObject> + Clone,
{
    fn call(&self) -> Vec<ClientRectObject> {
        self()
    }
}

dyn_clone::clone_trait_object!(GetClientRectsCloneable);

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

// impl<Element: Clone> Clone for DefaultVirtualElement<Element> {
//     fn clone(&self) -> Self {
//         Self {
//             get_bounding_client_rect: dyn_clone::clone_box(&*self.get_bounding_client_rect),
//             context_element: self.context_element.clone(),
//         }
//     }
// }

impl<Element: Clone> VirtualElement<Element> for DefaultVirtualElement<Element> {
    fn get_bounding_client_rect(&self) -> ClientRectObject {
        (self.get_bounding_client_rect).call()
    }

    fn get_client_rects(&self) -> Option<Vec<ClientRectObject>> {
        self.get_client_rects
            .as_ref()
            .map(|get_client_rects| get_client_rects.call())
    }

    fn context_element(&self) -> Option<Element> {
        self.context_element.clone()
    }
}

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

pub fn expand_padding_object(padding: PartialSideObject) -> SideObject {
    SideObject {
        top: padding.top.unwrap_or(0.0),
        right: padding.right.unwrap_or(0.0),
        bottom: padding.bottom.unwrap_or(0.0),
        left: padding.left.unwrap_or(0.0),
    }
}

pub fn get_padding_object(padding: Padding) -> SideObject {
    match padding {
        Padding::All(padding) => SideObject {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
        },
        Padding::PerSide(padding) => expand_padding_object(padding),
    }
}

pub fn rect_to_client_rect(rect: Rect) -> ClientRectObject {
    ClientRectObject {
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
