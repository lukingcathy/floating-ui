//! 定位取向

use crate::Axis;
use serde::{Deserialize, Serialize};

/// 对齐，每一个方位有三个对齐位置，默认中间
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alignment {
    /// 距离原点近的位置，对于左右的方位而言，这个就上边的点；对于上下的方位而言，这个就是左边的点
    Start,
    /// 距离原点远的位置，对于左右的方位而言，这个就下边的点；对于上下的方位而言，这个就是右边的点
    End,
}

/// 方位
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    /// 上
    Top,
    /// 右
    Right,
    /// 下
    Bottom,
    /// 左
    Left,
}

impl Side {
    /// 翻转方位
    pub fn opposite(&self) -> Side {
        use Side::{Bottom, Left, Right, Top};
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
        }
    }

    /// 获取每个方位所在的坐标轴
    pub fn axis(&self) -> Axis {
        use Side::{Bottom, Left, Right, Top};
        match self {
            Top | Bottom => Axis::Y,
            Right | Left => Axis::X,
        }
    }
}

/// 和方位结合，允许浮动元素对齐的位置点
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AlignedPlacement {
    /// 上左
    TopStart,
    /// 上右
    TopEnd,
    /// 右上
    RightStart,
    /// 右下
    RightEnd,
    /// 下左
    BottomStart,
    /// 下右
    BottomEnd,
    /// 左上
    LeftStart,
    /// 左下
    LeftEnd,
}

/// 浮动元素放置的位置
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Placement {
    /// 上，默认中间
    Top,
    /// 上左
    TopStart,
    /// 上右
    TopEnd,
    /// 右，默认中间
    Right,
    /// 右上
    RightStart,
    /// 右下
    RightEnd,
    /// 下，默认中间
    Bottom,
    /// 下左
    BottomStart,
    /// 下右
    BottomEnd,
    /// 左，默认中间
    Left,
    /// 左上
    LeftStart,
    /// 左下
    LeftEnd,
}

impl Placement {
    /// 通过浮动元素放置的位置获取对齐
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

    /// 通过浮动元素放置的位置获取方位
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

    /// 根据浮动元素放置的位置获取翻转后的位置
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

    /// 根据浮动元素放置的位置获取翻转的对齐点
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
        use Alignment::{End, Start};
        use Side::{Bottom, Left, Right, Top};
        match value {
            (Top, None) => Placement::Top,
            (Top, Some(Start)) => Placement::TopStart,
            (Top, Some(End)) => Placement::TopEnd,
            (Right, None) => Placement::Right,
            (Right, Some(Start)) => Placement::RightStart,
            (Right, Some(End)) => Placement::RightEnd,
            (Bottom, None) => Placement::Bottom,
            (Bottom, Some(Start)) => Placement::BottomStart,
            (Bottom, Some(End)) => Placement::BottomEnd,
            (Left, None) => Placement::Left,
            (Left, Some(Start)) => Placement::LeftStart,
            (Left, Some(End)) => Placement::LeftEnd,
        }
    }
}
