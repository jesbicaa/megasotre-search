use crate::index::ProductIndex;
use crate::product::Product;
use crate::error::SearchError;

// Biblioteca de logs
use log::{info, warn};

// Biblioteca para cálculo de similaridade (distância de Levenshtein)
use strsim::levenshtein;

// Estrutura principal do mecanismo de busca
pub struct SearchEngine {
    pub index: ProductIndex, // Índice contendo os produtos
}

impl SearchEngine {

    // Cria um novo motor de busca
    pub fn new() -> Self {
        Self {
            index: ProductIndex::new(),
        }
    }

    // Adiciona um produto ao sistema
    pub fn add_product(&mut self, product: Product) {
        // Log informando inserção
        info!("Adicionando produto: {}", product.name);

        self.index.add_product(product);
    }

    // 🔎 Função principal de busca inteligente
    pub fn smart_search(&self, query: &str) -> Result<Vec<&Product>, SearchError> {

        // Verifica se a busca está vazia
        if query.trim().is_empty() {
            warn!("Busca vazia recebida");
            return Err(SearchError::EmptyQuery);
        }

        // Normaliza para minúsculo
        let query = query.to_lowercase();

        // Vetor que armazenará os resultados
        let mut results: Vec<&Product> = Vec::new();

        info!("Iniciando busca por: {}", query);

        // =========================
        // 1️⃣ BUSCA EXATA (mais rápida)
        // =========================
        if let Some(products) = self.index.by_name.get(&query) {
            info!("Busca exata encontrada");
            return Ok(products.iter().collect());
        }

        // =========================
        // 2️⃣ BUSCA PARCIAL
        // =========================
        for (key, products) in &self.index.by_name {

            // Verifica se o nome contém o termo buscado
            if key.contains(&query) {
                info!("Match parcial encontrado: {}", key);

                for p in products {
                    results.push(p);
                }
            }
        }

        // =========================
        // 3️⃣ BUSCA FUZZY (similaridade)
        // =========================
        for (key, products) in &self.index.by_name {

            // Calcula a distância entre palavras
            let distance = levenshtein(&query, key);

            // Se for pequeno (até 2 erros), considera similar
            if distance <= 2 {
                info!(
                    "Match fuzzy encontrado: {} (distância: {})",
                    key, distance
                );

                for p in products {
                    results.push(p);
                }
            }
        }

        // =========================
        // VALIDAÇÃO FINAL
        // =========================
        if results.is_empty() {
            warn!("Nenhum resultado encontrado para: {}", query);
            return Err(SearchError::NotFound);
        }

        Ok(results)
    }
}