use crate::jekyll::{
    JekyllConfig, JekyllStructure,
    static_files::{StaticFile, StaticFileProcessor},
};
use std::{fs::File, io::Write, path::Path};
use tempfile::tempdir;

#[test]
fn test_static_file_new() {
    let root = Path::new("/test/root");
    let source = Path::new("/test/root/assets/image.jpg");
    let file = StaticFile::new(source, root);

    assert_eq!(file.source_path(), source);
    assert_eq!(file.relative_path(), Path::new("assets/image.jpg"));
}

#[test]
fn test_static_file_destination_path() {
    let root = Path::new("/test/root");
    let source = Path::new("/test/root/assets/image.jpg");
    let file = StaticFile::new(source, root);
    let dest = Path::new("/test/output");

    assert_eq!(file.destination_path(dest), Path::new("/test/output/assets/image.jpg"));
}

#[test]
fn test_matches_pattern() {
    let config = JekyllConfig::default();
    let structure = JekyllStructure::new(tempdir().unwrap().path()).unwrap();
    let processor = StaticFileProcessor::new(structure, config);

    assert!(processor.matches_pattern("test.txt", "test.txt"));
    assert!(processor.matches_pattern("dir/test.txt", "dir/test.txt"));
    assert!(processor.matches_pattern("any.txt", "*.txt"));
    assert!(processor.matches_pattern("dir/subdir/file.txt", "**/*.txt"));
    assert!(!processor.matches_pattern("test.jpg", "*.txt"));
}

#[test]
fn test_collect_static_files() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    std::fs::create_dir_all(root.join("assets")).unwrap();
    let mut file1 = File::create(root.join("assets/image.jpg")).unwrap();
    writeln!(file1, "test image").unwrap();

    let mut file2 = File::create(root.join("style.css")).unwrap();
    writeln!(file2, "test css").unwrap();

    std::fs::create_dir_all(root.join("_site")).unwrap();
    let mut excluded_file = File::create(root.join("_site/index.html")).unwrap();
    writeln!(excluded_file, "excluded").unwrap();

    let structure = JekyllStructure::new(root).unwrap();
    let config = JekyllConfig::default();
    let processor = StaticFileProcessor::new(structure, config);

    let files = processor.collect_static_files().unwrap();

    let relative_paths: Vec<_> = files.iter().map(|f| f.relative_path().to_string_lossy()).collect();
    assert!(relative_paths.contains(&"assets/image.jpg".into()));
    assert!(relative_paths.contains(&"style.css".into()));
    assert!(!relative_paths.contains(&"_site/index.html".into()));
}
