use crate::Length;
use serde::{Deserialize, Serialize};

/// 坐标轴
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Axis {
    /// X 轴
    X,
    /// Y 轴
    Y,
}

impl Axis {
    /// 通过当前轴获取翻转的轴
    pub fn opposite(&self) -> Axis {
        use Axis::{X, Y};
        match self {
            X => Y,
            Y => X,
        }
    }

    /// 获取长度标识， X 轴应该获取元素的宽度，Y 轴获取元素的高度
    pub fn length(&self) -> Length {
        use Axis::{X, Y};
        match self {
            X => Length::Width,
            Y => Length::Height,
        }
    }
}

/// 元素坐标
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

impl Coords {
    pub fn new(value: f64) -> Self {
        Self { x: value, y: value }
    }

    /// 根据坐标轴获取对应坐标值
    pub fn axis(&self, axis: Axis) -> f64 {
        use Axis::{X, Y};
        match axis {
            X => self.x,
            Y => self.y,
        }
    }

    /// 在指定的坐标轴上进行偏移
    pub fn update_axis<F>(&mut self, axis: Axis, update: F)
    where
        F: Fn(f64) -> f64,
    {
        use Axis::{X, Y};

        match axis {
            X => {
                self.x = update(self.x);
            }
            Y => {
                self.y = update(self.y);
            }
        }
    }
}
