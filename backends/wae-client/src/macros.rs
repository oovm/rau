/// 宏模块
/// 
/// 提供 `html!` 和 `style!` 等宏。

#[macro_export]
macro_rules! html {
    // 空元素
    ($tag:ident) => {
        $crate::element::ElementImpl::element(stringify!($tag), vec![], vec![])
    };
    
    // 带属性的元素
    ($tag:ident { $($attr:ident = $value:expr),* }) => {
        $crate::element::ElementImpl::element(
            stringify!($tag),
            vec![$((stringify!($attr).to_string(), $value.to_string()),)*],
            vec![]
        )
    };
    
    // 带属性和子元素的元素
    ($tag:ident { $($attr:ident = $value:expr),* } { $($child:expr),* }) => {
        $crate::element::ElementImpl::element(
            stringify!($tag),
            vec![$((stringify!($attr).to_string(), $value.to_string()),)*],
            vec![$($child,)*]
        )
    };
    
    // 带事件绑定的元素
    ($tag:ident { $($attr:ident = $value:expr),* $(, on:$event:ident = $handler:expr)* }) => {
        $crate::element::ElementImpl::element(
            stringify!($tag),
            vec![
                $((stringify!($attr).to_string(), $value.to_string()),)*
                $((format!("on:{}", stringify!($event)), stringify!($handler).to_string()),)*
            ],
            vec!
        )
    };
    
    // 带事件绑定和子元素的元素
    ($tag:ident { $($attr:ident = $value:expr),* $(, on:$event:ident = $handler:expr)* } { $($child:expr),* }) => {
        $crate::element::ElementImpl::element(
            stringify!($tag),
            vec![
                $((stringify!($attr).to_string(), $value.to_string()),)*
                $((format!("on:{}", stringify!($event)), stringify!($handler).to_string()),)*
            ],
            vec![$($child,)*]
        )
    };
    
    // 文本节点
    ($text:expr) => {
        $crate::element::ElementImpl::text(&$text.to_string())
    };
}

#[macro_export]
macro_rules! style {
    ($($class:expr),*) => {
        {
            let mut classes = Vec::new();
            $(if let Some(class) = $class {
                classes.push(class);
            })*
            classes.join(" ")
        }
    };
}

#[macro_export]
macro_rules! component {
    ($($vis:vis)? fn $name:ident($($param:ident: $ty:ty),*) -> $ret:ty $body:block) => {
        $($vis)? fn $name($($param: $ty),*) -> $ret {
            $body
        }
    };
    
    ($($vis:vis)? struct $name:ident {
        $($field:ident: $ty:ty $(, #[default($default:expr)])?)*
    }) => {
        $($vis)? struct $name {
            $($field: $ty)*
        }
        
        impl $name {
            pub fn new($($field: $ty),*) -> Self {
                Self {
                    $($field)*
                }
            }
        }
    };
}
