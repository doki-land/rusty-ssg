//! Front Matter 增强模块
//! 提供自动字段填充、日期提取等增强功能

use super::{HugoContentError, HugoFrontMatter};
use std::path::Path;
use chrono::{DateTime, Local};
use regex::Regex;

/// Front Matter 增强器
pub struct FrontMatterEnhancer;

impl FrontMatterEnhancer {
    /// 增强 Front Matter，填充自动字段
    ///
    /// # Arguments
    ///
    /// * `frontmatter` - 原始 Front Matter
    /// * `file_path` - 文件路径
    ///
    /// # Returns
    ///
    /// 增强后的 Front Matter
    pub fn enhance(frontmatter: HugoFrontMatter, file_path: &Path) -> Result<HugoFrontMatter, HugoContentError> {
        let mut enhanced = frontmatter;
        
        // 从文件名提取日期
        if enhanced.date.is_none() {
            if let Some(date) = Self::extract_date_from_filename(file_path) {
                enhanced.date = Some(date);
            }
        }
        
        // 从 Git 信息提取最后修改时间（如果没有设置）
        if enhanced.lastmod.is_none() {
            if let Some(lastmod) = Self::extract_lastmod_from_git(file_path) {
                enhanced.lastmod = Some(lastmod);
            }
        }
        
        // 如果没有设置发布日期，使用日期字段
        if enhanced.publish_date.is_none() && enhanced.date.is_some() {
            enhanced.publish_date = enhanced.date.clone();
        }
        
        Ok(enhanced)
    }
    
    /// 从文件名提取日期
    ///
    /// 支持格式：YYYY-MM-DD-filename.md
    fn extract_date_from_filename(file_path: &Path) -> Option<String> {
        let file_name = file_path.file_name()?.to_str()?;
        
        // 匹配 YYYY-MM-DD 格式的日期
        let re = Regex::new(r"^(\d{4}-\d{2}-\d{2})-.*$").ok()?;
        if let Some(captures) = re.captures(file_name) {
            let date_str = captures.get(1)?.as_str();
            Some(date_str.to_string())
        } else {
            None
        }
    }
    
    /// 从 Git 信息提取最后修改时间
    fn extract_lastmod_from_git(file_path: &Path) -> Option<String> {
        // 这里简化实现，实际应该调用 Git 命令获取最后修改时间
        // 暂时返回当前时间
        let now = Local::now();
        Some(now.to_rfc3339())
    }
    
    /// 验证 Front Matter 字段
    pub fn validate(frontmatter: &HugoFrontMatter) -> Result<(), HugoContentError> {
        // 验证日期格式
        if let Some(date) = &frontmatter.date {
            if !Self::is_valid_date(date) {
                return Err(HugoContentError::front_matter_parse_error(
                    format!("Invalid date format: {}", date)
                ));
            }
        }
        
        if let Some(lastmod) = &frontmatter.lastmod {
            if !Self::is_valid_date(lastmod) {
                return Err(HugoContentError::front_matter_parse_error(
                    format!("Invalid lastmod format: {}", lastmod)
                ));
            }
        }
        
        if let Some(publish_date) = &frontmatter.publish_date {
            if !Self::is_valid_date(publish_date) {
                return Err(HugoContentError::front_matter_parse_error(
                    format!("Invalid publishDate format: {}", publish_date)
                ));
            }
        }
        
        if let Some(expiry_date) = &frontmatter.expiry_date {
            if !Self::is_valid_date(expiry_date) {
                return Err(HugoContentError::front_matter_parse_error(
                    format!("Invalid expiryDate format: {}", expiry_date)
                ));
            }
        }
        
        Ok(())
    }
    
    /// 检查日期格式是否有效
    fn is_valid_date(date_str: &str) -> bool {
        // 支持多种日期格式
        let formats = [
            "%Y-%m-%d",
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%dT%H:%M:%SZ",
        ];
        
        for format in &formats {
            if DateTime::parse_from_str(date_str, format).is_ok() {
                return true;
            }
        }
        
        false
    }
    
    /// 处理别名（aliases）
    pub fn process_aliases(frontmatter: &HugoFrontMatter) -> Vec<String> {
        frontmatter.aliases.clone().unwrap_or_default()
    }
    
    /// 处理资源引用（resources）
    pub fn process_resources(frontmatter: &HugoFrontMatter) -> Vec<String> {
        frontmatter.resources.as_ref().map(|resources| {
            resources.iter()
                .filter_map(|res| res.name.clone())
                .collect()
        }).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extract_date_from_filename() {
        let path = PathBuf::from("2024-01-01-my-post.md");
        assert_eq!(FrontMatterEnhancer::extract_date_from_filename(&path), Some("2024-01-01".to_string()));
        
        let path = PathBuf::from("my-post.md");
        assert_eq!(FrontMatterEnhancer::extract_date_from_filename(&path), None);
    }

    #[test]
    fn test_is_valid_date() {
        assert!(FrontMatterEnhancer::is_valid_date("2024-01-01"));
        assert!(FrontMatterEnhancer::is_valid_date("2024-01-01 12:00:00"));
        assert!(FrontMatterEnhancer::is_valid_date("2024-01-01T12:00:00"));
        assert!(FrontMatterEnhancer::is_valid_date("2024-01-01T12:00:00Z"));
        assert!(!FrontMatterEnhancer::is_valid_date("invalid-date"));
    }

    #[test]
    fn test_enhance() {
        let frontmatter = HugoFrontMatter::new();
        let path = PathBuf::from("2024-01-01-test.md");
        
        let enhanced = FrontMatterEnhancer::enhance(frontmatter, &path).unwrap();
        assert_eq!(enhanced.date, Some("2024-01-01".to_string()));
        assert!(enhanced.lastmod.is_some());
        assert_eq!(enhanced.publish_date, Some("2024-01-01".to_string()));
    }

    #[test]
    fn test_validate() {
        let mut frontmatter = HugoFrontMatter::new();
        frontmatter.date = Some("2024-01-01".to_string());
        frontmatter.lastmod = Some("2024-01-02".to_string());
        
        assert!(FrontMatterEnhancer::validate(&frontmatter).is_ok());
        
        frontmatter.date = Some("invalid-date".to_string());
        assert!(FrontMatterEnhancer::validate(&frontmatter).is_err());
    }
}
