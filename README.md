# MegaStore Search 🚀

![Status](https://img.shields.io/badge/status-concluído-brightgreen)
![License](https://img.shields.io/badge/license-academic-blue)
![Tests](https://img.shields.io/badge/tests-passing-success)

Sistema de busca otimizado para um catálogo de produtos de e-commerce, desenvolvido em **Rust**, com foco em **desempenho, escalabilidade e precisão**. 

O projeto resolve problemas comuns de sistemas tradicionais, como lentidão e baixa relevância nos resultados através de técnicas modernas de indexação e algoritmos de busca.

---

## 📌 Descrição

Este motor de busca foi construído para lidar com grandes volumes de dados de produtos, garantindo que o usuário encontre o que precisa, mesmo com pequenas variações na escrita.

### 🚀 Funcionalidades
- 🔎 **Busca exata**: Resultados idênticos ao termo pesquisado.
- 🔍 **Busca parcial**: Encontra termos contidos em substrings.
- 🧠 **Busca fuzzy**: Correção de erros ortográficos utilizando o algoritmo de **Levenshtein**.
- 📦 **Indexação eficiente**: Uso de `HashMap` para acesso rápido.
- 📊 **Logs de execução**: Monitoramento em tempo real do comportamento do sistema.
- ⚠️ **Tratamento de erros**: Robusto contra falhas de entrada ou processamento.
- 🧪 **Testes automatizados**: Garantia de integridade do código.

---

## 🛠️ Tecnologias

- **Linguagem:** Rust
- **Coleções:** `HashMap` (std::collections)
- **Logging:** `log` e `env_logger`
- **Algoritmos de String:** `strsim` (Levenshtein)

---

## 📂 Estrutura do Projeto

```text
megastore-search/
├── src/
│   ├── main.rs
│   ├── product.rs
│   ├── index.rs
│   ├── search.rs
│   └── error.rs
├── tests/
│   └── search_test.rs
├── Cargo.toml
└── README.md
▶️ Como Executar
Certifique-se de ter o Rust/Cargo instalado.

Compilar o projeto:

Bash
cargo build
Rodar a aplicação:

Bash
cargo run
Executar Testes:

Bash
cargo test
📊 Configuração de Logs
Para visualizar os logs durante a execução:

Linux/macOS:

Bash
RUST_LOG=info cargo run
Windows (PowerShell):

PowerShell
$env:RUST_LOG="info"
cargo run
🔎 Exemplos de Uso
Rust
engine.smart_search("Notebook");   // Busca exata
engine.smart_search("note");       // Busca parcial
engine.smart_search("notebok");    // Busca fuzzy (corrige para Notebook)
🧠 Arquitetura e Algoritmos
O sistema segue o fluxo:
[Product] → [Index (HashMap)] → [Search Engine] → [Resultados]

⚙️ Detalhes Técnicos
HashMap: Utilizado para indexação com complexidade média O(1).

Vec: Armazenamento dinâmico dos dados brutos.

Busca Híbrida: O motor prioriza buscas exatas antes de aplicar algoritmos de similaridade (Fuzzy), otimizando o tempo de resposta.

📈 Desempenho
Alta velocidade de busca e baixa latência.

Arquitetura escalável para grandes volumes de dados.

🔮 Melhorias Futuras
[ ] 🔎 Autocomplete: Sugestões em tempo real.

[ ] 📊 Ranking de relevância: Ordenar por popularidade ou preço.

[ ] 🌐 API REST: Expor o serviço via Actix-web ou Rocket.

[ ] 🧠 Sistema de recomendação: Uso de grafos para produtos relacionados.

👩‍💻 Autora
Jéssica Bianca da Silva RA: 160289

Disciplina: Data Structure Strategy and Implementation

📄 Licença
Projeto desenvolvido para fins acadêmicos.
