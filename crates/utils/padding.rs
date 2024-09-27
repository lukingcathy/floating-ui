use crate::PartialSideLength;

/// 填充的方式
#[derive(Clone, Debug)]
pub enum Padding {
    /// 全部填充
    All(f64),
    /// 指定方位
    PerSide(PartialSideLength),
}
