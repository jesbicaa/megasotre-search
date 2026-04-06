mod product;
mod index;
mod search;
mod error;

use product::Product;
use search::SearchEngine;

fn main() {

    // Inicializa o sistema de logs
    // Permite visualizar logs no terminal
    env_logger::init();

    // Cria o motor de busca
    let mut engine = SearchEngine::new();

    // Adiciona produtos ao sistema
    engine.add_product(Product::new(1, "Notebook", "Eletronicos", "Dell"));
    engine.add_product(Product::new(2, "Smartphone", "Eletronicos", "Samsung"));
    engine.add_product(Product::new(3, "Camiseta", "Vestuario", "Nike"));

    // 🔎 Teste de busca inteligente (com erro proposital)
    match engine.smart_search("notebok") {

        // Caso encontre resultados
        Ok(results) => {
            println!("Resultados encontrados:");

            for p in results {
                println!("{:?}", p);
            }
        }

        // Caso ocorra erro
        Err(e) => println!("Erro: {}", e),
    }
}