extern crate eleventy;

use eleventy::data::DataSystem;

fn main() {
    // 创建数据系统
    let mut data_system = DataSystem::new("./_data");

    // 测试 frontmatter 解析
    println!("Testing frontmatter parsing...");
    let content = include_str!("test-data.md");
    match data_system.load_frontmatter(content) {
        Ok((frontmatter, content)) => {
            println!("Successfully parsed frontmatter");
            println!("Frontmatter: {:?}", frontmatter);
            println!("Content: {}", content);
        }
        Err(e) => {
            println!("Error parsing frontmatter: {:?}", e);
        }
    }

    // 测试模板数据加载
    println!("\nTesting template data loading...");
    match data_system.load_template_data("test-data.md") {
        Ok(data) => {
            println!("Successfully loaded template data");
            println!("Template data: {:?}", data);
        }
        Err(e) => {
            println!("Error loading template data: {:?}", e);
        }
    }

    // 测试全局数据加载
    println!("\nTesting global data loading...");
    match data_system.load_global_data() {
        Ok(_) => {
            println!("Successfully loaded global data");
            println!("Global data: {:?}", data_system.global_data());
        }
        Err(e) => {
            println!("Error loading global data: {:?}", e);
        }
    }
}
