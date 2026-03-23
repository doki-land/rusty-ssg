use gatsby::GatsbyConfig;

fn main() {
    let toml = r#"
[siteMetadata]
title = "Test Site"
description = "A test site"

[[plugins]]
resolve = "gatsby-plugin-test"
"#;

    let config = GatsbyConfig::load_from_toml_str(toml).unwrap();
    println!("Site title: {:?}", config.site_title());
    println!("Site description: {:?}", config.site_description());
    println!("Site metadata: {:?}", config.site_metadata);
}
