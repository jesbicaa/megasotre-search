# MegaStore Search 🚀

![Status: concluído](https://img.shields.io/badge/status-concluído-brightgreen)
![License: academic](https://img.shields.io/badge/license-academic-blue)
![Tests: passing](https://img.shields.io/badge/tests-passing-success)

## O motor de busca inteligente para e-commerce, desenvolvido em Rust.

---

Um sistema de busca de alta performance, projetado para catálogos de produtos de e-commerce. Resolve desafios comuns de velocidade e relevância em grandes volumes de dados através de técnicas modernas de indexação e algoritmos de busca.

### 📌 Funcionalidades

- 🔎 **Busca exata**: Resultados idênticos ao termo pesquisado.
- 🔍 **Busca parcial**: Encontra termos contidos em substrings.
- 🧠 **Busca fuzzy**: Correção automática de erros ortográficos com algoritmo **Levenshtein**.
- 📦 **Indexação eficiente**: Uso otimizado de `HashMap` para acesso rápido.
- 📊 **Logs de execução**: Monitoramento em tempo real do comportamento do sistema.
- ⚠️ **Tratamento de erros**: Robusto contra falhas de entrada ou processamento.
- 🧪 **Testes automatizados**: Garantia de integridade do código.

### 🛠️ Tecnologias

- **Linguagem**: Rust
- **Coleções**: `HashMap` (std::collections)
- **Logging**: `log` e `env_logger` (para traces e auditoria)
- **Algoritmos de String**: `strsim` (para cálculo de Levenshtein)

### 📂 Estrutura do Projeto

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
```

### ▶️ Como Executar

Certifique-se de ter o **Rust/Cargo** instalado.

1. **Compilar o projeto:**
   ```bash
   cargo build
   
2. **Rodar a aplicação**
   ```bash
   cargo run


3. **Executar Testes:**
   ```bash
   cargo test

---

### 📊 Configuração de Logs
Para visualizar os logs de execução, defina a variável de ambiente:

* **Linux/macOS:**
  ```bash
  RUST_LOG=info cargo run

* **Windows (PowerShell):**
  ```bash
  $env:RUST_LOG="info"
  cargo run


---

### 🔎 Exemplos de Uso
```rust
engine.smart_search("Notebook");   // Busca exata
engine.smart_search("note");       // Busca parcial
engine.smart_search("notebok");    // Busca fuzzy (corrige para Notebook)
```

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
