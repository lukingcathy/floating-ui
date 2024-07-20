use floating_ui_core::compute_coords_from_placement;
use floating_ui_utils::{Coords, ElementRects, Placement, Rect};

const ELEMENT_RECTS: ElementRects = ElementRects {
    reference: Rect {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    },
    floating: Rect {
        x: 0.0,
        y: 0.0,
        width: 50.0,
        height: 50.0,
    },
};

#[test]
fn test_top() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::Top, None),
        Coords { x: 25.0, y: -50.0 }
    )
}

#[test]
fn test_top_start() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::TopStart, None),
        Coords { x: 0.0, y: -50.0 }
    )
}

#[test]
fn test_top_end() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::TopEnd, None),
        Coords { x: 50.0, y: -50.0 }
    )
}

#[test]
fn test_right() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::Right, None),
        Coords { x: 100.0, y: 25.0 }
    )
}

#[test]
fn test_right_start() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::RightStart, None),
        Coords { x: 100.0, y: 0.0 }
    )
}

#[test]
fn test_right_end() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::RightEnd, None),
        Coords { x: 100.0, y: 50.0 }
    )
}

#[test]
fn test_bottom() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::Bottom, None),
        Coords { x: 25.0, y: 100.0 }
    )
}

#[test]
fn test_bottom_start() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::BottomStart, None),
        Coords { x: 0.0, y: 100.0 }
    )
}

#[test]
fn test_bottom_end() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::BottomEnd, None),
        Coords { x: 50.0, y: 100.0 }
    )
}

#[test]
fn test_left() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::Left, None),
        Coords { x: -50.0, y: 25.0 }
    )
}

#[test]
fn test_left_start() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::LeftStart, None),
        Coords { x: -50.0, y: 0.0 }
    )
}

#[test]
fn test_left_end() {
    assert_eq!(
        compute_coords_from_placement(&ELEMENT_RECTS, Placement::LeftEnd, None),
        Coords { x: -50.0, y: 50.0 }
    )
}
