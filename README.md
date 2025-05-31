# KWIC em Rust
## Estilo de progração: Double Map Reduce

Este repositório implementa o KWIC (Key Word In Context) em Rust, utilizando o estilo Double Map Reduce, e inclui uma interface gráfica (GUI) com egui/eframe, além de uma suíte de testes (unitários e de integração).

### Participantes:
Henrique Carvalho Wolski - 231013627  
Lucas Frutuoso Alvarenga - 221007152  
Rodrigo Vieira Gusmão - 232036466  
Lucas Teles Leiro - 211066131  
Marcus Vinicius Balbino Cavalcante - 160135974  

### O link do vídeo para o GoogleDrive
https://drive.google.com/drive/folders/1D3CjMrg-fIwYatb9ToyL4D-HZ-Zau9ri?hl=pt-br

## Estrutura do Projeto

```bash
kwic_gui
├── Cargo.toml         # Dependências e metadados do projeto
├── src
│   ├── lib.rs         # Lógica principal do KWIC (funções de leitura e geração de rotações)
│   ├── main.rs        # Arquivo que apenas inicializa a GUI
│   └── gui.rs         # Código da interface gráfica (chama funções do lib.rs)
└── tests
    ├── kwic_unit_tests.rs         # Testes unitários para geracao de rotações e leitura de arquivos
    └── kwic_integration_tests.rs  # Testes de integração que geram arquivos temporários
```
## Compilação e Execução

Para compilar o projeto (construir o binário sem rodá‑lo), utilize:

```bash
cargo build
```

Para executar a aplicação (que abrirá a janela da GUI), utilize:
```bash
cargo run
```

> Observação: Como o main.rs chama apenas a GUI (gui::run_gui()), nenhuma lógica de KWIC é executada no terminal—ela só é invocada pela interface.

## Testes
Todas as funções principais (em lib.rs) possuem testes unitários e de integração. Quando quiser validar se tudo está funcionando corretamente, rode o comando abaixo:

```bash
cargo test
```

Isso fará com que:

- O Cargo compile a biblioteca (lib.rs) e o binário (main.rs/gui.rs).

- Rode todos os testes de integração encontrados em tests/kwic_unit_tests.rs e tests/kwic_integration_tests.rs.

### Executar apenas testes unitários

Se desejar rodar apenas os testes unitários (aquele arquivo dedicado em tests/kwic_unit_tests.rs), use:

```bash
cargo test --test kwic_unit_tests
```

### Executar apenas testes de integração

Para rodar apenas os testes de integração (em tests/kwic_integration_tests.rs), utilize:

```bash
cargo test --test kwic_integration_tests
```

## Dependências Principais

Veja em Cargo.toml as dependências usadas:

- rayon = "1.7" (para iteradores paralelos em gerar_rotations).

- eframe = "0.24" e egui = "0.24" (para construir a interface gráfica).

Se preferir executar o projeto sem a GUI, basta rodar os testes ou invocar diretamente as funções em lib.rs a partir de outro código.
