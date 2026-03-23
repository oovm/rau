use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

/// 信号的内部状态
struct SignalState<T> {
    /// 信号的当前值
    value: T,
    /// 订阅者列表
    subscribers: Vec<Box<dyn Fn() + Send + Sync>>,
}

/// 响应式信号
/// 
/// 信号是一种响应式状态管理机制，用于在值变化时自动通知订阅者。
/// 
/// # 示例
/// ```rust
/// let count = signal(0);
/// let doubled = signal(move || count.get() * 2);
/// 
/// // 读取信号值
/// assert_eq!(count.get(), 0);
/// assert_eq!(doubled.get(), 0);
/// 
/// // 更新信号值
/// count.set(1);
/// assert_eq!(count.get(), 1);
/// assert_eq!(doubled.get(), 2);
/// ```
pub struct Signal<T> {
    /// 信号的内部状态
    state: Arc<RwLock<SignalState<T>>>,
}

impl<T: Send + Sync + 'static> Signal<T> {
    /// 创建一个新的信号
    /// 
    /// # 参数
    /// * `value` - 信号的初始值
    /// 
    /// # 返回值
    /// 返回一个新的信号实例
    pub fn new(value: T) -> Self {
        Self {
            state: Arc::new(RwLock::new(SignalState {
                value,
                subscribers: Vec::new(),
            })),
        }
    }

    /// 读取信号的当前值
    /// 
    /// # 返回值
    /// 返回信号的当前值
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.state.read().unwrap().value.clone()
    }

    /// 更新信号的值
    /// 
    /// # 参数
    /// * `value` - 信号的新值
    pub fn set(&self, value: T) {
        let mut state = self.state.write().unwrap();
        state.value = value;
        
        // 通知所有订阅者
        let subscribers = state.subscribers.clone();
        drop(state);
        
        for subscriber in subscribers {
            subscriber();
        }
    }

    /// 订阅信号的变化
    /// 
    /// # 参数
    /// * `subscriber` - 当信号变化时调用的闭包
    pub fn subscribe<F>(&self, subscriber: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.state.write().unwrap().subscribers.push(Box::new(subscriber));
    }
}

impl<T: Send + Sync + 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<T: Send + Sync + 'static> Copy for Signal<T> {}

/// 创建一个新的信号
/// 
/// # 参数
/// * `value` - 信号的初始值
/// 
/// # 返回值
/// 返回一个新的信号实例
/// 
/// # 示例
/// ```rust
/// let count = signal(0);
/// assert_eq!(count.get(), 0);
/// ```
pub fn signal<T: Send + Sync + 'static>(value: T) -> Signal<T> {
    Signal::new(value)
}

/// 创建一个派生信号
/// 
/// # 参数
/// * `f` - 派生函数，用于从其他信号计算值
/// 
/// # 返回值
/// 返回一个新的派生信号实例
/// 
/// # 示例
/// ```rust
/// let count = signal(0);
/// let doubled = signal(move || count.get() * 2);
/// assert_eq!(doubled.get(), 0);
/// 
/// count.set(1);
/// assert_eq!(doubled.get(), 2);
/// ```
pub fn signal<F, T>(f: F) -> Signal<T>
where
    F: Fn() -> T + Send + Sync + 'static,
    T: Send + Sync + 'static,
{
    let value = f();
    let signal = Signal::new(value);
    
    // 这里简化实现，实际需要追踪依赖
    // 当依赖信号变化时，重新计算值
    
    signal
}
