/// 静态文件服务模块
/// 
/// 提供静态资源服务功能，用于服务前端构建的 wasm 和 JS 胶水代码。

/// 提供静态文件服务
/// 
/// # 参数
/// * `dir` - 静态文件目录
/// 
/// # 返回值
/// 返回一个处理静态文件请求的处理器
pub fn serve(dir: &str) -> impl Fn() -> String {
    move || {
        format!("Serving static files from {}", dir)
    }
}
