// OpenDAL Google Drive Delete Demo
// 删除文件和目录示例

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
    
    println!("=== OpenDAL Google Drive 删除示例 ===\n");
    
    // 准备测试文件
    println!("准备测试文件...");
    op.write("deletions/file1.txt", "待删除文件1").await?;
    op.write("deletions/file2.txt", "待删除文件2").await?;
    op.write("deletions/file3.txt", "待删除文件3").await?;
    op.write("deletions/temp/temp1.txt", "临时文件1").await?;
    op.write("deletions/temp/temp2.txt", "临时文件2").await?;
    op.write("deletions/backup/data.txt", "备份文件").await?;
    println!("✓ 测试文件准备完成\n");
    
    // 1. 删除单个文件
    println!("1. 删除单个文件...");
    println!("   删除前检查: {}", op.is_exist("deletions/file1.txt").await?);
    op.delete("deletions/file1.txt").await?;
    println!("   ✓ 文件已删除");
    println!("   删除后检查: {}", op.is_exist("deletions/file1.txt").await?);
    
    // 2. 批量删除文件
    println!("\n2. 批量删除文件...");
    let files_to_delete = vec![
        "deletions/file2.txt",
        "deletions/file3.txt",
    ];
    
    for file in &files_to_delete {
        match op.delete(file).await {
            Ok(_) => println!("   ✓ 删除成功: {}", file),
            Err(e) => println!("   ✗ 删除失败: {} - {}", file, e),
        }
    }
    
    // 3. 删除目录 (需要先删除目录中的所有文件)
    println!("\n3. 删除目录及其内容...");
    let dir_to_delete = "deletions/temp/";
    
    // 先列出目录内容
    let entries = op.list(dir_to_delete).await?;
    println!("   目录包含 {} 个项目", entries.len());
    
    // 删除目录中的所有文件
    for entry in entries {
        if entry.metadata().is_file() {
            op.delete(entry.path()).await?;
            println!("   ✓ 删除文件: {}", entry.path());
        }
    }
    
    // 删除空目录
    op.delete(dir_to_delete).await?;
    println!("   ✓ 删除目录: {}", dir_to_delete);
    
    // 4. 递归删除目录
    println!("\n4. 递归删除目录...");
    async fn delete_recursive(op: &Operator, path: &str) -> Result<()> {
        let entries = op.list(path).await?;
        
        for entry in entries {
            if entry.metadata().is_dir() {
                // 递归删除子目录
                delete_recursive(op, entry.path()).await?;
                op.delete(entry.path()).await?;
                println!("   ✓ 删除目录: {}", entry.path());
            } else {
                // 删除文件
                op.delete(entry.path()).await?;
                println!("   ✓ 删除文件: {}", entry.path());
            }
        }
        
        Ok(())
    }
    
    delete_recursive(&op, "deletions/backup/").await?;
    op.delete("deletions/backup/").await?;
    println!("   ✓ 递归删除完成");
    
    // 5. 安全删除 (先检查是否存在)
    println!("\n5. 安全删除 (先检查是否存在)...");
    let file_to_check = "deletions/maybe_exists.txt";
    
    if op.is_exist(file_to_check).await? {
        op.delete(file_to_check).await?;
        println!("   ✓ 文件存在，已删除");
    } else {
        println!("   ⚠ 文件不存在，跳过删除");
    }
    
    // 6. 删除不存在的文件 (错误处理)
    println!("\n6. 删除不存在的文件 (错误处理)...");
    match op.delete("deletions/nonexistent.txt").await {
        Ok(_) => println!("   ✓ 删除成功"),
        Err(e) => println!("   ⚠ 删除失败 (预期): {}", e),
    }
    
    // 7. 使用通配符批量删除
    println!("\n7. 按模式批量删除文件...");
    // 创建测试文件
    for i in 1..=5 {
        op.write(&format!("deletions/pattern/test_{}.txt", i), "test").await?;
    }
    
    // 列出并删除匹配的文件
    let entries = op.list("deletions/pattern/").await?;
    let mut deleted_count = 0;
    for entry in entries {
        if entry.name().starts_with("test_") {
            op.delete(entry.path()).await?;
            deleted_count += 1;
        }
    }
    println!("   ✓ 删除了 {} 个文件", deleted_count);
    
    // 8. 清理空目录
    println!("\n8. 清理测试目录...");
    if op.is_exist("deletions/pattern/").await? {
        op.delete("deletions/pattern/").await?;
        println!("   ✓ 清理完成");
    }
    
    // 9. 验证删除结果
    println!("\n9. 验证删除结果...");
    let remaining = op.list("deletions/").await?;
    println!("   📊 剩余项目数: {}", remaining.len());
    for entry in remaining {
        println!("      - {}", entry.path());
    }
    
    println!("\n=== 删除示例完成 ===");
    println!("提示: 删除操作不可逆，请谨慎操作");
    
    Ok(())
}
