// src/main.rs

mod gui;   // importa o módulo gui.rs

fn main() {
    // chama a GUI, que internamente fará ler_linhas_do_arquivo() e gerar_rotations()
    gui::run_gui();
}