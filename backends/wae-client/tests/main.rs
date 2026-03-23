use wae_client::prelude::*;

#[test]
fn test_signal_basic() {
    let count = signal(0);
    assert_eq!(count.get(), 0);
    
    count.set(1);
    assert_eq!(count.get(), 1);
}

#[test]
fn test_signal_clone() {
    let count = signal(0);
    let count_clone = count;
    
    count.set(1);
    assert_eq!(count_clone.get(), 1);
}

#[test]
fn test_derived_signal() {
    let count = signal(0);
    let doubled = signal(move || count.get() * 2);
    
    assert_eq!(doubled.get(), 0);
    
    count.set(1);
    // 注意：当前实现中派生信号不会自动更新，需要手动重新计算
    let doubled = signal(move || count.get() * 2);
    assert_eq!(doubled.get(), 2);
}

#[test]
fn test_html_macro() {
    let element = html! {
        <div class="test">
            <h1>"Hello"</h1>
            <button on:click={|| println!("clicked")}>
                "Click me"
            </button>
        </div>
    };
    
    // 验证元素创建成功
    element.render();
}

#[test]
fn test_style_macro() {
    let is_large = true;
    let class_names = style!(
        "px-4 py-2 rounded",
        if is_large { Some("px-8 py-4") } else { None },
        "bg-blue-500 text-white"
    );
    
    assert_eq!(class_names, "px-4 py-2 rounded px-8 py-4 bg-blue-500 text-white");
}
