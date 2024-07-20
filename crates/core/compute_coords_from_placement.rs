use floating_ui_utils::{
    get_alignment, get_alignment_axis, get_axis_length, get_side, get_side_axis, Alignment, Axis,
    Coords, ElementRects, Placement, Side,
};

/// Computes the `x` and `y` coordinates that will place the floating element next to a given reference element based on a `placement`.
pub fn compute_coords_from_placement(
    ElementRects {
        reference,
        floating,
    }: &ElementRects,
    placement: Placement,
    rtl: Option<bool>,
) -> Coords {
    let side_axis = get_side_axis(placement);
    let alignment_axis = get_alignment_axis(placement);
    let align_length = get_axis_length(alignment_axis);
    let side = get_side(placement);
    let is_vertical = side_axis == Axis::Y;

    let common_x = reference.x + reference.width / 2.0 - floating.width / 2.0;
    let common_y = reference.y + reference.height / 2.0 - floating.height / 2.0;
    let common_align = reference.length(align_length) / 2.0 - floating.length(align_length) / 2.0;

    let mut coords = match side {
        Side::Top => Coords {
            x: common_x,
            y: reference.y - floating.height,
        },
        Side::Right => Coords {
            x: reference.x + reference.width,
            y: common_y,
        },
        Side::Bottom => Coords {
            x: common_x,
            y: reference.y + reference.height,
        },
        Side::Left => Coords {
            x: reference.x - floating.width,
            y: common_y,
        },
    };

    let rtl = rtl.unwrap_or(false);
    match get_alignment(placement) {
        Some(Alignment::Start) => {
            coords.update_axis(alignment_axis, |value| {
                value - common_align * (if rtl && is_vertical { -1.0 } else { 1.0 })
            });
        }
        Some(Alignment::End) => {
            coords.update_axis(alignment_axis, |value| {
                value + common_align * (if rtl && is_vertical { -1.0 } else { 1.0 })
            });
        }
        None => {}
    }

    coords
}

#[cfg(test)]
mod tests {}
