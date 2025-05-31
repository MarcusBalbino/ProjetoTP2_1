// src/lib.rs

use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

/// Lê todas as linhas de um arquivo de texto e retorna um Vec<String>
pub fn ler_linhas_do_arquivo(caminho: &str) -> io::Result<Vec<String>> {
    let f = File::open(caminho)?;
    BufReader::new(f).lines().collect()
}

/// Gera rotações, filtra stopwords, aplica truncamento por window_size e retorna as strings únicas e ordenadas.
///   - `phrases`: vetor de frases (cada frase é uma String).
///   - `stopwords`: conjunto de stopwords em minúsculas.
///   - `window_size`: número máximo de palavras que cada “trecho” de rotação deve ter (se a rotação for maior, corta).
///   - `case_sensitive`: se `true`, a ordenação final faz distinção de maiúsculas/minúsculas; se `false`, converte tudo para lowercase ao ordenar.
pub fn gerar_rotations(
    phrases: &[String],
    stopwords: &HashSet<String>,
    window_size: usize,
    case_sensitive: bool,
) -> Vec<String> {
    // 1) Para cada frase, gera todas as rotações, filtra stopwords e mantém apenas as primeiras window_size palavras.
    let rotations_per_phrase: Vec<Vec<String>> = phrases
        .par_iter()                              // paraleliza sobre cada frase
        .map(|phrase| {
            let words: Vec<&str> = phrase.split_whitespace().collect();
            let n = words.len();
            let mut contexts: Vec<String> = Vec::with_capacity(n);

            for i in 0..n {
                // 1.1) gera a rotação completa “shift” das palavras
                let mut rot = Vec::with_capacity(n);
                rot.extend_from_slice(&words[i..]);
                rot.extend_from_slice(&words[..i]);

                // 1.2) filtra pelo caso da primeira palavra ser stopword
                let first_word = rot[0].to_lowercase();
                if stopwords.contains(&first_word) {
                    continue;
                }

                // 1.3) pega apenas window_size palavras da rotação
                let take = window_size.min(rot.len());
                let snippet = rot[..take].join(" ");
                contexts.push(snippet);
            }
            contexts
        })
        .collect();

    // 2) “Flatten” (acha todas as Vec<String> em um único Vec<String>) e, em paralelo, remove duplicatas
    let mut final_contexts: Vec<String> = rotations_per_phrase
        .into_par_iter()
        .reduce(|| Vec::new(), |mut acc, mut vec| {
            acc.append(&mut vec);
            acc
        })
        .into_par_iter()
        .collect::<HashSet<_>>()  // remove duplicatas
        .into_par_iter()
        .collect::<Vec<String>>();

    // 3) Ordena conforme case_sensitive
    if case_sensitive {
        final_contexts.sort();
    } else {
        final_contexts.par_sort_by_key(|s| s.to_lowercase());
    }

    final_contexts
}