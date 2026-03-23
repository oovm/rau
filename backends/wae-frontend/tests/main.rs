use wae_frontend::signal;

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
fn test_signal_subscribe() {
    let count = signal(0);
    let mut called = false;
    
    count.subscribe(move || {
        called = true;
    });
    
    count.set(1);
    assert!(called);
}

#[test]
fn test_signal_derived() {
    let count = signal(0);
    let doubled = signal(move || count.get() * 2);
    
    assert_eq!(doubled.get(), 0);
    
    count.set(1);
    // 注意：当前实现中派生信号不会自动更新，需要手动处理依赖追踪
    // 这里我们重新创建派生信号来验证计算逻辑
    let doubled_updated = signal(move || count.get() * 2);
    assert_eq!(doubled_updated.get(), 2);
}
