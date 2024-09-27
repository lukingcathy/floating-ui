use crate::{Axis, Length};

/// 矩形，将元素抽象称为一个矩形
#[derive(Clone, Debug)]
pub struct Rect {
    /// X 坐标值
    pub x: f64,
    /// Y 坐标值
    pub y: f64,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
}

impl Rect {
    /// 通过指定的坐标轴，获取对应坐标值
    pub fn axis(&self, axis: Axis) -> f64 {
        use Axis::{X, Y};
        match axis {
            X => self.x,
            Y => self.y,
        }
    }

    /// 通过指定长度标识，获取元素对应边的长度
    pub fn length(&self, length: Length) -> f64 {
        use Length::{Height, Width};
        match length {
            Width => self.width,
            Height => self.height,
        }
    }
}

/// 元素对应的各个点的值
#[derive(Clone, Debug)]
pub struct ClientRect {
    /// X 坐标值
    pub x: f64,
    /// Y 坐标值
    pub y: f64,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
    /// 上
    pub top: f64,
    /// 右
    pub right: f64,
    /// 下
    pub bottom: f64,
    /// 左
    pub left: f64,
}

impl From<Rect> for ClientRect {
    fn from(value: Rect) -> Self {
        ClientRect {
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
        impl ClientRect {
            pub fn from_dom_rect_list(value: web_sys::DomRectList) -> Vec<Self> {
                (0..value.length())
                    .filter_map(|i| value.item(i).map(ClientRect::from))
                    .collect()
            }
        }

        impl From<web_sys::DomRect> for ClientRect {
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

/// 参与浮动运算的元素对象
#[derive(Clone, Debug)]
pub struct ElementRects {
    /// 引用元素
    pub reference: Rect,
    /// 浮动元素
    pub floating: Rect,
}
