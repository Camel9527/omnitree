// OpenDAL Google Drive Complete Demo
// 完整示例：展示所有主要操作的综合应用

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
        println!("✓ 使用 refresh_token 认证\n");
    } else if let Ok(access_token) = std::env::var("GDRIVE_ACCESS_TOKEN") {
        builder = builder.access_token(&access_token);
        println!("✓ 使用 access_token 认证\n");
    } else {
        eprintln!("错误: 请设置 Google Drive 认证环境变量");
        eprintln!("  方式1: GDRIVE_REFRESH_TOKEN, GDRIVE_CLIENT_ID, GDRIVE_CLIENT_SECRET");
        eprintln!("  方式2: GDRIVE_ACCESS_TOKEN");
        std::process::exit(1);
    }
    
    Ok(Operator::new(builder)?.finish())
}

async fn demo_basic_operations(op: &Operator) -> Result<()> {
    println!("=== 1. 基础操作 ===");
    
    // 写入
    op.write("complete/basic/file.txt", "Hello, OpenDAL!").await?;
    println!("✓ 写入文件");
    
    // 读取
    let content = op.read("complete/basic/file.txt").await?;
    println!("✓ 读取文件: {}", String::from_utf8(content.to_vec())?);
    
    // 元数据
    let meta = op.stat("complete/basic/file.txt").await?;
    println!("✓ 文件大小: {} bytes", meta.content_length());
    
    // 检查存在
    let exists = op.is_exist("complete/basic/file.txt").await?;
    println!("✓ 文件存在: {}", exists);
    
    Ok(())
}

async fn demo_directory_operations(op: &Operator) -> Result<()> {
    println!("\n=== 2. 目录操作 ===");
    
    // 创建目录结构
    op.write("complete/dirs/dir1/file1.txt", "内容1").await?;
    op.write("complete/dirs/dir1/file2.txt", "内容2").await?;
    op.write("complete/dirs/dir2/file3.txt", "内容3").await?;
    println!("✓ 创建目录结构");
    
    // 列出目录
    let entries = op.list("complete/dirs/").await?;
    println!("✓ 目录包含 {} 个项目", entries.len());
    
    // 递归列出
    async fn list_recursive(op: &Operator, path: &str, indent: usize) -> Result<()> {
        let entries = op.list(path).await?;
        for entry in entries {
            let prefix = "  ".repeat(indent);
            let icon = if entry.metadata().is_dir() { "📁" } else { "📄" };
            println!("{}{} {}", prefix, icon, entry.name());
            
            if entry.metadata().is_dir() {
                list_recursive(op, entry.path(), indent + 1).await?;
            }
        }
        Ok(())
    }
    
    println!("目录树:");
    list_recursive(op, "complete/dirs/", 1).await?;
    
    Ok(())
}

async fn demo_batch_operations(op: &Operator) -> Result<()> {
    println!("\n=== 3. 批量操作 ===");
    
    // 批量上传
    println!("批量上传文件...");
    for i in 1..=5 {
        let path = format!("complete/batch/file_{}.txt", i);
        let content = format!("批量文件 {}", i);
        op.write(&path, content).await?;
    }
    println!("✓ 上传了 5 个文件");
    
    // 批量读取
    println!("批量读取文件...");
    for i in 1..=5 {
        let path = format!("complete/batch/file_{}.txt", i);
        let content = op.read(&path).await?;
        println!("  ✓ {}: {}", path, String::from_utf8(content.to_vec())?);
    }
    
    // 批量复制
    println!("批量复制文件...");
    for i in 1..=5 {
        let src = format!("complete/batch/file_{}.txt", i);
        let dst = format!("complete/batch/copy_{}.txt", i);
        op.copy(&src, &dst).await?;
    }
    println!("✓ 复制了 5 个文件");
    
    // 批量删除
    println!("批量删除文件...");
    for i in 1..=5 {
        let path = format!("complete/batch/copy_{}.txt", i);
        op.delete(&path).await?;
    }
    println!("✓ 删除了 5 个文件");
    
    Ok(())
}

async fn demo_copy_rename(op: &Operator) -> Result<()> {
    println!("\n=== 4. 复制和重命名 ===");
    
    // 复制
    op.write("complete/ops/original.txt", "原始内容").await?;
    op.copy("complete/ops/original.txt", "complete/ops/copy.txt").await?;
    println!("✓ 复制文件");
    
    // 重命名
    op.rename("complete/ops/copy.txt", "complete/ops/renamed.txt").await?;
    println!("✓ 重命名文件");
    
    // 移动
    op.rename("complete/ops/renamed.txt", "complete/ops/moved/renamed.txt").await?;
    println!("✓ 移动文件");
    
    Ok(())
}

