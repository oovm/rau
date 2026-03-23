/// WAE 前端客户端库
/// 
/// 支持 Web、桌面和移动平台的前端渲染库，提供信号系统、组件定义、事件处理等功能。
pub mod signal;
pub mod renderer;
pub mod element;
pub mod macros;

/// 导出核心功能
pub use signal::{signal, Signal};
pub use renderer::Renderer;
pub use element::Element;

/// 启动应用
/// 
/// 根据启用的 feature 自动选择渲染器并启动应用
/// 
/// # 示例
/// ```rust
/// use wae_client::prelude::*;
/// 
/// #[component]
/// fn App() -> Element {
///     html! {
///         <div>Hello World!</div>
///     }
/// }
/// 
/// fn main() {
///     wae_client::start(App);
/// }
/// ```
pub fn start<F>(root_component: F)
where
    F: Fn() -> Element + 'static,
{
    #[cfg(feature = "web")]
    {
        use wasm_bindgen::prelude::*;
        
        #[wasm_bindgen(start)]
        fn wasm_start() {
            // 这里将实现 Web 平台的启动逻辑
        }
    }
    
    #[cfg(feature = "desktop")]
    {
        // 这里将实现桌面平台的启动逻辑
    }
    
    #[cfg(feature = "mobile")]
    {
        // 这里将实现移动平台的启动逻辑
    }
}

/// 预导入模块
pub mod prelude {
    pub use super::{signal, Signal, Renderer, Element, start};
}
