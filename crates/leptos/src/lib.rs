pub use arrow::*;
#[doc(no_inline)]
pub use floating_ui_dom::{
    auto_update, compute_position, dom, AlignedPlacement, Alignment, ApplyState, ArrowData,
    AutoPlacement, AutoPlacementData, AutoPlacementDataOverflow, AutoPlacementOptions,
    AutoUpdateOptions, Axis, Boundary, ClientRect, ComputePositionConfig, ComputePositionReturn,
    Coords, DefaultLimiter, DefaultVirtualElement, Derivable, DerivableFn, DetectOverflowOptions,
    Dimensions, ElementContext, ElementOrVirtual, ElementRects, FallbackStrategy, Flip, FlipData,
    FlipDataOverflow, FlipOptions, Hide, HideData, HideOptions, HideStrategy, Inline,
    InlineOptions, Length, LimitShift, LimitShiftOffset, LimitShiftOffsetValues, LimitShiftOptions,
    Middleware, MiddlewareData, MiddlewareReturn, MiddlewareState, MiddlewareVec,
    MiddlewareWithOptions, Offset, OffsetData, OffsetOptions, OffsetOptionsValues, Padding,
    Placement, Rect, RootBoundary, Shift, ShiftData, ShiftOptions, Side, Size, SizeOptions,
    Strategy, VirtualElement, ARROW_NAME, AUTO_PLACEMENT_NAME, FLIP_NAME, HIDE_NAME, INLINE_NAME,
    OFFSET_NAME, SHIFT_NAME, SIZE_NAME,
};
pub use types::*;
pub use use_floating::*;

mod arrow;
mod node_ref;
mod types;
mod use_floating;
mod utils;
