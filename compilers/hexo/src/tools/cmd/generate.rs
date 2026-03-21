//! 生成命令实现

use super::super::GenerateArgs;
use crate::types::Result;
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

/// 生成命令
pub struct GenerateCommand;

impl GenerateCommand {
    /// 执行生成命令
    pub async fn execute(args: GenerateArgs) -> Result<()> {
        // 确定源目录和输出目录
        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("source"));
        let output_dir = args.output.unwrap_or_else(|| PathBuf::from("public"));

        // 清理输出目录
        if args.clean {
            if output_dir.exists() {
                fs::remove_dir_all(&output_dir)?;
            }
        }

        // 创建输出目录
        fs::create_dir_all(&output_dir)?;

        // 开始计时
        let start_time = Instant::now();

        println!("Generating static files...");

        // 处理源目录中的文件
        if source_dir.exists() {
            Self::process_dir(&source_dir, &output_dir)?;
        }

        // 结束计时
        let elapsed = start_time.elapsed();
        let compile_time_ms = elapsed.as_millis() as u64;

        println!("Generated successfully in {} ms", compile_time_ms);
        Ok(())
    }

    /// 处理目录
    fn process_dir(src: &PathBuf, dst: &PathBuf) -> Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            let dest_path = dst.join(file_name);

            if path.is_dir() {
                fs::create_dir_all(&dest_path)?;
                Self::process_dir(&path, &dest_path)?;
            }
            else {
                Self::process_file(&path, &dest_path)?;
            }
        }
        Ok(())
    }

    /// 处理文件
    fn process_file(src: &Path, dst: &Path) -> Result<()> {
        // 复制所有文件
        fs::copy(src, dst)?;
        Ok(())
    }
}
