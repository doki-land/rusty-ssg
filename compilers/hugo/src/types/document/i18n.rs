//! 国际化翻译模块
//! 提供多语言支持和翻译功能

use super::HugoContentIndex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read, path::Path};

/// 翻译条目
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct I18nEntry {
    /// 翻译文本
    pub translation: String,
    /// 翻译上下文
    pub context: Option<String>,
    /// 翻译注释
    pub comment: Option<String>,
}

/// 语言翻译映射
pub type I18nMap = HashMap<String, I18nEntry>;

/// 国际化系统
pub struct I18nSystem {
    /// 所有语言的翻译映射
    translations: HashMap<String, I18nMap>,
    /// 默认语言
    default_lang: String,
}

impl I18nSystem {
    /// 创建新的国际化系统
    pub fn new(default_lang: &str) -> Self {
        Self { translations: HashMap::new(), default_lang: default_lang.to_string() }
    }

    /// 加载翻译文件
    pub fn load_translations(&mut self, translations_dir: &Path) -> Result<(), std::io::Error> {
        if !translations_dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(translations_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yaml" || ext == "yml" || ext == "toml" {
                        self.load_translation_file(&path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// 加载单个翻译文件
    fn load_translation_file(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let filename = path.file_name().unwrap().to_str().unwrap();
        let lang = filename.split('.').next().unwrap();

        let translations: I18nMap = if path.extension() == Some(std::ffi::OsStr::new("toml")) {
            oak_toml::language::from_str(&content).unwrap()
        }
        else {
            oak_yaml::from_str(&content).unwrap()
        };

        self.translations.insert(lang.to_string(), translations);

        Ok(())
    }

    /// 获取翻译
    pub fn t(&self, lang: &str, key: &str) -> String {
        // 首先尝试获取指定语言的翻译
        if let Some(translations) = self.translations.get(lang) {
            if let Some(entry) = translations.get(key) {
                return entry.translation.clone();
            }
        }

        // 如果指定语言没有翻译，尝试默认语言
        if lang != self.default_lang {
            if let Some(translations) = self.translations.get(&self.default_lang) {
                if let Some(entry) = translations.get(key) {
                    return entry.translation.clone();
                }
            }
        }

        // 如果都没有找到，返回键本身
        key.to_string()
    }

    /// 获取翻译（带参数）
    pub fn t_with_params(&self, lang: &str, key: &str, params: &HashMap<&str, &str>) -> String {
        let mut translation = self.t(lang, key);

        // 替换参数
        for (param, value) in params {
            let placeholder = format!("{{{}}}", param);
            translation = translation.replace(&placeholder, value);
        }

        translation
    }

    /// 获取支持的语言列表
    pub fn get_supported_languages(&self) -> Vec<String> {
        self.translations.keys().cloned().collect()
    }

    /// 检查语言是否支持
    pub fn is_language_supported(&self, lang: &str) -> bool {
        self.translations.contains_key(lang)
    }
}
