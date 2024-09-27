//! 元素边长

use crate::Side;
use serde::{Deserialize, Serialize};

/// 元素长度标识
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Length {
    /// 宽，矩形的长度
    Width,
    /// 高，矩形的宽度
    Height,
}

/// 维度，宽度和长度的具体数值
#[derive(Clone, Debug)]
pub struct Dimensions {
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
}

impl Dimensions {
    /// 根据元素长度标识获取对应的维度值
    pub fn length(&self, length: Length) -> f64 {
        use Length::{Height, Width};
        match length {
            Width => self.width,
            Height => self.height,
        }
    }
}

/// 元素各边长度
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SideLength {
    /// 上
    pub top: f64,
    /// 右
    pub right: f64,
    /// 下
    pub bottom: f64,
    /// 左
    pub left: f64,
}

impl SideLength {
    /// 根据指定的边获取对应的长度
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

/// 部分元素的各边长度
#[derive(Clone, Debug)]
pub struct PartialSideLength {
    pub top: Option<f64>,
    pub right: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,
}
