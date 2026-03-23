# 架构概览

WAE 采用模块化的 Crate 设计，严格区分了 `wae-client` 和 `wae-server` 两个独立的 crate，分别负责前端渲染与后端服务，避免混淆。

## 后端架构 (wae-server)

```
backends/
├── wae-server/             # 后端服务入口
│   ├── ssr/                # 服务端渲染
│   ├── static_files/       # 静态资源服务
│   └── router/             # API 路由
│
├── wae-types/              # 基础类型定义
│   ├── WaeError            # 统一错误类型
│   ├── WaeResult           # 结果类型别名
│   └── ErrorCategory       # 错误分类
│
├── wae-https/              # HTTP/HTTPS 服务
│   ├── HttpsServerBuilder  # 服务构建器
│   ├── ApiResponse         # 统一响应结构
│   ├── middleware/         # 中间件 (CORS, 压缩, 追踪)
│   ├── extract/            # 请求提取器
│   ├── router/             # 路由器
│   ├── template/           # 模板引擎
│   └── tls/                # TLS 配置
│
├── wae-authentication/     # 认证服务
│   ├── jwt/                # JWT 认证
│   ├── oauth2/             # OAuth2 认证
│   │   └── providers/      # GitHub, Google, WeChat
│   ├── saml/               # SAML 认证
│   └── totp/               # TOTP 双因素认证
│
├── wae-ai/                 # AI 服务抽象
│   ├── capabilities/       # 能力定义
│   │   ├── chat.rs         # 对话能力
│   │   ├── text_to_image.rs # 文生图
│   │   ├── ai_image.rs     # AI 图像处理
│   │   └── video.rs        # 视频生成
│   └── providers/          # 服务商实现
│       ├── hunyuan.rs      # 腾讯混元
│       └── volcengine.rs   # 火山引擎
│
├── wae-storage/            # 存储服务抽象
│   ├── StorageProvider     # 存储提供者 trait
│   └── providers/
│       ├── cos.rs          # 腾讯云 COS
│       ├── oss.rs          # 阿里云 OSS
│       └── local.rs        # 本地存储
│
├── wae-websocket/          # WebSocket 服务
│   ├── WebSocketServer     # 服务端
│   ├── WebSocketClient     # 客户端
│   ├── ConnectionManager   # 连接管理
│   └── RoomManager         # 房间管理
│
├── wae-service/            # 微服务支持
│   ├── ServiceRegistry     # 服务注册
│   ├── ServiceDiscovery    # 服务发现
│   └── LoadBalancer        # 负载均衡
│
├── wae-config/             # 配置管理
│   └── ConfigLoader        # 配置加载器
│
├── wae-resilience/         # 弹性容错
│   ├── circuit_breaker.rs  # 熔断器
│   ├── rate_limiter.rs     # 限流器
│   ├── retry.rs            # 重试
│   ├── timeout.rs          # 超时
│   ├── bulkhead.rs         # 舱壁隔离
│   └── pipeline.rs         # 弹性管道
│
├── wae-scheduler/          # 任务调度
│   ├── cron.rs             # Cron 调度
│   ├── interval.rs         # 间隔调度
│   └── delayed.rs          # 延迟队列
│
├── wae-session/            # Session 管理
│   ├── Session             # Session 结构
│   ├── SessionStore        # 存储接口
│   └── SessionLayer        # 中间件层
│
├── wae-email/              # 邮件服务
│   ├── smtp.rs             # SMTP 发送
│   ├── sendmail.rs         # Sendmail 发送
│   └── mime.rs             # MIME 处理
│
├── wae-database/           # 数据库 ORM
│   ├── connection.rs       # 连接管理
│   ├── orm.rs              # ORM 实现
│   └── schema.rs           # Schema 定义
│
├── wae-event/              # 事件驱动
├── wae-queue/              # 消息队列
├── wae-distributed/        # 分布式支持
├── wae-testing/            # 测试支持
├── wae-tools/              # 开发工具
├── wae-macros/             # 过程宏
├── wae-request/            # HTTP 客户端
├── wae-effect/             # 副作用管理
├── wae-cache/              # 缓存服务
└── wae-schema/             # Schema 定义
```

## 前端架构 (wae-client)

