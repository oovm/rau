# 快速开始

本节将帮助你在 5 分钟内创建一个基于 WAE 的 HTTP 服务。

## 安装

### 后端 (Rust)

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
wae = { path = "wae" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

### 前端 (TypeScript)

```bash
npm install @wae/core @wae/client
```

## 基础 HTTP 服务

创建一个使用 WAE 的 HTTP 服务：

```rust
use axum::{Router, routing::{get, post}, Json};
use wae_https::{HttpsServerBuilder, ApiResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    email: Option<String>,
}

async fn health() -> &'static str {
    "OK"
}

async fn hello() -> ApiResponse<HelloResponse> {
    ApiResponse::success(HelloResponse {
        message: "Hello, WAE!".to_string(),
    })
}

async fn create_user(
    Json(req): Json<CreateUserRequest>,
) -> ApiResponse<HelloResponse> {
    ApiResponse::success(HelloResponse {
        message: format!("Created user: {}", req.username),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello))
        .route("/users", post(create_user));

    let server = HttpsServerBuilder::new()
        .addr("0.0.0.0:3000".parse()?)
        .service_name("my-service")
        .router(router)
        .build();

    server.serve().await?;

    Ok(())
}
```

## 运行

```bash
cargo run
```

访问 `http://localhost:3000/health` 查看健康检查端点。

## HTTPS 配置

启用 HTTPS 只需添加 TLS 配置：

```rust
let server = HttpsServerBuilder::new()
    .addr("0.0.0.0:443".parse()?)
    .service_name("my-secure-service")
    .router(router)
    .tls("cert.pem", "key.pem")
    .build();
```

## HTTP/2 配置

WAE 支持 HTTP/2 协议：

```rust
use wae_https::{HttpsServerBuilder, Http2Config};

let http2_config = Http2Config::new()
    .with_max_concurrent_streams(256)
    .with_enable_push(false);

let server = HttpsServerBuilder::new()
    .addr("0.0.0.0:443".parse()?)
    .service_name("my-http2-service")
    .router(router)
    .tls("cert.pem", "key.pem")
    .http2_config(http2_config)
    .build();
```

## 统一响应结构

WAE 提供统一的 JSON 响应结构 `ApiResponse`：

```rust
use wae_https::ApiResponse;

#[derive(serde::Serialize)]
struct UserData {
    id: u64,
    name: String,
}

async fn get_user() -> ApiResponse<UserData> {
    ApiResponse::success(UserData {
        id: 1,
        name: "Alice".to_string(),
    })
}

async fn create_user() -> ApiResponse<()> {
    ApiResponse::error("INVALID_PARAMS", "用户名已存在")
}
```

响应格式：

```json
{
    "success": true,
    "data": { "id": 1, "name": "Alice" },
    "error": null,
    "trace_id": null
}
```

## 前端客户端

使用 TypeScript 客户端调用后端 API：

```typescript
import { createHttpClient } from "@wae/client";

const client = createHttpClient({
    baseUrl: "http://localhost:3000",
    timeout: 30000,
});

const response = await client.get<UserData>("/hello");

if (response.success) {
    console.log(response.data.message);
}
```

## 配置管理

使用 wae-config 加载配置：

```rust
use wae_config::{ConfigLoader, load_config};
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
    database: DatabaseConfig,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
}

let config: AppConfig = ConfigLoader::new()
    .with_toml("config.toml")
    .with_env("APP_")
    .extract()?;
```

TOML 配置文件：

```toml
name = "my-service"
port = 8080

[database]
host = "localhost"
port = 5432
name = "mydb"
```

环境变量：

```bash
APP_NAME=my-service
APP_PORT=8080
APP_DATABASE__HOST=localhost
APP_DATABASE__PORT=5432
```

## 前端框架使用

WAE 提供了完整的前端框架，让 Rust 开发者能够以简洁、强类型的方式编写前端代码。

### 1. 添加依赖

```toml
[dependencies]
wae-client = { path = "backends/wae-client", features = ["web"] }
wasm-bindgen = "0.2"
```

### 2. 创建组件

```rust
use wae_client::prelude::*;

#[component]
fn Counter(initial: i32) -> Element {
    let count = signal(initial);

    html! {
        <div class="flex items-center gap-2">
            <span>{count.get()}</span>
            <button on:click={move || count.set(count.get() + 1)}>
                "+"
            </button>
        </div>
    }
}

#[component]
fn App() -> Element {
    html! {
        <div class="p-4">
            <h1 class="text-xl font-bold">Welcome to WAE</h1>
            <Counter initial={0} />
        </div>
    }
}

fn main() {
    wae_client::start(App);
}
```

### 3. 构建和运行

```bash
# 构建前端资源
wae build --target web --out-dir static/wasm

# 运行后端服务
cargo run --bin server
```

## Schema 驱动开发

WAE 提供了基于 Schema-Driven Development 的代码生成工具，让前后端开发更加高效。

### 1. 创建 Schema 文件

创建 `api/schema.wae` 文件：

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
}
```

### 2. 生成代码

```bash
wae generate -i api/schema.wae -o generated
```

### 3. 实现服务端

```rust
use generated::server::{TodoService, CreateTodo, Todo};

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
}

// 挂载路由
let service = MyTodoService::new(db);
let app = todo_service_router(service);
```

### 4. 前端调用

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
            <button on:click={move || {
                // 创建新任务
            }}>
                "Add Todo"
            </button>
        </div>
    }
}
```

## 下一步

- [核心优势](/guide/advantages) - 了解 WAE 的设计理念
- [架构设计](/architecture/overview) - 深入了解 WAE 的架构
- [模块文档](/modules/ai) - 了解各个模块的功能
- [Schema 语法](/api/schema) - 学习 Schema 语法规范
- [代码生成工具](/api/codegen) - 了解代码生成工具的使用
