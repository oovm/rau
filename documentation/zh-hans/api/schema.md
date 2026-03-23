# Schema 语法规范

WAE 采用基于 Schema-Driven Development 的代码生成工具 `wae generate`，它读取自定义的 `.wae` 或 `.rbq` 文件，生成前后端各自的 Rust 类型和 API 客户端/服务端桩代码。

## 一、基本结构

```ruby
# 命名空间（可选，用于生成模块）
namespace homepage;

# 表定义（数据库模型）
@table(name = "providers")
model Providers {
    @key
    id: string;
    user_id: string?;
    organization_id: string?;
    service_scope: string;
    years_of_experience: i32? = 0;
    team_size: i32? = 0;
    notes: string;
    created_at: string;
    updated_at: string;
}

# 传输类型（纯 DTO）
message CreateProviderRequest {
    user_id: string?;
    organization_id: string?;
    service_scope: string;
    years_of_experience: i32?;
    team_size: i32?;
    notes: string;
}

# 中性类型（可同时用于数据库和传输）
class ProviderInfo {
    id: string;
    service_scope: string;
    notes: string;
}

# 服务定义
service ProviderService {
    # RPC 方法（使用自定义协议）
    rpc CreateProvider(req: CreateProviderRequest) -> ProviderInfo;

    # HTTP RESTful 方法
    @http(GET, "/providers/:id")
    rpc GetProvider(id: string) -> ProviderInfo;

    @http(POST, "/providers")
    rpc CreateProvider(req: CreateProviderRequest) -> ProviderInfo;

    @http(PUT, "/providers/:id")
    rpc UpdateProvider(id: string, req: CreateProviderRequest) -> ProviderInfo;

    @http(DELETE, "/providers/:id")
    rpc DeleteProvider(id: string) -> Empty;

    # 流式响应（SSE 或 WebSocket）
    @http(GET, "/providers/stream")
    rpc StreamProviders() -> stream<ProviderInfo>;
}
```

## 二、类型系统

| 类型 | 说明 | 示例 |
|------|------|------|
| `string` | UTF-8 字符串 | `name: string;` |
| `i32`, `i64`, `u32`, `u64` | 整数 | `age: i32;` |
| `f32`, `f64` | 浮点数 | `price: f64;` |
| `bool` | 布尔值 | `active: bool;` |
| `bytes` | 二进制数据 | `avatar: bytes;` |
| `timestamp` | 时间戳（自动映射为 `chrono::DateTime<Utc>`） | `created_at: timestamp;` |
| `T?` | 可选类型（`Option<T>`） | `user_id: string?;` |
| `list<T>` | 列表（`Vec<T>`） | `tags: list<string>;` |
| `map<K, V>` | 映射（`HashMap<K, V>`） | `metadata: map<string, string>;` |
| 自定义类型 | 其他 model/message/class | `provider: ProviderInfo;` |

## 三、注解

- `@table(name = "xxx")`：标记为数据库表，指定表名。
- `@key`：标记主键（支持复合键，用多个 `@key` 或 `@key(columns = ["a","b"])`）。
- `@unique`：标记唯一约束。
- `@index`：标记索引。
- `@default`：默认值（可直接 `= 0` 语法）。
- `@http(method, path)`：指定 HTTP 方法和路径。
- `@stream`：表示响应是流式。

## 四、特殊类型

- `Empty`：内置空类型，用于无返回值的方法。

## 五、Service 语法规则

```ruby
service ProviderService {
    # 1. 高效 RPC（wae 原生二进制协议）
    rpc create_provider(req: CreateProviderRequest) -> ProviderInfo;

    # 等价写法（使用 @rpc 属性）
    @rpc
    create_provider(req: CreateProviderRequest) -> ProviderInfo;

    # 2. HTTP RESTful（JSON）
    @http(GET, "/providers/:id")
    get_provider(id: string) -> ProviderInfo;
    # 参数和返回值默认使用 JSON 序列化，无需显式包裹

    # 3. 明确指定 JSON 包装（可选，但推荐保持一致性）
    @http(POST, "/providers")
    create_provider(req: CreateProviderRequest) -> ProviderInfo;

    # 4. 流式响应（SSE/WebSocket）
    @http(GET, "/providers/stream")
    stream_providers() -> stream<ProviderInfo>;

    # 5. gRPC（待扩展）
    @grpc
    get_provider_by_id(id: string) -> ProviderInfo;
}
```

### 关键点
- **无参数的方法**可以省略 `@` 属性，直接写方法名（但通常推荐显式标注协议）。
- **参数和返回值**默认都是 JSON 序列化（对于 `@http`），无需包裹 `Json<T>`。
- **`rpc` 关键字**表示使用 wae 自有的高效二进制协议（例如基于 MessagePack 或自定义编码）。
- **`@http`** 注解必须指定 HTTP 方法和路径模板，路径中的 `:id` 等变量会自动映射到同名参数。
- **`stream<T>`** 返回类型表示流式响应，可映射为 SSE 或 WebSocket（取决于配置）。
- 未来可以添加 `@grpc`、`@websocket` 等注解，支持多种协议。

## 六、参数绑定与路径参数

对于 `@http` 方法，路径中的参数（如 `:id`）会自动匹配同名的方法参数（不需要额外注解）。其他参数（如请求体）则作为 JSON 体发送。

```ruby
@http(GET, "/users/:user_id/posts/:post_id")
get_post(user_id: string, post_id: string) -> Post;

@http(PUT, "/users/:user_id")
update_user(user_id: string, req: UpdateUserRequest) -> User;
```

- `user_id` 和 `post_id` 从 URL 路径中提取。
- `req` 从请求体中反序列化（JSON）。