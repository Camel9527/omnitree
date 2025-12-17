// OpenDAL Google Drive Stat Demo
// 获取文件/目录元数据信息示例

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

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let op = setup_operator().await?;
    
    println!("=== OpenDAL Google Drive 元数据示例 ===\n");
    
    // 准备测试文件
    println!("准备测试文件...");
    op.write("stat-demo/small.txt", "小文件内容").await?;
    op.write("stat-demo/medium.txt", "x".repeat(1024 * 10)).await?; // 10KB
    op.write("stat-demo/large.txt", "x".repeat(1024 * 100)).await?; // 100KB
    op.write("stat-demo/subdir/nested.txt", "嵌套文件").await?;
    println!("✓ 测试文件准备完成\n");
    
    // 1. 获取文件基本元数据
    println!("1. 获取文件基本元数据...");
    let meta = op.stat("stat-demo/small.txt").await?;
    println!("   文件: stat-demo/small.txt");
    println!("   类型: {:?}", meta.mode());
    println!("   是文件: {}", meta.is_file());
    println!("   是目录: {}", meta.is_dir());
    println!("   大小: {} bytes", meta.content_length());
    
    // 2. 获取详细元数据
    println!("\n2. 获取文件详细信息...");
    let meta = op.stat("stat-demo/medium.txt").await?;
    println!("   📄 stat-demo/medium.txt");
    println!("   ├─ 类型: {:?}", meta.mode());
    println!("   ├─ 大小: {}", format_size(meta.content_length()));
    
    if let Some(content_type) = meta.content_type() {
        println!("   ├─ Content-Type: {}", content_type);
    }
    
    if let Some(modified) = meta.last_modified() {
        println!("   ├─ 最后修改: {:?}", modified);
    }
    
    if let Some(etag) = meta.etag() {
        println!("   └─ ETag: {}", etag);
    }
    
    // 3. 比较不同大小文件的元数据
    println!("\n3. 比较不同文件的元数据...");
    let files = vec![
        "stat-demo/small.txt",
        "stat-demo/medium.txt",
        "stat-demo/large.txt",
    ];
    
    println!("   {:<30} {:>15} {:>10}", "文件名", "大小", "类型");
    println!("   {}", "-".repeat(58));
    
    for file in files {
        let meta = op.stat(file).await?;
        let size = format_size(meta.content_length());
        let mode = if meta.is_file() { "FILE" } else { "DIR" };
        println!("   {:<30} {:>15} {:>10}", file, size, mode);
    }
    
    // 4. 检查目录元数据
    println!("\n4. 检查目录元数据...");
    let dir_meta = op.stat("stat-demo/subdir/").await?;
    println!("   目录: stat-demo/subdir/");
    println!("   是目录: {}", dir_meta.is_dir());
    println!("   模式: {:?}", dir_meta.mode());
    
    // 5. 批量获取元数据
    println!("\n5. 批量获取文件信息...");
    let entries = op.list("stat-demo/").await?;
    println!("   找到 {} 个项目:\n", entries.len());
    
    for entry in entries.iter().take(10) {
        let meta = entry.metadata();
        let icon = if meta.is_dir() { "📁" } else { "📄" };
        let size = if meta.is_file() {
            format_size(meta.content_length())
        } else {
            String::from("-")
        };
        
        println!("   {} {:<25} {}", icon, entry.name(), size);
    }
    
    // 6. 检查文件是否存在
    println!("\n6. 检查文件存在性...");
    let files_to_check = vec![
        "stat-demo/small.txt",
        "stat-demo/nonexistent.txt",
        "stat-demo/subdir/",
    ];
    
    for file in files_to_check {
        let exists = op.is_exist(file).await?;
        let status = if exists { "✓ 存在" } else { "✗ 不存在" };
        println!("   {} {}", status, file);
    }
    
    // 7. 获取文件的时间戳信息
    println!("\n7. 获取时间戳信息...");
    let meta = op.stat("stat-demo/small.txt").await?;
    
    if let Some(modified) = meta.last_modified() {
        println!("   最后修改时间: {:?}", modified);
        
        // 计算距今时间
        let now = std::time::SystemTime::now();
        if let Ok(duration) = now.duration_since(modified.into()) {
            let seconds = duration.as_secs();
            if seconds < 60 {
                println!("   距今: {} 秒前", seconds);
            } else if seconds < 3600 {
                println!("   距今: {} 分钟前", seconds / 60);
            } else if seconds < 86400 {
                println!("   距今: {} 小时前", seconds / 3600);
            } else {
                println!("   距今: {} 天前", seconds / 86400);
            }
        }
    }
    
    // 8. 统计目录信息
    println!("\n8. 统计目录信息...");
    let entries = op.list("stat-demo/").await?;
    
    let mut total_size: u64 = 0;
    let mut file_count = 0;
    let mut dir_count = 0;
    
    for entry in &entries {
        let meta = entry.metadata();
        if meta.is_file() {
            file_count += 1;
            total_size += meta.content_length();
        } else if meta.is_dir() {
            dir_count += 1;
        }
    }
    
    println!("   📊 目录统计 (stat-demo/):");
    println!("   ├─ 文件数: {}", file_count);
    println!("   ├─ 目录数: {}", dir_count);
    println!("   └─ 总大小: {}", format_size(total_size));
    
    // 9. 查找最大和最小的文件
    println!("\n9. 查找最大和最小的文件...");
    let mut files: Vec<_> = entries.iter()
        .filter(|e| e.metadata().is_file())
        .collect();
    
    if !files.is_empty() {
        files.sort_by_key(|e| e.metadata().content_length());
        
        let smallest = files.first().unwrap();
        let largest = files.last().unwrap();
        
        println!("   最小文件: {} ({})", 
            smallest.name(), 
            format_size(smallest.metadata().content_length())
        );
        println!("   最大文件: {} ({})", 
            largest.name(), 
            format_size(largest.metadata().content_length())
        );
    }
    
    // 10. 错误处理 - 获取不存在文件的元数据
    println!("\n10. 错误处理示例...");
    match op.stat("stat-demo/nonexistent.txt").await {
        Ok(meta) => println!("   文件存在: {:?}", meta.mode()),
        Err(e) => println!("   ✓ 预期的错误: {}", e),
    }
    
    // 11. 元数据缓存和性能
    println!("\n11. 元数据访问性能测试...");
    let file = "stat-demo/small.txt";
    let iterations = 5;
    
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = op.stat(file).await?;
    }
    let duration = start.elapsed();
    
    println!("   {} 次 stat 操作耗时: {:?}", iterations, duration);
    println!("   平均耗时: {:?}", duration / iterations);
    
    println!("\n=== 元数据示例完成 ===");
    
    Ok(())
}
