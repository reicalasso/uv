use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

// Basit test: iki requirements.txt dosyası birbirini -r ile referanslıyor
// Infinite recursion olmamalı, hata dönmeli
fn main() {
    let tmp = tempfile::tempdir().unwrap();
    let a_path = tmp.path().join("a.txt");
    let b_path = tmp.path().join("b.txt");
    fs::write(&a_path, "-r b.txt\n").unwrap();
    fs::write(&b_path, "-r a.txt\n").unwrap();
    let mut visited = HashSet::new();
    let working_dir = tmp.path();
    let client_builder = /* uygun client builder */ todo!("client builder mock");
    let result = uv_requirements_txt::RequirementsTxt::parse_with_visited(&a_path, working_dir, &client_builder, &mut visited);
    match result {
        Ok(_) => panic!("Circular include test failed: recursion not detected"),
        Err(e) => println!("Circular include detected as expected: {e}"),
    }
}
