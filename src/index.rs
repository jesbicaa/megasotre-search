use std::collections::HashMap;
use crate::product::Product;

// Estrutura responsável pela indexação dos produtos
pub struct ProductIndex {
    // HashMap para busca por nome
    // Chave: nome do produto (em minúsculo)
    // Valor: lista de produtos com esse nome
    pub by_name: HashMap<String, Vec<Product>>,

    // HashMap para busca por categoria
    pub by_category: HashMap<String, Vec<Product>>,
}

impl ProductIndex {
    // Inicializa o índice vazio
    pub fn new() -> Self {
        Self {
            by_name: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    // Adiciona um produto aos índices
    pub fn add_product(&mut self, product: Product) {

        // Indexa pelo nome (convertendo para minúsculo)
        self.by_name
            .entry(product.name.to_lowercase()) // acessa ou cria a chave
            .or_insert(Vec::new())              // cria vetor se não existir
            .push(product.clone());             // adiciona o produto

        // Indexa pela categoria
        self.by_category
            .entry(product.category.to_lowercase())
            .or_insert(Vec::new())
            .push(product);
    }
}