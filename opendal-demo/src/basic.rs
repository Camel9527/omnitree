// OpenDAL Google Drive Basic Operations Demo
// 基础操作示例：读写、删除文件

use anyhow::Result;
use opendal::services::Gdrive;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // 从环境变量读取 Google Drive 凭证
    // 方式1: 使用 access_token (临时访问，有效期1小时)
    // 方式2: 使用 refresh_token + client_id + client_secret (长期访问，自动刷新)
    
    let mut builder = Gdrive::default();
    
    // 设置根目录 (可选)
    builder = builder.root("/opendal-demo");
    
    // 优先使用 refresh_token 方式 (推荐用于生产环境)
    if let Ok(refresh_token) = std::env::var("GDRIVE_REFRESH_TOKEN") {
        let client_id = std::env::var("GDRIVE_CLIENT_ID")?;
        let client_secret = std::env::var("GDRIVE_CLIENT_SECRET")?;
        
        builder = builder
            .refresh_token(&refresh_token)
            .client_id(&client_id)
            .client_secret(&client_secret);
        
        println!("✓ 使用 refresh_token 方式认证");
    } 
    // 备用方案：使用 access_token
    else if let Ok(access_token) = std::env::var("GDRIVE_ACCESS_TOKEN") {
        builder = builder.access_token(&access_token);
        println!("✓ 使用 access_token 方式认证 (1小时有效期)");
    } else {
        eprintln!("错误: 请设置环境变量:");
        eprintln!("  方式1 (推荐): GDRIVE_REFRESH_TOKEN, GDRIVE_CLIENT_ID, GDRIVE_CLIENT_SECRET");
        eprintln!("  方式2: GDRIVE_ACCESS_TOKEN");
        std::process::exit(1);
    }
    
    // 创建 Operator
    let op = Operator::new(builder)?.finish();
    
    println!("\n=== OpenDAL Google Drive 基础操作示例 ===\n");
    
    // 1. 写入文件
    println!("1. 写入文件 'hello.txt'...");
    op.write("hello.txt", "Hello, OpenDAL with Google Drive!")
        .await?;
    println!("   ✓ 写入成功");
    
    // 2. 读取文件
    println!("\n2. 读取文件 'hello.txt'...");
    let content = op.read("hello.txt").await?;
    let text = String::from_utf8(content.to_vec())?;
    println!("   ✓ 读取内容: {}", text);
    
    // 3. 获取文件元数据
    println!("\n3. 获取文件元数据...");
    let meta = op.stat("hello.txt").await?;
    println!("   ✓ 文件类型: {:?}", meta.mode());
    println!("   ✓ 文件大小: {} bytes", meta.content_length());
    if let Some(modified) = meta.last_modified() {
        println!("   ✓ 修改时间: {:?}", modified);
    }
    
    // 4. 检查文件是否存在
    println!("\n4. 检查文件是否存在...");
    let exists = op.is_exist("hello.txt").await?;
    println!("   ✓ 文件存在: {}", exists);
    
    // 5. 删除文件
    println!("\n5. 删除文件 'hello.txt'...");
    op.delete("hello.txt").await?;
    println!("   ✓ 删除成功");
    
    // 6. 验证文件已删除
    println!("\n6. 验证文件已删除...");
    let exists = op.is_exist("hello.txt").await?;
    println!("   ✓ 文件存在: {}", exists);
    
    println!("\n=== 基础操作演示完成 ===");
    
    Ok(())
}
