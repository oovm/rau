/// 路由模块
/// 
/// 提供 HTTP 路由功能，用于处理 API 请求。

/// 路由器
pub struct Router {
    routes: Vec<(String, Box<dyn Fn() -> String>)>,
}

impl Router {
    /// 创建一个新的路由器
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }
    
    /// 添加 GET 路由
    pub fn route<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn() -> String + 'static,
    {
        self.routes.push((path.to_string(), Box::new(handler)));
        self
    }
    
    /// 添加嵌套路由
    pub fn nest<F>(mut self, prefix: &str, handler: F) -> Self
    where
        F: Fn() -> String + 'static,
    {
        self.routes.push((prefix.to_string(), Box::new(handler)));
        self
    }
}
