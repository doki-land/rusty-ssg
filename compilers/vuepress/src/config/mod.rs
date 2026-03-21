//! VuePress 配置文件解析模块
//!
//! 支持解析 CommonJS 和 ES 模块格式的配置文件，处理配置文件中的导入和依赖

pub mod parser;
pub mod types;

pub use parser::ConfigParser;
pub use types::VuePressConfig;
