use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Write, BufRead, BufReader};
use std::env;

use kwic_gui::{ler_linhas_do_arquivo, gerar_rotations};

#[test]
fn integration_test_full_flow() {
    // 1) Cria caminhos para arquivos temporários:
    //    - phrases_path: onde vamos escrever as “frases” de entrada
    //    - stopwords_path: onde escrevemos as “stop words”
    //    - output_path: onde a função gravará o resultado final
    let tmp = std::env::temp_dir();
    let phrases_path = tmp.join("phrases_integration.txt");
    let stopwords_path = tmp.join("stopwords_integration.txt");
    let output_path = tmp.join("resultado_integration.txt");

    // 2) Escreve no arquivo de “phrases” (as frases de entrada):
    //    Duas linhas de exemplo:
    //      “The rain in Spain”
    //      “In Hartford, Hereford, and Hampshire”
    let mut f_phrases = std::fs::File::create(&phrases_path).unwrap();
    writeln!(f_phrases, "The rain in Spain").unwrap();
    writeln!(f_phrases, "In Hartford, Hereford, and Hampshire").unwrap();
    drop(f_phrases);

    // 3) Escreve no arquivo de “stopwords” (as stop words que serão filtradas):
    //      “in”
    //      “and”
    let mut f_stop = std::fs::File::create(&stopwords_path).unwrap();
    writeln!(f_stop, "in").unwrap();
    writeln!(f_stop, "and").unwrap();
    drop(f_stop);

    // 4) Lê o conteúdo desses dois arquivos usando a função ler_linhas_do_arquivo:
    //    - ‘phrases’ vira Vec<String> com ["The rain in Spain", "In Hartford, Hereford, and Hampshire"]
    //    - ‘stopwords_vec’ vira Vec<String> com ["in", "and"]
    let phrases = kwic_gui::ler_linhas_do_arquivo(phrases_path.to_str().unwrap()).unwrap();
    let stopwords_vec = kwic_gui::ler_linhas_do_arquivo(stopwords_path.to_str().unwrap()).unwrap();

    // 5) Converte as stop words para minúsculas e insere num HashSet<String>:
    //    – chamamos .to_lowercase() para uniformizar (apesar de já estarem em minúsculas, 
    //      garantimos qualquer variação futura)
    let stopwords: std::collections::HashSet<String> =
        stopwords_vec.into_iter().map(|s| s.to_lowercase()).collect();

    // 6) Chama gerar_rotations com esses parâmetros:
    //    - phrases: vetor de frases de entrada
    //    - stopwords: HashSet com {"in", "and"}
    //    - window_size = 2
    //    - case_sensitive = false
    let results = kwic_gui::gerar_rotations(&phrases, &stopwords, 2, false);

    // 7) Grava cada linha de “results” num arquivo de saída temporário:
    let mut f_out = std::fs::File::create(&output_path).unwrap();
    for line in &results {
        writeln!(f_out, "{}", line).unwrap();
    }
    drop(f_out);

    // 8) Lê o arquivo de saída e verifica o conteúdo:
    //    Aqui verificamos:
    //      * Que exista a rotação “Spain The” (pois “in” no começo de “in Spain The” foi filtrado).
    //      * Que a rotação “Hereford, and” apareça, porque ela não começa com “and”:
    let reader = std::io::BufReader::new(std::fs::File::open(&output_path).unwrap());
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    assert!(lines.contains(&"Spain The".to_string()));
    assert!(lines.contains(&"Hereford, and".to_string()));

    // 9) Remove os arquivos temporários para não “poluir” o diretório de teste
    std::fs::remove_file(phrases_path).unwrap();
    std::fs::remove_file(stopwords_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn integration_case_sensitive_and_window_size() {
    // 1) Cria novos arquivos temporários
    let tmp = env::temp_dir();
    let phrases_path = tmp.join("phrases_case.txt");
    let stopwords_path = tmp.join("stopwords_case.txt");
    let output_path = tmp.join("resultado_case.txt");

    // 2) Escreve frases com maiúsculas e minúsculas
    let mut f_phrases = File::create(&phrases_path).unwrap();
    writeln!(f_phrases, "Apple Banana apple").unwrap();
    writeln!(f_phrases, "banana Apple Banana").unwrap();
    drop(f_phrases);

    // 3) Cria um arquivo de stopwords vazio
    let f_stop = File::create(&stopwords_path).unwrap();
    drop(f_stop);

    // 4) Lê conteúdo
    let phrases = ler_linhas_do_arquivo(phrases_path.to_str().unwrap()).unwrap();
    let stopwords_vec = ler_linhas_do_arquivo(stopwords_path.to_str().unwrap()).unwrap();
    let stopwords: HashSet<String> = stopwords_vec.into_iter().map(|s| s.to_lowercase()).collect();

    // 5) Chama gerar_rotations com window_size = 1, case_sensitive = true
    let results = gerar_rotations(&phrases, &stopwords, 1, true);

    // 6) Grava em arquivo de saída
    let mut f_out = File::create(&output_path).unwrap();
    for line in &results {
        writeln!(f_out, "{}", line).unwrap();
    }
    drop(f_out);

    // 7) Lê o arquivo de saída e verifica a ordenação sensível a maiúsculas/minúsculas
    let reader = BufReader::new(File::open(&output_path).unwrap());
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // Quando case_sensitive = true, “Apple” deve vir antes de “apple”
    assert!(lines[0].starts_with("Apple"));
    // E deve conter alguma rotação começando com “Banana”
    assert!(lines.iter().any(|l| l.starts_with("Banana")));

    // 8) Limpeza
    fs::remove_file(phrases_path).unwrap();
    fs::remove_file(stopwords_path).unwrap();
    fs::remove_file(output_path).unwrap();
}
