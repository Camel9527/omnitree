// OpenDAL Google Drive Rename Demo
// 重命名/移动文件示例

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
    
    println!("=== OpenDAL Google Drive 重命名/移动示例 ===\n");
    
    // 准备测试文件
    println!("准备测试文件...");
    op.write("rename-demo/old_name.txt", "待重命名的文件").await?;
    op.write("rename-demo/to_move.txt", "待移动的文件").await?;
    op.write("rename-demo/source/file1.txt", "文件1").await?;
    op.write("rename-demo/source/file2.txt", "文件2").await?;
    op.write("rename-demo/temp.txt", "临时文件").await?;
    println!("✓ 测试文件准备完成\n");
    
    // 1. 基本重命名操作
    println!("1. 重命名文件 (同目录)...");
    println!("   原文件: rename-demo/old_name.txt");
    op.rename("rename-demo/old_name.txt", "rename-demo/new_name.txt").await?;
    println!("   ✓ 重命名为: rename-demo/new_name.txt");
    
    // 验证
    let exists_old = op.is_exist("rename-demo/old_name.txt").await?;
    let exists_new = op.is_exist("rename-demo/new_name.txt").await?;
    println!("   旧文件存在: {}, 新文件存在: {}", exists_old, exists_new);
    
    // 2. 移动文件到不同目录
    println!("\n2. 移动文件到不同目录...");
    op.rename("rename-demo/to_move.txt", "rename-demo/destination/moved_file.txt").await?;
    println!("   ✓ 移动成功: rename-demo/to_move.txt -> rename-demo/destination/moved_file.txt");
    
    // 3. 同时重命名和移动
    println!("\n3. 同时重命名和移动...");
    op.write("rename-demo/original.txt", "原始文件").await?;
    op.rename("rename-demo/original.txt", "rename-demo/archive/renamed_original.txt").await?;
    println!("   ✓ 文件已移动并重命名");
    
    // 4. 批量重命名
    println!("\n4. 批量重命名文件...");
    // 创建测试文件
    for i in 1..=3 {
        op.write(&format!("rename-demo/batch/old_{}.txt", i), &format!("内容{}", i)).await?;
    }
    
    // 批量重命名
    for i in 1..=3 {
        let old_path = format!("rename-demo/batch/old_{}.txt", i);
        let new_path = format!("rename-demo/batch/new_{}.txt", i);
        op.rename(&old_path, &new_path).await?;
        println!("   ✓ {} -> {}", old_path, new_path);
    }
    
    // 5. 移动到上级目录
    println!("\n5. 移动文件到上级目录...");
    op.write("rename-demo/subdir/nested.txt", "嵌套文件").await?;
    op.rename("rename-demo/subdir/nested.txt", "rename-demo/moved_up.txt").await?;
    println!("   ✓ 文件已移动到上级目录");
    
    // 6. 重命名时添加前缀/后缀
    println!("\n6. 添加前缀和后缀...");
    op.write("rename-demo/document.txt", "文档内容").await?;
    op.rename("rename-demo/document.txt", "rename-demo/backup_document_v2.txt").await?;
    println!("   ✓ 已添加前缀和后缀");
    
    // 7. 创建文件历史版本
    println!("\n7. 创建文件版本历史...");
    op.write("rename-demo/current.txt", "当前版本").await?;
    
    // 保存历史版本
    let content = op.read("rename-demo/current.txt").await?;
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let version_path = format!("rename-demo/versions/current_{}.txt", timestamp);
    op.write(&version_path, content).await?;
    println!("   ✓ 版本已保存: {}", version_path);
    
    // 8. 规范化文件名
    println!("\n8. 规范化文件名...");
    op.write("rename-demo/File With Spaces.txt", "包含空格").await?;
    op.rename("rename-demo/File With Spaces.txt", "rename-demo/file_with_underscores.txt").await?;
    println!("   ✓ 文件名已规范化");
    
    // 9. 移动整个目录的文件
    println!("\n9. 移动目录中的所有文件...");
    let source_entries = op.list("rename-demo/source/").await?;
    println!("   找到 {} 个文件需要移动", source_entries.len());
    
    for entry in source_entries {
        if entry.metadata().is_file() {
            let filename = entry.name();
            let new_path = format!("rename-demo/moved/{}", filename);
            op.rename(entry.path(), &new_path).await?;
            println!("   ✓ 移动: {} -> {}", entry.path(), new_path);
        }
    }
    
    // 10. 错误处理
    println!("\n10. 错误处理示例...");
    
    // 尝试重命名不存在的文件
    match op.rename("rename-demo/nonexistent.txt", "rename-demo/new.txt").await {
        Ok(_) => println!("   重命名成功"),
        Err(e) => println!("   ✓ 预期的错误: {}", e),
    }
    
    // 11. 查看最终结果
    println!("\n11. 查看目录结构...");
    async fn show_directory(op: &Operator, path: &str, indent: usize) -> Result<()> {
        let entries = op.list(path).await?;
        for entry in entries {
            let prefix = "  ".repeat(indent);
            if entry.metadata().is_dir() {
                println!("{}📁 {}", prefix, entry.name());
                show_directory(op, entry.path(), indent + 1).await?;
            } else {
                println!("{}📄 {}", prefix, entry.name());
            }
        }
        Ok(())
    }
    
    show_directory(&op, "rename-demo/", 0).await?;
    
    println!("\n=== 重命名/移动示例完成 ===");
    
    Ok(())
}
