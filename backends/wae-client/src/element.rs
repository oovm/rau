/// 元素模块
/// 
/// 定义 UI 元素的结构和操作。

use super::renderer::ElementTrait;

/// 元素类型
#[derive(Debug, Clone)]
pub enum ElementType {
    /// 文本节点
    Text(String),
    /// 元素节点
    Element {
        /// 标签名
        tag: String,
        /// 属性
        attrs: Vec<(String, String)>,
        /// 子元素
        children: Vec<Element>,
    },
    /// 组件节点
    Component {
        /// 组件名称
        name: String,
        /// 组件属性
        props: Vec<(String, String)>,
        /// 子元素
        children: Vec<Element>,
    },
}

/// 元素
/// 
/// 表示 UI 中的一个元素，可以是文本、普通元素或组件。
pub struct ElementImpl {
    /// 元素类型
    element_type: ElementType,
}

impl ElementImpl {
    /// 创建文本元素
    pub fn text(content: &str) -> Element {
        Box::new(Self {
            element_type: ElementType::Text(content.to_string()),
        })
    }
    
    /// 创建元素节点
    pub fn element(tag: &str, attrs: Vec<(String, String)>, children: Vec<Element>) -> Element {
        Box::new(Self {
            element_type: ElementType::Element {
                tag: tag.to_string(),
                attrs,
                children,
            },
        })
    }
    
    /// 创建组件节点
    pub fn component(name: &str, props: Vec<(String, String)>, children: Vec<Element>) -> Element {
        Box::new(Self {
            element_type: ElementType::Component {
                name: name.to_string(),
                props,
                children,
            },
        })
    }
}

impl ElementTrait for ElementImpl {
    fn render(&self) -> Box<dyn std::any::Any> {
        // 这里将实现元素的渲染逻辑
        Box::new(())
    }
}

/// 元素类型别名
pub type Element = Box<dyn ElementTrait>;
