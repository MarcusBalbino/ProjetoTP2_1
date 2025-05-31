use std::collections::HashSet;
use std::io::Write;
use kwic_gui::{ler_linhas_do_arquivo, gerar_rotations};

#[test]
fn unit_test_gerar_rotations_basic() {
    let phrases = vec!["The car is blue".to_string()];
    let mut stopwords = HashSet::new();
    stopwords.insert("is".to_string());
    // window_size = 2, case_insensitive
    let result = gerar_rotations(&phrases, &stopwords, 2, false);
    let mut expected = vec![
        "The car".to_string(),
        "blue The".to_string(),
        "car is".to_string(),
    ];
    expected.sort_by_key(|s| s.to_lowercase());
    assert_eq!(result, expected);
}

#[test]
fn unit_test_gerar_rotations_case_sensitive() {
    let phrases = vec!["Apple apple".to_string()];
    let stopwords = HashSet::new();
    // window_size = 1
    let result = gerar_rotations(&phrases, &stopwords, 1, true);
    let mut expected = vec!["Apple".to_string(), "apple".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn unit_test_gerar_rotations_large_window() {
    let phrases = vec!["a b c".to_string()];
    let stopwords = HashSet::new();
    // window_size maior que comprimento da frase
    let result = gerar_rotations(&phrases, &stopwords, 10, false);
    let mut expected = vec![
        "a b c".to_string(),
        "b c a".to_string(),
        "c a b".to_string(),
    ];
    expected.sort_by_key(|s| s.to_lowercase());
    assert_eq!(result, expected);
}

#[test]
fn unit_test_ler_linhas_do_arquivo() {
    // Cria um arquivo temporário para testar ler_linhas_do_arquivo
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("phrases_test.txt");
    let mut f = std::fs::File::create(&file_path).unwrap();
    writeln!(f, "linha1").unwrap();
    writeln!(f, "linha2").unwrap();
    drop(f);

    let lines = ler_linhas_do_arquivo(file_path.to_str().unwrap()).unwrap();
    assert_eq!(lines, vec!["linha1".to_string(), "linha2".to_string()]);

    // Remove o arquivo temporário
    std::fs::remove_file(file_path).unwrap();
}
