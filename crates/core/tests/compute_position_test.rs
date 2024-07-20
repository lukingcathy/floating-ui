use serde_json::json;

use floating_ui_core::{
    compute_position, ComputePositionConfig, ComputePositionReturn, GetClippingRectArgs,
    GetElementRectsArgs, Middleware, MiddlewareReturn, MiddlewareState, Platform,
};
use floating_ui_utils::{Dimensions, ElementRects, Placement, Rect, Strategy};

#[derive(Clone, Debug)]
pub struct Element {}

#[derive(Clone, Debug)]
pub struct Window {}

pub const REFERENCE: Element = Element {};
pub const FLOATING: Element = Element {};
pub const REFERENCE_RECT: Rect = Rect {
    x: 0.0,
    y: 0.0,
    width: 100.0,
    height: 100.0,
};
pub const FLOATING_RECT: Rect = Rect {
    x: 0.0,
    y: 0.0,
    width: 50.0,
    height: 50.0,
};

#[derive(Debug)]
pub struct TestPlatform {}

impl Platform<Element, Window> for TestPlatform {
    fn get_element_rects(&self, _args: GetElementRectsArgs<Element>) -> ElementRects {
        ElementRects {
            reference: REFERENCE_RECT,
            floating: FLOATING_RECT,
        }
    }

    fn get_clipping_rect(&self, _args: GetClippingRectArgs<Element>) -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width: 1000.0,
            height: 1000.0,
        }
    }

    fn get_dimensions(&self, _element: &Element) -> Dimensions {
        Dimensions {
            width: 10.0,
            height: 10.0,
        }
    }
}

pub const PLATFORM: TestPlatform = TestPlatform {};

#[test]
fn test_returned_data() {
    #[derive(Clone)]
    struct CustomMiddleware {}

    impl<Element: Clone, Window: Clone> Middleware<Element, Window> for CustomMiddleware {
        fn name(&self) -> &'static str {
            "custom"
        }

        fn compute(&self, _state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
            MiddlewareReturn {
                x: None,
                y: None,
                data: Some(json!({"property": true})),
                reset: None,
            }
        }
    }

    let ComputePositionReturn {
        x,
        y,
        placement,
        strategy,
        middleware_data,
    } = compute_position(
        (&REFERENCE).into(),
        &FLOATING,
        ComputePositionConfig {
            platform: &PLATFORM,
            placement: Some(Placement::Top),
            strategy: None,
            middleware: Some(vec![Box::new(CustomMiddleware {})]),
        },
    );

    assert_eq!(x, 25.0);
    assert_eq!(y, -50.0);
    assert_eq!(placement, Placement::Top);
    assert_eq!(strategy, Strategy::Absolute);
    assert_eq!(
        middleware_data.get("custom"),
        Some(&json!({"property": true}))
    );
}

#[test]
fn test_middleware() {
    #[derive(Clone)]
    struct TestMiddleware {}

    impl<Element: Clone, Window: Clone> Middleware<Element, Window> for TestMiddleware {
        fn name(&self) -> &'static str {
            "test"
        }

        fn compute(
            &self,
            MiddlewareState { x, y, .. }: MiddlewareState<Element, Window>,
        ) -> MiddlewareReturn {
            MiddlewareReturn {
                x: Some(x + 1.0),
                y: Some(y + 1.0),
                data: None,
                reset: None,
            }
        }
    }

    let ComputePositionReturn { x, y, .. } = compute_position(
        (&REFERENCE).into(),
        &FLOATING,
        ComputePositionConfig {
            platform: &PLATFORM,
            placement: None,
            strategy: None,
            middleware: None,
        },
    );

    let ComputePositionReturn { x: x2, y: y2, .. } = compute_position(
        (&REFERENCE).into(),
        &FLOATING,
        ComputePositionConfig {
            platform: &PLATFORM,
            placement: None,
            strategy: None,
            middleware: Some(vec![Box::new(TestMiddleware {})]),
        },
    );

    assert_eq!((x2, y2), (x + 1.0, y + 1.0));
}

#[test]
fn test_middleware_data() {
    #[derive(Clone)]
    struct TestMiddleware {}

    impl<Element: Clone, Window: Clone> Middleware<Element, Window> for TestMiddleware {
        fn name(&self) -> &'static str {
            "test"
        }

        fn compute(&self, _state: MiddlewareState<Element, Window>) -> MiddlewareReturn {
            MiddlewareReturn {
                x: None,
                y: None,
                data: Some(json!({"hello": true})),
                reset: None,
            }
        }
    }

    let ComputePositionReturn {
        middleware_data, ..
    } = compute_position(
        (&REFERENCE).into(),
        &FLOATING,
        ComputePositionConfig {
            platform: &PLATFORM,
            placement: None,
            strategy: None,
            middleware: Some(vec![Box::new(TestMiddleware {})]),
        },
    );

    assert_eq!(middleware_data.get("test"), Some(&json!({"hello": true})));
}