async fn demo_large_files(op: &Operator) -> Result<()> {
    println!("\n=== 5. 大文件操作 ===");
    
    // 上传大文件
    let large_content = "x".repeat(1024 * 500); // 500 KB
    let start = std::time::Instant::now();
    op.write("complete/large/big_file.txt", large_content).await?;
    let upload_time = start.elapsed();
    println!("✓ 上传 500 KB 文件，耗时: {:?}", upload_time);
    
    // 下载大文件
    let start = std::time::Instant::now();
    let content = op.read("complete/large/big_file.txt").await?;
    let download_time = start.elapsed();
    println!("✓ 下载 {} KB 文件，耗时: {:?}", 
        content.len() / 1024, download_time);
    
    // 范围读取
    let partial = op.read_with("complete/large/big_file.txt")
        .range(0..1024)
        .await?;
    println!("✓ 范围读取: {} bytes", partial.len());
    
    Ok(())
}

async fn demo_error_handling(op: &Operator) -> Result<()> {
    println!("\n=== 6. 错误处理 ===");
    
    // 读取不存在的文件
    match op.read("complete/nonexistent.txt").await {
        Ok(_) => println!("✗ 不应该成功"),
        Err(e) => println!("✓ 捕获错误: NotFound"),
    }
    
    // 安全操作
    let file = "complete/maybe_exists.txt";
    if op.is_exist(file).await? {
        op.delete(file).await?;
        println!("✓ 文件存在，已删除");
    } else {
        println!("✓ 文件不存在，跳过");
    }
    
    Ok(())
}

async fn demo_metadata_analysis(op: &Operator) -> Result<()> {
    println!("\n=== 7. 元数据分析 ===");
    
    // 创建测试文件
    op.write("complete/analysis/small.txt", "小").await?;
    op.write("complete/analysis/medium.txt", "x".repeat(1024)).await?;
    op.write("complete/analysis/large.txt", "x".repeat(10240)).await?;
    
    let entries = op.list("complete/analysis/").await?;
    
    // 统计信息
    let mut total_size: u64 = 0;
    let mut file_count = 0;
    
    println!("\n文件列表:");
    for entry in &entries {
        if entry.metadata().is_file() {
            let size = entry.metadata().content_length();
            total_size += size;
            file_count += 1;
            
            let size_str = if size >= 1024 {
                format!("{:.2} KB", size as f64 / 1024.0)
            } else {
                format!("{} bytes", size)
            };
            
            println!("  📄 {} - {}", entry.name(), size_str);
        }
    }
    
    println!("\n统计:");
    println!("  文件数: {}", file_count);
    println!("  总大小: {:.2} KB", total_size as f64 / 1024.0);
    println!("  平均大小: {:.2} KB", 
        (total_size as f64 / file_count as f64) / 1024.0);
    
    Ok(())
}

async fn demo_backup_workflow(op: &Operator) -> Result<()> {
    println!("\n=== 8. 备份工作流 ===");
    
    // 创建原始数据
    op.write("complete/backup/data/config.json", r#"{"version":"1.0"}"#).await?;
    op.write("complete/backup/data/data.txt", "重要数据").await?;
    println!("✓ 创建原始数据");
    
    // 创建备份
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_dir = format!("complete/backup/backups/{}/", timestamp);
    
    let entries = op.list("complete/backup/data/").await?;
    for entry in entries {
        if entry.metadata().is_file() {
            let filename = entry.name();
            let dst = format!("{}{}", backup_dir, filename);
            op.copy(entry.path(), &dst).await?;
            println!("  ✓ 备份: {} -> {}", filename, dst);
        }
    }
    
    // 验证备份
    let backup_entries = op.list(&backup_dir).await?;
    println!("✓ 备份完成，共 {} 个文件", backup_entries.len());
    
    Ok(())
}

async fn cleanup(op: &Operator) -> Result<()> {
    println!("\n=== 9. 清理 ===");
    
    // 清理演示数据
    async fn delete_recursive(op: &Operator, path: &str) -> Result<usize> {
        let mut count = 0;
        let entries = op.list(path).await?;
        
        for entry in entries {
            if entry.metadata().is_dir() {
                count += delete_recursive(op, entry.path()).await?;
                op.delete(entry.path()).await?;
            } else {
                op.delete(entry.path()).await?;
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    let deleted = delete_recursive(op, "complete/").await?;
    op.delete("complete/").await?;
    
    println!("✓ 清理完成，删除了 {} 个文件", deleted);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║   OpenDAL Google Drive 完整示例               ║");
    println!("║   All Google Drive Operations Demo           ║");
    println!("╚═══════════════════════════════════════════════╝\n");
    
    let op = setup_operator().await?;
    
    // 运行所有演示
    demo_basic_operations(&op).await?;
    demo_directory_operations(&op).await?;
    demo_batch_operations(&op).await?;
    demo_copy_rename(&op).await?;
    demo_large_files(&op).await?;
    demo_error_handling(&op).await?;
    demo_metadata_analysis(&op).await?;
    demo_backup_workflow(&op).await?;
    
    // 清理（可选）
    println!("\n是否清理演示数据? (跳过清理，请注释掉下面这行)");
    // cleanup(&op).await?;
    println!("提示: 如需清理，请取消注释 cleanup 调用");
    
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   演示完成！                                  ║");
    println!("║   Demo Completed Successfully!                ║");
    println!("╚═══════════════════════════════════════════════╝");
    
    Ok(())
}
