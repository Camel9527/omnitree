// OpenDAL Google Drive Download Demo
// 下载文件示例：单文件下载、批量下载、范围读取

use anyhow::Result;
use opendal::services::Gdrive;
use opendal::Operator;

async fn setup_operator() -> Result<Operator> {
    let mut builder = Gdrive::default().root("/opendal-demo");
    
    if let Ok(refresh_token) = std::env::var("GDRIVE_REFRESH_TOKEN") {
        builder = builder
            .refresh_token(&refresh_token)
            .client_id(&std::env::var("GDRIVE_CLIENT_ID")?)
            .client_secret(&std::env::var("GDRIVE_CLIENT_SECRET")?);
    } else if let Ok(access_token) = std::env::var("GDRIVE_ACCESS_TOKEN") {
        builder = builder.access_token(&access_token);
    } else {
        anyhow::bail!("请设置 Google Drive 认证环境变量");
    }
    
    Ok(Operator::new(builder)?.finish())
}

#[tokio::main]
async fn main() -> Result<()> {
    let op = setup_operator().await?;
    
    println!("=== OpenDAL Google Drive 下载示例 ===\n");
    
    // 先上传一些测试文件
    println!("准备测试文件...");
    op.write("downloads/sample.txt", "这是一个示例文本文件，用于演示下载功能。").await?;
    op.write("downloads/data.json", r#"{"name":"test","value":123}"#).await?;
    println!("✓ 测试文件准备完成\n");
    
    // 1. 下载文本文件
    println!("1. 下载文本文件...");
    let content = op.read("downloads/sample.txt").await?;
    let text = String::from_utf8(content.to_vec())?;
    println!("   ✓ 下载内容: {}", text);
    println!("   ✓ 文件大小: {} bytes", content.len());
    
    // 2. 下载 JSON 文件并解析
    println!("\n2. 下载并解析 JSON 文件...");
    let json_content = op.read("downloads/data.json").await?;
    let json_value: serde_json::Value = serde_json::from_slice(&json_content)?;
    println!("   ✓ JSON 内容: {}", json_value);
    
    // 3. 范围读取 (部分下载)
    println!("\n3. 范围读取 (下载前10个字节)...");
    let partial = op.read_with("downloads/sample.txt")
        .range(0..10)
        .await?;
    let partial_text = String::from_utf8(partial.to_vec())?;
    println!("   ✓ 部分内容: '{}'", partial_text);
    
    // 4. 使用 Reader 流式下载
    println!("\n4. 流式下载文件...");
    let reader = op.reader("downloads/sample.txt").await?;
    let meta = reader.metadata().clone();
    println!("   ✓ 准备下载，文件大小: {} bytes", meta.content_length());
    
    // 5. 批量下载
    println!("\n5. 批量下载文件...");
    let files = vec!["downloads/sample.txt", "downloads/data.json"];
    for file in files {
        match op.read(file).await {
            Ok(content) => {
                println!("   ✓ 下载 {} 成功 ({} bytes)", file, content.len());
            }
            Err(e) => {
                println!("   ✗ 下载 {} 失败: {}", file, e);
            }
        }
    }
    
    // 6. 检查文件是否存在后再下载
    println!("\n6. 安全下载 (先检查存在性)...");
    let file_to_check = "downloads/sample.txt";
    if op.is_exist(file_to_check).await? {
        let content = op.read(file_to_check).await?;
        println!("   ✓ 文件存在，下载成功 ({} bytes)", content.len());
    } else {
        println!("   ✗ 文件不存在: {}", file_to_check);
    }
    
    // 7. 下载并获取元数据
    println!("\n7. 下载文件同时获取元数据...");
    let meta = op.stat("downloads/sample.txt").await?;
    let content = op.read("downloads/sample.txt").await?;
    println!("   ✓ 文件信息:");
    println!("     - 类型: {:?}", meta.mode());
    println!("     - 大小: {} bytes", meta.content_length());
    println!("     - 内容: {}", String::from_utf8(content.to_vec())?);
    
    // 8. 下载不存在的文件 (错误处理)
    println!("\n8. 错误处理示例...");
    match op.read("downloads/nonexistent.txt").await {
        Ok(_) => println!("   ✓ 文件存在"),
        Err(e) => println!("   ✓ 预期的错误: {}", e),
    }
    
    println!("\n=== 下载示例完成 ===");
    
    Ok(())
}
