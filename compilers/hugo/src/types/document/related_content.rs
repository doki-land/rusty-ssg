//! 相关内容推荐模块
//! 提供基于标签、分类和内容相似度的相关内容推荐功能

use super::{HugoContentIndex, HugoPage};
use std::collections::HashMap;
use std::f64;

/// 相关内容推荐器
pub struct RelatedContentRecommender {
    /// 内容索引
    content_index: &'static HugoContentIndex,
}

impl RelatedContentRecommender {
    /// 创建新的相关内容推荐器
    pub fn new(content_index: &'static HugoContentIndex) -> Self {
        Self {
            content_index,
        }
    }

    /// 获取与指定页面相关的内容
    ///
    /// # Arguments
    ///
    /// * `page` - 当前页面
    /// * `limit` - 返回的相关内容数量限制
    ///
    /// # Returns
    ///
    /// 相关内容列表
    pub fn get_related(&self, page: &HugoPage, limit: usize) -> Vec<(&HugoPage, f64)> {
        let mut scores = Vec::new();

        for other_page in &self.content_index.pages {
            // 跳过当前页面
            if other_page.path == page.path {
                continue;
            }

            // 计算相关度分数
            let score = self.calculate_relevance_score(page, other_page);
            
            // 只添加分数大于 0 的页面
            if score > 0.0 {
                scores.push((other_page, score));
            }
        }

        // 按分数降序排序
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // 限制返回数量
        scores.into_iter().take(limit).collect()
    }

    /// 计算两个页面之间的相关度分数
    fn calculate_relevance_score(&self, page: &HugoPage, other_page: &HugoPage) -> f64 {
        let mut score = 0.0;

        // 标签匹配分数（权重：0.5）
        score += self.calculate_tag_similarity(page, other_page) * 0.5;

        // 分类匹配分数（权重：0.3）
        score += self.calculate_category_similarity(page, other_page) * 0.3;

        // 部分匹配分数（权重：0.2）
        score += self.calculate_section_similarity(page, other_page) * 0.2;

        score
    }

    /// 计算标签相似度
    fn calculate_tag_similarity(&self, page: &HugoPage, other_page: &HugoPage) -> f64 {
        let page_tags = page.frontmatter.tags.as_ref().unwrap_or(&Vec::new());
        let other_tags = other_page.frontmatter.tags.as_ref().unwrap_or(&Vec::new());

        if page_tags.is_empty() || other_tags.is_empty() {
            return 0.0;
        }

        // 计算共同标签数量
        let common_tags = page_tags
            .iter()
            .filter(|tag| other_tags.contains(tag))
            .count();

        // 计算相似度分数（使用 Jaccard 相似度）
        let total_tags = page_tags.len() + other_tags.len() - common_tags;
        common_tags as f64 / total_tags as f64
    }

    /// 计算分类相似度
    fn calculate_category_similarity(&self, page: &HugoPage, other_page: &HugoPage) -> f64 {
        let page_categories = page.frontmatter.categories.as_ref().unwrap_or(&Vec::new());
        let other_categories = other_page.frontmatter.categories.as_ref().unwrap_or(&Vec::new());

        if page_categories.is_empty() || other_categories.is_empty() {
            return 0.0;
        }

        // 计算共同分类数量
        let common_categories = page_categories
            .iter()
            .filter(|category| other_categories.contains(category))
            .count();

        // 计算相似度分数（使用 Jaccard 相似度）
        let total_categories = page_categories.len() + other_categories.len() - common_categories;
        common_categories as f64 / total_categories as f64
    }

    /// 计算部分相似度
    fn calculate_section_similarity(&self, page: &HugoPage, other_page: &HugoPage) -> f64 {
        if page.section.is_none() || other_page.section.is_none() {
            return 0.0;
        }

        if page.section == other_page.section {
            1.0
        } else {
            0.0
        }
    }
}
