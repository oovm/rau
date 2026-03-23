# 代码生成工具

WAE 提供基于 Schema-Driven Development 的代码生成工具 `wae generate`，它读取自定义的 `.wae` 或 `.rbq` 文件，生成前后端各自的 Rust 类型和 API 客户端/服务端桩代码。

## 一、使用方式

```bash
wae generate -i schema.wae -o ./generated
```

### 参数说明
- `-i`：输入 schema 文件（可多个，用逗号分隔）。
- `-o`：输出目录。
- `--server`：仅生成服务端代码（默认生成两者）。
- `--client`：仅生成客户端代码。
- `--rustfmt`：自动格式化生成的代码。

## 二、生成目录结构

```
generated/
├── server/                      # 服务端代码（供 wae-server 使用）
│   ├── models.rs                # 数据库模型（对应 model 和 class）
│   ├── dto.rs                   # 传输 DTO（对应 message 和 class 中用于传输的）
│   ├── services.rs              # Service trait 定义（供实现）
│   └── mod.rs                   # 模块导出
├── client/                      # 客户端代码（供 wae-client 使用）
│   ├── dto.rs                   # 传输 DTO（与 server 端一致）
│   ├── client.rs                # HTTP 客户端封装（基于 reqwest）
│   └── mod.rs
└── shared/                      # 可选共享辅助代码（如错误类型）
    ├── error.rs
    └── mod.rs
```

## 三、类型生成规则

- **model**：在 server 端生成带 ORM 特性的结构体（如 `#[derive(Queryable, Insertable)]` for diesel），在 client 端不生成（或仅生成只读类型）。
- **message**：在 server 和 client 端都生成纯 DTO（`#[derive(Serialize, Deserialize)]`）。
- **class**：根据依赖分析决定：
  - 如果仅被 model 引用，则只生成 server 端模型。
  - 如果仅被 message/service 引用，则生成 DTO。
  - 如果同时被两者引用，则生成两个版本（或通过 feature 区分）。

依赖分析：在生成时遍历整个 schema，标记每个 class 的使用场景。

## 四、服务生成

### Server 端

为每个 service 生成一个 trait，供开发者实现：

```rust
// services.rs
pub trait ProviderService: Send + Sync {
    async fn create_provider(&self, req: CreateProviderRequest) -> Result<ProviderInfo, Error>;
    async fn get_provider(&self, id: String) -> Result<ProviderInfo, Error>;
    async fn create_provider(&self, req: CreateProviderRequest) -> Result<ProviderInfo, Error>;
    async fn update_provider(&self, id: String, req: CreateProviderRequest) -> Result<ProviderInfo, Error>;
    async fn delete_provider(&self, id: String) -> Result<(), Error>;
    async fn stream_providers(&self) -> Result<impl Stream<Item = ProviderInfo>, Error>;
}
```

同时生成 axum 路由适配器，将 trait 方法挂载到对应的 HTTP 路由上：

```rust
// router.rs（由 generate 生成）
pub fn provider_service_router<S: ProviderService>(service: S) -> axum::Router {
    Router::new()
        .route("/providers/:id", get(get_provider_handler::<S>))
        .route("/providers", post(create_provider_handler::<S>))
        // ... 其他路由
        .with_state(service)
}
```

### Client 端

生成一个 HTTP 客户端，封装所有请求，供前端调用：

```rust
// client.rs
pub struct ProviderServiceClient {
    client: reqwest::Client,
    base_url: String,
}

impl ProviderServiceClient {
    pub async fn create_provider(&self, req: CreateProviderRequest) -> Result<ProviderInfo, Error> {
        self.client.post(format!("{}/providers", self.base_url))
            .json(&req)
            .send()
            .await?
            .json()
            .await
    }
    // ...
}
```

## 五、与 wae-server/wae-client 集成

### wae-server 端

在服务端实现生成的 trait：

```rust
use generated::server::{ProviderService, CreateProviderRequest, ProviderInfo};

struct MyProviderService {
    db: DbPool,
}

#[async_trait]
impl ProviderService for MyProviderService {
    async fn create_provider(&self, req: CreateProviderRequest) -> Result<ProviderInfo, Error> {
        // 业务逻辑
    }
    // ...
}
```

然后挂载路由：

```rust
let service = MyProviderService::new(db);
let app = provider_service_router(service);
```

### wae-client 端

在前端（Web 或桌面）使用生成的客户端：

```rust
use generated::client::ProviderServiceClient;

let client = ProviderServiceClient::new("http://localhost:3000");

// 在组件中调用
let provider = client.create_provider(req).await?;
```

可以结合 `wae-client` 的信号系统，例如在组件挂载时发起请求：

```rust
#[component]
fn ProviderList() -> Element {
    let client = ProviderServiceClient::new("http://localhost:3000");
    let providers = signal(Vec::new());
    
    // 组件挂载时加载数据
    {}
    
    html! {
        <div>
            {providers.get().iter().map(|provider| {
                html! { <div>{provider.service_scope}</div> }
            })}
        </div>
    }
}
```

## 六、开发工作流

1. **定义 Schema**：在 `api/schema.wae` 中定义数据模型和服务。
2. **生成代码**：运行 `wae generate -i api/schema.wae -o generated`。
3. **实现服务端**：在 `server` crate 中实现生成的 trait，添加业务逻辑和数据库操作。
4. **开发前端**：在 `client` crate 中导入生成的客户端类型，调用 API。
5. **修改 Schema**：当需求变化时，修改 `.wae` 文件并重新生成，两端同步更新。

## 七、优势

- **前后端完全解耦**：没有共享依赖，避免版本冲突。
- **类型安全**：编译时检查，生成代码保证两端契约一致。
- **开发效率**：自动生成大量样板代码（序列化、路由、客户端）。
- **灵活性**：支持多种协议（RPC、HTTP、流），可扩展。
- **与框架无关**：生成的是纯 Rust 代码，可以轻松集成到 `wae-server` 和 `wae-client` 中。

## 八、示例完整流程

### 8.1 schema.wae

```ruby
namespace todo;

message Todo {
    id: string;
    text: string;
    completed: bool;
}

message CreateTodo {
    text: string;
}

service TodoService {
    @http(GET, "/todos")
    rpc ListTodos() -> list<Todo>;

    @http(POST, "/todos")
    rpc CreateTodo(req: CreateTodo) -> Todo;

    @http(PUT, "/todos/:id")
    rpc UpdateTodo(id: string, req: CreateTodo) -> Todo;

    @http(DELETE, "/todos/:id")
    rpc DeleteTodo(id: string) -> Empty;
}
```

### 8.2 生成代码

- server 端：生成 `TodoService` trait 和 axum 路由。
- client 端：生成 `TodoServiceClient`。

### 8.3 服务端实现

```rust
struct MyTodoService {
    db: DbPool,
}

#[async_trait]
impl TodoService for MyTodoService {
    async fn list_todos(&self) -> Result<Vec<Todo>, Error> {
        // 从数据库查询
    }
    async fn create_todo(&self, req: CreateTodo) -> Result<Todo, Error> {
        // 插入数据库
    }
    // ...
}
```

### 8.4 前端调用

```rust
use generated::client::TodoServiceClient;

#[component]
fn TodoList() -> Element {
    let client = TodoServiceClient::new("/api");
    let todos = signal(Vec::new());
    
    // 组件挂载时加载数据
    {}
    
    html! {
        <div>
            {todos.get().iter().map(|todo| {
                html! { <div>{todo.text}</div> }
            })}
        </div>
    }
}
```