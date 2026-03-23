/// WAE 后端服务库
/// 
/// 提供 HTTP 服务器、SSR 渲染、静态资源服务和 API 路由等功能。
pub mod ssr;
pub mod static_files;
pub mod router;

/// 导出核心功能
pub use ssr::render;
pub use static_files::serve;
pub use router::Router;

/// 预导入模块
pub mod prelude {
    pub use super::{render, serve, Router};
}
