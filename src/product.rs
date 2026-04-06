// Estrutura que representa um produto no sistema
#[derive(Clone, Debug)]
pub struct Product {
    pub id: u32,          // Identificador único do produto
    pub name: String,     // Nome do produto
    pub category: String, // Categoria (ex: Eletrônicos, Vestuário)
    pub brand: String,    // Marca do produto
}

impl Product {
    // Função construtora para facilitar a criação de produtos
    pub fn new(id: u32, name: &str, category: &str, brand: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            category: category.to_string(),
            brand: brand.to_string(),
        }
    }
}