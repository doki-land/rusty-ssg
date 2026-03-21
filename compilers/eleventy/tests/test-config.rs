extern crate eleventy;

use eleventy::config::Config;

fn main() {
    // 测试从 JSON 文件加载配置
    match Config::from_file("test-config.json") {
        Ok(config) => {
            println!("Successfully loaded config from test-config.json");
            println!("Input directory: {}", config.input_dir);
            println!("Output directory: {}", config.output_dir);
            println!("Template directory: {}", config.template_dir);
            println!("Data directory: {}", config.data_dir);
            println!("Plugins: {:?}", config.plugins);
            println!("Global data: {:?}", config.global_data);
        }
        Err(e) => {
            println!("Error loading config: {:?}", e);
        }
    }

    // 测试默认配置
    let default_config = Config::default();
    println!("\nDefault config:");
    println!("Input directory: {}", default_config.input_dir);
    println!("Output directory: {}", default_config.output_dir);
    println!("Template directory: {}", default_config.template_dir);
    println!("Data directory: {}", default_config.data_dir);
}
