use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashSet;


// Função para criar um arquivo de teste temporário
pub fn criar_arquivo_de_teste(nome: &str, conteudo: &str) {
    fs::write(nome, conteudo).expect("Falha ao criar arquivo de teste");
}

// Função para remover o arquivo após o teste
pub fn remover_arquivo_de_teste(nome: &str) {
    fs::remove_file(nome).expect("Falha ao remover arquivo de teste");
}

pub fn ler_linhas_do_arquivo(caminho: &str) -> io::Result<Vec<String>> {
    let file = File::open(caminho)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() {
    let read_lines_phrases = ler_linhas_do_arquivo("src/Phrases.txt")
        .expect("Erro ao ler o arquivo");
    let read_lines_stopwords: Vec<String> = ler_linhas_do_arquivo("src/StopWords.txt")
        .expect("Erro ao ler o arquivo");

    let stopwords: HashSet<String> = read_lines_stopwords
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect();

    let rotations_by_phrase: Vec<Vec<String>> = read_lines_phrases
        .into_iter()
        .map(|phrase| {
            let words: Vec<&str> = phrase.split_whitespace().collect();
            let n = words.len();

            let rotations: Vec<String> = (0..n)
                .map(|i| {
                    let mut rot = Vec::with_capacity(n);
                    rot.extend_from_slice(&words[i..]);
                    rot.extend_from_slice(&words[..i]);
                    rot.join(" ")
                })
                .collect();

           
            rotations
                .into_iter()
                .filter(|rotation| {
                    let first_word = rotation
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_lowercase();
                    !stopwords.contains(&first_word)
                })
                .collect::<Vec<String>>() 
        })
        .collect();

   
    let final_rotations: Vec<String> = rotations_by_phrase
        .into_iter()
        .reduce(|mut acc, mut vec| {
            acc.append(&mut vec);
            acc
        })
        .unwrap_or_default() 
        .into_iter()
        .collect::<HashSet<_>>() 
        .into_iter()
        .collect::<Vec<String>>();

    let mut sorted_rotations = final_rotations;
    sorted_rotations.sort();

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("src/resultado.txt")
        .expect("Erro ao criar o arquivo de saída");

    for line in sorted_rotations {
        writeln!(output_file, "{}", line).expect("Erro ao escrever no arquivo");
    }

    println!("Arquivo src/resultado.txt criado com sucesso!");
}

// Módulo de testes (só compila quando rodamos testes)
#[cfg(test)]
mod tests {
    use super::*; // Importa tudo do módulo pai


    #[test]
    fn test_leitura_sucesso() {
        let resultado = ler_linhas_do_arquivo("src/Phrases.txt");
        assert!(resultado.is_ok(), "A função deveria retornar Ok");
    }

    #[test]
    fn test_leitura_arquivo_vazio(){
        criar_arquivo_de_teste("arquivo_vazio", "");
        let resultado = ler_linhas_do_arquivo("arquivo_vazio");
        let linhas = resultado.unwrap();
        assert!(linhas.is_empty(), "O arquivo não está vazio");
        remover_arquivo_de_teste("arquivo_vazio");
    }

    #[test]
    fn test_leitura_de_linha(){
        criar_arquivo_de_teste("arquivo_uma_linha", "The fox is Brown");
        let resultado = ler_linhas_do_arquivo("arquivo_uma_linha");
        let linhas = resultado.unwrap();
        assert_eq!(linhas[0], "The fox is Brown", "Conteúdo inesperado na primeira linha");
        remover_arquivo_de_teste("arquivo_uma_linha");
    }
    
    #[test]
    fn teste_arquivo_inexistente() {
        let resultado = ler_linhas_do_arquivo("nao_existe.txt");
        assert!(resultado.is_err(), "A função deveria retornar Err");
    }
}
