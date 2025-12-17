# OpenDAL Google Drive Demo

这是一个完整的 OpenDAL Google Drive 操作示例项目，展示了如何使用 Rust 和 OpenDAL 进行 Google Drive 的各种操作。

## 📋 目录

- [功能特性](#功能特性)
- [前置要求](#前置要求)
- [Google Drive API 设置](#google-drive-api-设置)
- [安装](#安装)
- [配置](#配置)
- [运行示例](#运行示例)
- [示例说明](#示例说明)
- [常见问题](#常见问题)

## ✨ 功能特性

本项目包含以下 Google Drive 操作的完整示例：

- ✅ **基础操作** - 读写、删除文件
- ✅ **上传文件** - 单文件、批量、大文件上传
- ✅ **下载文件** - 完整下载、范围读取、流式下载
- ✅ **列出文件** - 列出目录、递归遍历、过滤
- ✅ **删除操作** - 删除文件、目录、批量删除
- ✅ **复制文件** - 文件复制、跨目录复制
- ✅ **重命名/移动** - 文件重命名、移动文件
- ✅ **元数据获取** - 文件信息、大小、时间戳
- ✅ **完整示例** - 综合所有操作的实际应用

## 📦 前置要求

- Rust 1.70 或更高版本
- Google Cloud 账户
- Google Drive API 访问权限

## 🔑 Google Drive API 设置

### 1. 创建 Google Cloud 项目

1. 访问 [Google Cloud Console](https://console.cloud.google.com/)
2. 创建新项目或选择现有项目
3. 启用 **Google Drive API**

### 2. 创建 OAuth 2.0 凭证

1. 在 Google Cloud Console 中，进入 **APIs & Services** > **Credentials**
2. 点击 **Create Credentials** > **OAuth client ID**
3. 选择应用类型（例如：Desktop app）
4. 下载凭证 JSON 文件

### 3. 获取 Refresh Token

#### 方式 1: 使用 OAuth 2.0 Playground

1. 访问 [OAuth 2.0 Playground](https://developers.google.com/oauthplayground/)
2. 点击右上角的设置图标 ⚙️
3. 勾选 **"Use your own OAuth credentials"**
4. 输入你的 Client ID 和 Client Secret
5. 在左侧 **Step 1** 中，找到 **Drive API v3**
6. 选择 `https://www.googleapis.com/auth/drive`
7. 点击 **Authorize APIs**
8. 登录你的 Google 账户并授权
9. 在 **Step 2** 中，点击 **Exchange authorization code for tokens**
10. 复制 **Refresh token**

#### 方式 2: 使用 Access Token（临时，1小时有效期）

如果只是快速测试，可以从 OAuth Playground 直接获取 Access Token，但它只有1小时有效期。

## 🚀 安装

```bash
# 克隆或创建项目目录
cd opendal-demo1

# 构建项目
cargo build --release
```

## ⚙️ 配置

### 环境变量设置

创建 `.env` 文件或设置环境变量：

#### 方式 1: 使用 Refresh Token（推荐，长期有效）

```bash
export GDRIVE_REFRESH_TOKEN="your_refresh_token"
export GDRIVE_CLIENT_ID="your_client_id.apps.googleusercontent.com"
export GDRIVE_CLIENT_SECRET="your_client_secret"
```

#### 方式 2: 使用 Access Token（临时，1小时有效期）

```bash
export GDRIVE_ACCESS_TOKEN="your_access_token"
```

### 使用 .env 文件

创建 `.env` 文件：

```bash
# 方式 1: Refresh Token (推荐)
GDRIVE_REFRESH_TOKEN=your_refresh_token
GDRIVE_CLIENT_ID=your_client_id.apps.googleusercontent.com
GDRIVE_CLIENT_SECRET=your_client_secret

# 方式 2: Access Token (临时)
# GDRIVE_ACCESS_TOKEN=your_access_token
```

然后在运行命令前加载：

```bash
source .env
```

## 🎯 运行示例

### 1. 基础操作示例

```bash
cargo run --bin gdrive-basic
```

演示内容：
- 写入文件
- 读取文件
- 获取元数据
- 检查文件是否存在
- 删除文件

### 2. 上传文件示例

```bash
cargo run --bin gdrive-upload
```

演示内容：
- 上传文本文件
- 上传 JSON 数据
- 上传二进制数据
- 批量上传
- 上传大文件

### 3. 下载文件示例

```bash
cargo run --bin gdrive-download
```

演示内容：
- 下载文本文件
- 下载并解析 JSON
- 范围读取（部分下载）
- 流式下载
- 批量下载

### 4. 列出文件示例

```bash
cargo run --bin gdrive-list
```

演示内容：
- 列出目录内容
- 递归遍历
- 过滤文件/目录
- 获取详细信息
- 统计目录信息

### 5. 删除操作示例

```bash
cargo run --bin gdrive-delete
```

演示内容：
- 删除单个文件
- 批量删除
- 递归删除目录
- 安全删除（先检查存在性）

### 6. 复制文件示例

```bash
cargo run --bin gdrive-copy
```

演示内容：
- 基本复制
- 跨目录复制
- 批量复制
- 创建备份

### 7. 重命名/移动示例

```bash
cargo run --bin gdrive-rename
```

演示内容：
- 重命名文件
- 移动文件到不同目录
- 批量重命名
- 文件版本管理

### 8. 元数据获取示例

```bash
cargo run --bin gdrive-stat
```

演示内容：
- 获取文件元数据
- 比较文件信息
- 统计目录大小
- 查找最大/最小文件

### 9. 完整示例（推荐）

```bash
cargo run --bin gdrive-complete
```

这是一个综合示例，包含所有主要操作的实际应用场景。

## 📚 示例说明

### 项目结构

```
opendal-demo1/
├── Cargo.toml              # 项目配置和依赖
├── README.md               # 本文档
└── src/
    ├── basic.rs            # 基础操作
    ├── upload.rs           # 上传示例
    ├── download.rs         # 下载示例
    ├── list.rs             # 列表操作
    ├── delete.rs           # 删除操作
    ├── copy.rs             # 复制操作
    ├── rename.rs           # 重命名/移动
    ├── stat.rs             # 元数据获取
    └── complete_demo.rs    # 完整综合示例
```

### 代码示例

```rust
use anyhow::Result;
use opendal::services::Gdrive;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // 配置 Google Drive
    let builder = Gdrive::default()
        .root("/my-folder")
        .refresh_token(&std::env::var("GDRIVE_REFRESH_TOKEN")?)
        .client_id(&std::env::var("GDRIVE_CLIENT_ID")?)
        .client_secret(&std::env::var("GDRIVE_CLIENT_SECRET")?);
    
    // 创建 Operator
    let op = Operator::new(builder)?.finish();
    
    // 写入文件
    op.write("hello.txt", "Hello, OpenDAL!").await?;
    
    // 读取文件
    let content = op.read("hello.txt").await?;
    println!("{}", String::from_utf8(content.to_vec())?);
    
    // 列出文件
    let entries = op.list("/").await?;
    for entry in entries {
        println!("{}", entry.path());
    }
    
    Ok(())
}
```

## 🔧 常见问题

### Q: 出现 "invalid_grant" 错误

**A:** Refresh Token 可能已过期或无效。请重新获取 Refresh Token。

### Q: 出现 "insufficient authentication scopes" 错误

**A:** 确保 OAuth 授权时包含了 `https://www.googleapis.com/auth/drive` 范围。

### Q: 文件上传后在 Google Drive 中找不到

**A:** 检查 `root` 配置。如果设置了 `root("/folder")`，文件会在该文件夹下。可以尝试不设置 root 或设置为 `root("/")`。

### Q: Access Token 过期怎么办？

**A:** 使用 Refresh Token 方式（方式1），OpenDAL 会自动刷新 Access Token。

### Q: 如何在 Google Drive 中查看上传的文件？

**A:** 
1. 登录 [Google Drive](https://drive.google.com/)
2. 如果设置了 root 目录（如 `/opendal-demo`），文件会在该文件夹中
3. 如果没有设置 root，文件会在 "我的云端硬盘" 根目录

### Q: 支持哪些操作？

**A:** OpenDAL Google Drive 支持：
- ✅ create_dir - 创建目录
- ✅ stat - 获取元数据
- ✅ read - 读取文件
- ✅ write - 写入文件
- ✅ delete - 删除文件
- ✅ list - 列出目录
- ✅ copy - 复制文件
- ✅ rename - 重命名/移动文件
- ❌ presign - 预签名 URL（不支持）

## 📖 参考资源

- [OpenDAL 官方文档](https://opendal.apache.org/)
- [OpenDAL Google Drive 服务文档](https://docs.rs/opendal/latest/opendal/services/struct.Gdrive.html)
- [Google Drive API 文档](https://developers.google.com/drive/api/guides/about-sdk)
- [OAuth 2.0 文档](https://developers.google.com/identity/protocols/oauth2)

## 📝 许可证

本示例代码基于 Apache License 2.0 许可证。

## 🤝 贡献

欢迎提交问题和改进建议！

## 💡 提示

1. 首次运行建议先运行 `gdrive-basic` 测试基本连接
2. 使用 Refresh Token 方式可以长期使用，不需要频繁更新凭证
3. 所有示例都会在 Google Drive 中创建 `/opendal-demo` 目录
4. 可以通过修改 `root()` 参数来改变工作目录
5. 建议在开发环境中使用 `.env` 文件管理凭证，不要提交到版本控制

---

**Happy Coding! 🎉**
