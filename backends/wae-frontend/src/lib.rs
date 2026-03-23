/// 信号系统的核心模块
/// 
/// 提供响应式状态管理，包括信号的创建、读取和更新。
pub mod signal;

/// 导出信号系统的核心功能
pub use signal::{signal, Signal};
