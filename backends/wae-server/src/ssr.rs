/// SSR 模块
/// 
/// 提供服务端渲染功能，将组件渲染为 HTML 字符串。

use wae_client::Element;

/// 渲染组件为 HTML 字符串
/// 
/// # 参数
/// * `component` - 要渲染的组件函数
/// 
/// # 返回值
/// 返回渲染后的 HTML 字符串
pub fn render<F>(component: F) -> String
where
    F: Fn() -> Element,
{
    let element = component();
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WAE App</title>
</head>
<body>
    <div id="app">{}</div>
    <script src="/static/wae_client.js"></script>
</body>
</html>
"#, render_element(&element))
}

/// 渲染元素为 HTML 字符串
fn render_element(element: &Element) -> String {
    // 这里将实现元素的服务端渲染逻辑
    "<div>Server Rendered</div>".to_string()
}
