// src/gui.rs
use eframe::{egui, App, Frame, NativeOptions};
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use crate::{ler_linhas_do_arquivo, gerar_rotations};

pub fn run_gui() {
    let native_options = NativeOptions::default();
    let _ = eframe::run_native(
        "KWIC GUI",
        native_options,
        Box::new(|_cc| Box::new(KwicApp::default())),
    );
}

struct KwicApp {
    use_phrases_file: bool,
    phrases_input: String,
    phrases_path: String,
    use_stopwords_file: bool,
    stopwords_input: String,
    stopwords_path: String,
    output_path: String,
    result: Vec<String>,
    error: Option<String>,
    // novas opções
    case_sensitive: bool,
    window_size: usize,
}

impl Default for KwicApp {
    fn default() -> Self {
        Self {
            use_phrases_file: true,
            phrases_input: String::new(),
            phrases_path: String::from("src/Phrases.txt"),
            use_stopwords_file: true,
            stopwords_input: String::new(),
            stopwords_path: String::from("src/StopWords.txt"),
            output_path: String::from("src/resultado.txt"),
            result: Vec::new(),
            error: None,
            case_sensitive: false,
            window_size: 2,
        }
    }
}

impl App for KwicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("KWIC - Key Word In Context");

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.use_phrases_file, true, "Usar arquivo de frases");
                ui.radio_value(&mut self.use_phrases_file, false, "Entrada manual");
            });
            if self.use_phrases_file {
                ui.label("Caminho do arquivo de frases:");
                ui.text_edit_singleline(&mut self.phrases_path);
            } else {
                ui.label("Digite as frases (uma por linha):");
                ui.add(
                    egui::TextEdit::multiline(&mut self.phrases_input)
                        .desired_rows(5)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.use_stopwords_file, true, "Usar arquivo de stopwords");
                ui.radio_value(&mut self.use_stopwords_file, false, "Entrada manual");
            });
            if self.use_stopwords_file {
                ui.label("Caminho do arquivo de stopwords:");
                ui.text_edit_singleline(&mut self.stopwords_path);
            } else {
                ui.label("Digite as stopwords (uma por linha):");
                ui.add(
                    egui::TextEdit::multiline(&mut self.stopwords_input)
                        .desired_rows(5)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Ordenação:");
                ui.radio_value(&mut self.case_sensitive, true, "Sensible a Maiúsc/Minúsc");
                ui.radio_value(&mut self.case_sensitive, false, "Insensible");
            });
            ui.horizontal(|ui| {
                ui.label("Tamanho da janela de contexto:");
                ui.add(egui::DragValue::new(&mut self.window_size).clamp_range(1..=1000));
            });

            if ui.button("Processar").clicked() {
                self.error = None;
                let phrases: Vec<String> = if self.use_phrases_file {
                    match ler_linhas_do_arquivo(&self.phrases_path) {
                        Ok(v) => v,
                        Err(e) => {
                            self.error = Some(format!("Erro ao ler frases: {}", e));
                            Vec::new()
                        }
                    }
                } else {
                    self.phrases_input.lines().map(str::to_string).collect()
                };
                let stopwords: HashSet<String> = if self.use_stopwords_file {
                    match ler_linhas_do_arquivo(&self.stopwords_path) {
                        Ok(v) => v.into_iter().map(|s| s.to_lowercase()).collect(),
                        Err(e) => {
                            self.error = Some(format!("Erro ao ler stopwords: {}", e));
                            HashSet::new()
                        }
                    }
                } else {
                    self.stopwords_input.lines().map(|s| s.to_lowercase()).collect()
                };
                if self.error.is_none() {
                    self.result = gerar_rotations(&phrases, &stopwords, self.window_size, self.case_sensitive);
                }
            }

            ui.separator();
            ui.label("Resultado:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in &self.result {
                    ui.label(line);
                }
            });

            ui.separator();
            ui.label("Salvar resultado em arquivo:");
            ui.text_edit_singleline(&mut self.output_path);
            if ui.button("Salvar").clicked() {
                match fs::File::create(&self.output_path) {
                    Ok(mut f) => {
                        for line in &self.result {
                            if let Err(e) = writeln!(f, "{}", line) {
                                self.error = Some(format!("Erro ao escrever arquivo: {}", e));
                                break;
                            }
                        }
                    }
                    Err(e) => self.error = Some(format!("Erro ao criar arquivo: {}", e)),
                }
            }

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, err);
            }
        });
    }
}
