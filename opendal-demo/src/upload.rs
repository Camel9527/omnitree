// OpenDAL Google Drive Upload Demo
// 上传文件示例：单文件上传、批量上传、大文件上传

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
    
    println!("=== OpenDAL Google Drive 上传示例 ===\n");
    
    // 1. 上传文本文件
    println!("1. 上传文本文件...");
    let text_content = "这是一个测试文本文件\nOpenDAL Google Drive Demo\n2024";
    op.write("uploads/test.txt", text_content).await?;
    println!("   ✓ 上传 test.txt 成功");
    
    // 2. 上传 JSON 数据
    println!("\n2. 上传 JSON 文件...");
    let json_data = serde_json::json!({
        "name": "OpenDAL Demo",
        "version": "1.0",
        "features": ["upload", "download", "list", "delete"],
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    op.write("uploads/data.json", json_data.to_string().as_bytes())
        .await?;
    println!("   ✓ 上传 data.json 成功");
    
    // 3. 上传二进制数据
    println!("\n3. 上传二进制数据...");
    let binary_data: Vec<u8> = (0..=255).collect();
    op.write("uploads/binary.dat", binary_data).await?;
    println!("   ✓ 上传 binary.dat 成功 (256 bytes)");
    
    // 4. 批量上传文件
    println!("\n4. 批量上传文件...");
    for i in 1..=5 {
        let filename = format!("uploads/batch/file_{}.txt", i);
        let content = format!("这是批量上传的第 {} 个文件", i);
        op.write(&filename, content).await?;
        println!("   ✓ 上传 {} 成功", filename);
    }
    
    // 5. 上传大文件示例 (模拟)
    println!("\n5. 上传较大文件...");
    let large_content = "x".repeat(1024 * 100); // 100 KB
    op.write("uploads/large_file.txt", large_content).await?;
    println!("   ✓ 上传 large_file.txt 成功 (100 KB)");
    
    // 6. 使用不同路径上传
    println!("\n6. 上传到不同目录...");
    op.write("uploads/documents/report.txt", "年度报告内容")
        .await?;
    op.write("uploads/images/photo.txt", "图片描述文件")
        .await?;
    println!("   ✓ 上传到多个目录成功");
    
    // 7. 覆盖已有文件
    println!("\n7. 覆盖已有文件...");
    op.write("uploads/test.txt", "更新后的内容 - 第二版")
        .await?;
    println!("   ✓ 覆盖 test.txt 成功");
    
    // 验证上传
    println!("\n8. 验证上传结果...");
    let updated_content = op.read("uploads/test.txt").await?;
    println!("   ✓ 验证内容: {}", String::from_utf8(updated_content.to_vec())?);
    
    println!("\n=== 上传示例完成 ===");
    println!("提示: 运行 'cargo run --bin gdrive-list' 查看所有上传的文件");
    
    Ok(())
}
