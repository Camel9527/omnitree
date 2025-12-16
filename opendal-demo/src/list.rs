// OpenDAL Google Drive List Demo
// 列出文件和目录示例

use anyhow::Result;
use opendal::services::Gdrive;
use opendal::Operator;
use opendal::EntryMode;

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
    
    println!("=== OpenDAL Google Drive 列表示例 ===\n");
    
    // 准备测试文件和目录
    println!("准备测试数据...");
    op.write("listing/file1.txt", "文件1").await?;
    op.write("listing/file2.txt", "文件2").await?;
    op.write("listing/subdir/file3.txt", "文件3").await?;
    op.write("listing/subdir/file4.txt", "文件4").await?;
    op.write("listing/another/file5.txt", "文件5").await?;
    println!("✓ 测试数据准备完成\n");
    
    // 1. 列出根目录下的所有内容
    println!("1. 列出根目录内容...");
    let mut entries = op.list("/").await?;
    println!("   根目录包含 {} 个项目:", entries.len());
    for entry in &entries {
        let mode = if entry.metadata().is_dir() { "📁 DIR " } else { "📄 FILE" };
        println!("   {} {}", mode, entry.path());
    }
    
    // 2. 列出特定目录
    println!("\n2. 列出 'listing/' 目录...");
    entries = op.list("listing/").await?;
    println!("   找到 {} 个项目:", entries.len());
    for entry in &entries {
        let meta = entry.metadata();
        let mode = if meta.is_dir() { "📁" } else { "📄" };
        let size = if meta.is_file() {
            format!("{} bytes", meta.content_length())
        } else {
            String::from("-")
        };
        println!("   {} {} ({})", mode, entry.path(), size);
    }
    
    // 3. 递归列出所有文件
    println!("\n3. 递归列出 'listing/' 下所有文件...");
    async fn list_recursive(op: &Operator, path: &str, indent: usize) -> Result<()> {
        let entries = op.list(path).await?;
        for entry in entries {
            let prefix = "  ".repeat(indent);
            let meta = entry.metadata();
            
            if meta.is_dir() {
                println!("{}📁 {}", prefix, entry.name());
                // 递归列出子目录
                list_recursive(op, entry.path(), indent + 1).await?;
            } else {
                println!("{}📄 {} ({} bytes)", 
                    prefix, 
                    entry.name(), 
                    meta.content_length()
                );
            }
        }
        Ok(())
    }
    
    list_recursive(&op, "listing/", 0).await?;
    
    // 4. 过滤文件类型
    println!("\n4. 只列出文件 (不包括目录)...");
    entries = op.list("listing/").await?;
    let files: Vec<_> = entries.iter()
        .filter(|e| e.metadata().is_file())
        .collect();
    println!("   找到 {} 个文件:", files.len());
    for file in files {
        println!("   📄 {}", file.path());
    }
    
    // 5. 只列出目录
    println!("\n5. 只列出目录...");
    let dirs: Vec<_> = entries.iter()
        .filter(|e| e.metadata().is_dir())
        .collect();
    println!("   找到 {} 个目录:", dirs.len());
    for dir in dirs {
        println!("   📁 {}", dir.path());
    }
    
    // 6. 获取文件详细信息
    println!("\n6. 获取文件详细信息...");
    entries = op.list("listing/").await?;
    for entry in entries.iter().take(3) {
        let meta = entry.metadata();
        println!("   文件: {}", entry.name());
        println!("     类型: {:?}", meta.mode());
        println!("     大小: {} bytes", meta.content_length());
        if let Some(modified) = meta.last_modified() {
            println!("     修改时间: {:?}", modified);
        }
        if let Some(content_type) = meta.content_type() {
            println!("     内容类型: {}", content_type);
        }
        println!();
    }
    
    // 7. 统计目录信息
    println!("7. 统计 'listing/' 目录信息...");
    let all_entries = op.list("listing/").await?;
    let file_count = all_entries.iter().filter(|e| e.metadata().is_file()).count();
    let dir_count = all_entries.iter().filter(|e| e.metadata().is_dir()).count();
    let total_size: u64 = all_entries.iter()
        .filter(|e| e.metadata().is_file())
        .map(|e| e.metadata().content_length())
        .sum();
    
    println!("   📊 目录统计:");
    println!("     文件数: {}", file_count);
    println!("     目录数: {}", dir_count);
    println!("     总大小: {} bytes", total_size);
    
    // 8. 检查空目录
    println!("\n8. 检查目录是否为空...");
    let test_dirs = vec!["listing/", "listing/subdir/"];
    for dir in test_dirs {
        let entries = op.list(dir).await?;
        let is_empty = entries.is_empty();
        println!("   {} - {}", dir, if is_empty { "空" } else { "非空" });
    }
    
    println!("\n=== 列表示例完成 ===");
    
    Ok(())
}