```
backends/
├── wae-client/             # 前端客户端库
│   ├── signal/             # 信号系统
│   ├── renderer/           # 渲染器抽象
│   ├── element/            # UI 元素
│   └── macros/             # 宏定义 (html!, style!)
│
└── frontends/              # TypeScript 前端库
    ├── wae-core/           # 核心类型定义
    │   ├── error.ts        # CloudError 类型
    │   ├── response.ts     # ApiResponse 类型
    │   ├── billing.ts      # 计费维度
    │   ├── auth.ts         # 认证类型
    │   ├── storage.ts      # 存储类型
    │   └── websocket.ts    # WebSocket 类型
    │
    ├── wae-client/         # HTTP 客户端
    │   └── client.ts       # HttpClient 实现
    │
    ├── wae-auth/           # 认证客户端
    │   └── auth.ts         # AuthClient 实现
    │
    ├── wae-websocket/      # WebSocket 客户端
    │   └── websocket.ts    # WebSocketClient 实现
    │
    ├── wae-storage/        # 存储客户端
    │   └── storage.ts      # StorageClient 实现
    │
    └── wae/                # All-in-one 入口
        └── index.ts        # 重导出所有子模块
```

## 设计原则

### 1. 模块化

每个模块都是独立的 crate，可以单独使用：

```toml
# 只使用 AI 模块
[dependencies]
wae-ai = { path = "backends/wae-ai" }

# 使用后端模块
[dependencies]
wae-server = { path = "backends/wae-server" }

# 使用前端模块
[dependencies]
wae-client = { path = "backends/wae-client", features = ["web"] }
```

### 2. 前后端分离

严格区分 `wae-client` 和 `wae-server` 两个独立的 crate：

- `wae-client`：专注于前端渲染，支持 Web、桌面、移动平台
- `wae-server`：专注于后端服务，提供 HTTP 服务、SSR 渲染、API 路由

### 3. 可扩展

通过 trait 抽象，轻松添加新的服务商和平台支持：

```rust
// 添加新的 AI 服务商
pub struct NewProvider;

impl ChatCapability for NewProvider {
    async fn chat(&self, params: &ChatParams, config: &AiConfig) -> AiResult<String> {
        // 实现调用逻辑
    }
}

// 添加新的平台渲染器
pub struct NewPlatformRenderer;

impl Renderer for NewPlatformRenderer {
    type Node = ();
    
    fn mount(&self, container: &str, element: Element) {
        // 实现挂载逻辑
    }
    
    fn update(&self, node: &Self::Node, element: Element) {
        // 实现更新逻辑
    }
    
    fn run(&self) -> ! {
        // 实现事件循环
    }
}
```

### 4. 类型安全

前后端类型一一对应，确保 API 调用的类型安全：

```rust
// 后端 Rust
pub struct UserInfo {
    pub id: UserId,
    pub username: String,
    pub email: Option<String>,
}
```

```typescript
// 前端 TypeScript
export interface UserInfo {
    id: UserId;
    username: string;
    email: string | null;
}
```

### 5. 异步优先

所有可能涉及 I/O 的操作都是异步的：

```rust
pub trait AuthService: Send + Sync {
    async fn login(&self, credentials: &Credentials) -> WaeResult<AuthToken>;
    async fn logout(&self, token: &str) -> WaeResult<()>;
    async fn refresh_token(&self, refresh_token: &str) -> WaeResult<AuthToken>;
}
```

### 6. 统一错误处理

所有模块使用统一的错误类型：

```rust
pub type WaeResult<T> = Result<T, WaeError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    Validation,
    Auth,
    Permission,
    NotFound,
    Conflict,
    RateLimited,
    Network,
    Storage,
    Database,
    Cache,
    Config,
    Timeout,
    Internal,
}
```

## 模块依赖关系

```
                    ┌─────────────┐
                    │  wae-types  │
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│  wae-https    │  │ wae-authentic │  │  wae-storage  │
└───────────────┘  └───────────────┘  └───────────────┘
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ wae-server  │
                    └─────────────┘

                    ┌─────────────┐
                    │  wae-types  │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ wae-client  │
                    └─────────────┘
```

## 前后端通信

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend                              │
│  ┌──────────────────┐  ┌──────────────────┐                 │
│  │ Rust wae-client  │  │ TypeScript @wae/ │                 │
│  │ (信号系统、组件)  │  │ 客户端库        │                 │
│  └────────────┬─────┘  └──────────┬───────┘                 │
│               │                   │                         │
│               └───────────────────┘                         │
│                           │                                  │
└───────────────────────────┼──────────────────────────────────┘
                            │ HTTP/WebSocket
                            ▼
┌───────────────────────────┼──────────────────────────────────┐
│                       Backend                                │
│  ┌────────────┐  ┌─────────────────┐  ┌────────────────┐    │
│  │ wae-server │  │ wae-websocket   │  │ wae-storage    │    │
│  └─────┬──────┘  └────────┬────────┘  └───────┬────────┘    │
│        │                  │                    │              │
│        └──────────────────┼────────────────────┘              │
│                           │                                   │
│                    ┌──────┴──────┐                            │
│                    │  wae-types  │                            │
│                    └─────────────┘                            │
└──────────────────────────────────────────────────────────────┘
```
