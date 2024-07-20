pub use arrow::*;
#[doc(no_inline)]
pub use floating_ui_dom::{
    AlignedPlacement, Alignment, ApplyState, ARROW_NAME, ArrowData, AUTO_PLACEMENT_NAME, auto_update,
    AutoPlacement, AutoPlacementData, AutoPlacementDataOverflow, AutoPlacementOptions,
    AutoUpdateOptions, Axis, Boundary, ClientRectObject, compute_position,
    ComputePositionConfig, ComputePositionReturn, Coords, DefaultLimiter, DefaultVirtualElement, Derivable,
    DerivableFn, DetectOverflowOptions, Dimensions, dom, ElementContext,
    ElementOrVirtual, ElementRects, FallbackStrategy, Flip, FLIP_NAME, FlipData, FlipDataOverflow, FlipOptions,
    Hide, HIDE_NAME, HideData, HideOptions, HideStrategy, Inline,
    INLINE_NAME, InlineOptions, Length, LimitShift, LimitShiftOffset,
    LimitShiftOffsetValues, LimitShiftOptions, Middleware, MiddlewareData, MiddlewareReturn, MiddlewareState,
    MiddlewareVec, MiddlewareWithOptions, Offset, OFFSET_NAME, OffsetData, OffsetOptions, OffsetOptionsValues, Padding,
    Placement, Rect, RootBoundary, Shift, SHIFT_NAME, ShiftData, ShiftOptions, Side,
    Size, SIZE_NAME, SizeOptions, Strategy, VirtualElement,
};
pub use types::*;
pub use use_floating::*;

mod arrow;
mod node_ref;
mod types;
mod use_floating;
mod utils;
