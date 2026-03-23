//! 测试基于 oaks 框架的 Astro 解析器

use astro::compiler::{AstroLanguage, AstroLexer, AstroParser};

#[test]
fn test_basic_astro_component() {
    let input = r#"---
import SomeComponent from './SomeComponent.astro';
const { title } = Astro.props;
---
<h1>{title}</h1>
<SomeComponent />
"#;

    let language = AstroLanguage::default();
    let lexer = AstroLexer;
    let parser = AstroParser;

    let tokens = lexer.lex(input);
    assert!(!tokens.is_empty());

    let nodes = parser.parse(input);
    assert!(!nodes.is_empty());
}

#[test]
fn test_astro_component_with_interpolation() {
    let input = r#"---
const { name } = Astro.props;
---
<div>
    <p>Hello, {name}!</p>
    <p>Unescaped: {{{name}}}</p>
</div>
"#;

    let language = AstroLanguage::default();
    let lexer = AstroLexer;
    let parser = AstroParser;

    let tokens = lexer.lex(input);
    assert!(!tokens.is_empty());

    let nodes = parser.parse(input);
    assert!(!nodes.is_empty());
}

#[test]
fn test_astro_component_with_directives() {
    let input = r#"---
const items = [1, 2, 3];
---
<ul>
    {% for item in items %}
        <li>{item}</li>
    {% endfor %}
</ul>
"#;

    let language = AstroLanguage::default();
    let lexer = AstroLexer;
    let parser = AstroParser;

    let tokens = lexer.lex(input);
    assert!(!tokens.is_empty());

    let nodes = parser.parse(input);
    assert!(!nodes.is_empty());
}

#[test]
fn test_astro_component_with_slots() {
    let input = r#"---
---
<div>
    <slot />
    <slot name="footer" />
</div>
"#;

    let language = AstroLanguage::default();
    let lexer = AstroLexer;
    let parser = AstroParser;

    let tokens = lexer.lex(input);
    assert!(!tokens.is_empty());

    let nodes = parser.parse(input);
    assert!(!nodes.is_empty());
}
