mod gui;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

/// Lê todas as linhas de um arquivo
pub fn ler_linhas_do_arquivo(caminho: &str) -> io::Result<Vec<String>> {
    let f = File::open(caminho)?;
    BufReader::new(f).lines().collect()
}

/// Gera rotações truncadas pela janela, filtra stopwords e retorna vetores únicos e ordenados
/// case_sensitive: se true, ordena diferenciado por maiúsculas/minúsculas
/// window_size: número máximo de palavras na rotação (truncada)
pub fn gerar_rotations(
    phrases: &[String],
    stopwords: &HashSet<String>,
    window_size: usize,
    case_sensitive: bool,
) -> Vec<String> {
    // 1) Para cada frase, gerar todas as rotações, filtrar stopwords e truncar janela
    let rotations_per_phrase: Vec<Vec<String>> = phrases
        .par_iter()
        .map(|phrase| {
            let words: Vec<&str> = phrase.split_whitespace().collect();
            let n = words.len();
            let mut contexts: Vec<String> = Vec::with_capacity(n);

            for i in 0..n {
                // gera rotação completa
                let mut rot = Vec::with_capacity(n);
                rot.extend_from_slice(&words[i..]);
                rot.extend_from_slice(&words[..i]);
                // filtra stopwords
                let first_word = rot[0].to_lowercase();
                if stopwords.contains(&first_word) {
                    continue;
                }
                // pega apenas as primeiras window_size palavras da rotação
                let take = window_size.min(rot.len());
                let snippet = rot[..take].join(" ");
                contexts.push(snippet);
            }
            contexts
        })
        .collect();

    // 2) Flatten e remover duplicatas em paralelo
    let mut final_contexts: Vec<String> = rotations_per_phrase
        .into_par_iter()
        .reduce(|| Vec::new(), |mut acc, mut vec| { acc.append(&mut vec); acc })
        .into_par_iter()
        .collect::<HashSet<_>>()
        .into_par_iter()
        .collect::<Vec<String>>();

    // 3) Ordenação de acordo com case_sensitive
    if case_sensitive {
        final_contexts.sort();
    } else {
        final_contexts.par_sort_by_key(|s| s.to_lowercase());
    }

    final_contexts
}

fn main() {
    // inicializa GUI que chamará gerar_rotations
    gui::run_gui();
}