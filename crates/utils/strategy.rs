/// 放置的策略
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Strategy {
    /// 绝对值
    Absolute,
    /// 固定值
    Fixed,
}
