// OpenDAL Google Drive Copy Demo
// 复制文件示例

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
    
    println!("=== OpenDAL Google Drive 复制示例 ===\n");
    
    // 准备源文件
    println!("准备源文件...");
    op.write("copy-demo/source/original.txt", "这是原始文件内容").await?;
    op.write("copy-demo/source/data.json", r#"{"type":"source","value":100}"#).await?;
    op.write("copy-demo/source/large.txt", "x".repeat(1024)).await?;
    println!("✓ 源文件准备完成\n");
    
    // 1. 基本复制操作
    println!("1. 复制单个文件...");
    op.copy("copy-demo/source/original.txt", "copy-demo/dest/copy1.txt").await?;
    println!("   ✓ 复制成功: original.txt -> copy1.txt");
    
    // 验证复制结果
    let original = op.read("copy-demo/source/original.txt").await?;
    let copied = op.read("copy-demo/dest/copy1.txt").await?;
    assert_eq!(original, copied);
    println!("   ✓ 验证：内容一致");
    
    // 2. 复制到不同目录
    println!("\n2. 复制到不同目录...");
    op.copy("copy-demo/source/data.json", "copy-demo/backup/data.json").await?;
    op.copy("copy-demo/source/data.json", "copy-demo/archive/data.json").await?;
    println!("   ✓ 已复制到多个目录");
    
    // 3. 批量复制
    println!("\n3. 批量复制文件...");
    let files_to_copy = vec![
        ("copy-demo/source/original.txt", "copy-demo/batch/file1.txt"),
        ("copy-demo/source/data.json", "copy-demo/batch/file2.json"),
        ("copy-demo/source/large.txt", "copy-demo/batch/file3.txt"),
    ];
    
    for (src, dst) in &files_to_copy {
        match op.copy(src, dst).await {
            Ok(_) => println!("   ✓ {} -> {}", src, dst),
            Err(e) => println!("   ✗ 复制失败: {} - {}", src, e),
        }
    }
    
    // 4. 带重命名的复制
    println!("\n4. 复制并重命名...");
    op.copy("copy-demo/source/original.txt", "copy-demo/renamed/new_name.txt").await?;
    println!("   ✓ 复制并重命名完成");
    
    // 5. 创建备份副本
    println!("\n5. 创建带时间戳的备份...");
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("copy-demo/backups/original_{}.txt", timestamp);
    op.copy("copy-demo/source/original.txt", &backup_name).await?;
    println!("   ✓ 备份创建: {}", backup_name);
    
    // 6. 复制大文件
    println!("\n6. 复制大文件...");
    let start = std::time::Instant::now();
    op.copy("copy-demo/source/large.txt", "copy-demo/dest/large_copy.txt").await?;
    let duration = start.elapsed();
    println!("   ✓ 大文件复制完成 (耗时: {:?})", duration);
    
    // 7. 验证复制的文件元数据
    println!("\n7. 比较源文件和副本的元数据...");
    let src_meta = op.stat("copy-demo/source/original.txt").await?;
    let dst_meta = op.stat("copy-demo/dest/copy1.txt").await?;
    println!("   源文件大小: {} bytes", src_meta.content_length());
    println!("   副本大小: {} bytes", dst_meta.content_length());
    println!("   大小一致: {}", src_meta.content_length() == dst_meta.content_length());
    
    // 8. 覆盖已存在的目标文件
    println!("\n8. 覆盖已存在的文件...");
    op.write("copy-demo/dest/to_overwrite.txt", "旧内容").await?;
    println!("   原始内容: {}", String::from_utf8(op.read("copy-demo/dest/to_overwrite.txt").await?.to_vec())?);
    
    op.write("copy-demo/source/new_content.txt", "新内容").await?;
    op.copy("copy-demo/source/new_content.txt", "copy-demo/dest/to_overwrite.txt").await?;
    println!("   覆盖后内容: {}", String::from_utf8(op.read("copy-demo/dest/to_overwrite.txt").await?.to_vec())?);
    
    // 9. 统计复制结果
    println!("\n9. 统计复制结果...");
    let dest_entries = op.list("copy-demo/dest/").await?;
    let backup_entries = op.list("copy-demo/backup/").await?;
    let batch_entries = op.list("copy-demo/batch/").await?;
    
    println!("   📊 复制统计:");
    println!("     目标目录: {} 个文件", dest_entries.len());
    println!("     备份目录: {} 个文件", backup_entries.len());
    println!("     批量目录: {} 个文件", batch_entries.len());
    
    // 10. 错误处理 - 复制不存在的文件
    println!("\n10. 错误处理示例...");
    match op.copy("copy-demo/nonexistent.txt", "copy-demo/dest/error.txt").await {
        Ok(_) => println!("   ✓ 复制成功"),
        Err(e) => println!("   ✓ 预期的错误: {}", e),
    }
    
    println!("\n=== 复制示例完成 ===");
    
    Ok(())
}
