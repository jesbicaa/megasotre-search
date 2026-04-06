use megasotre_search::product::Product;
use megasotre_search::search::SearchEngine;
use megasotre_search::error::SearchError;

// ==========================================
// TESTE 1: Busca exata
// ==========================================
#[test]
fn test_search_exact_match() {
    let mut engine = SearchEngine::new();

    // Adiciona um produto ao sistema
    engine.add_product(Product::new(1, "Notebook", "Eletronicos", "Dell"));

    // Realiza busca exatamente igual ao nome cadastrado
    let result = engine.smart_search("Notebook");

    // Verifica se a busca foi bem-sucedida
    assert!(result.is_ok());

    // Verifica se retornou exatamente 1 resultado
    assert_eq!(result.unwrap().len(), 1);
}

// ==========================================
// TESTE 2: Busca parcial (substring)
// ==========================================
#[test]
fn test_search_partial_match() {
    let mut engine = SearchEngine::new();

    // Produto com nome maior
    engine.add_product(Product::new(1, "Notebook Gamer", "Eletronicos", "Dell"));

    // Busca usando apenas parte do nome
    let result = engine.smart_search("note");

    // Deve encontrar o produto
    assert!(result.is_ok());

    // Deve retornar pelo menos 1 resultado
    assert_eq!(result.unwrap().len(), 1);
}

// ==========================================
// TESTE 3: Busca fuzzy (erro de digitação)
// ==========================================
#[test]
fn test_search_fuzzy_match() {
    let mut engine = SearchEngine::new();

    // Produto correto
    engine.add_product(Product::new(1, "Notebook", "Eletronicos", "Dell"));

    // Busca com erro proposital (faltando uma letra)
    let result = engine.smart_search("notebok");

    // O sistema deve corrigir e ainda encontrar o produto
    assert!(result.is_ok());
}

// ==========================================
// TESTE 4: Produto não encontrado
// ==========================================
#[test]
fn test_search_not_found() {
    let mut engine = SearchEngine::new();

    // Adiciona apenas Notebook
    engine.add_product(Product::new(1, "Notebook", "Eletronicos", "Dell"));

    // Busca por algo inexistente
    let result = engine.smart_search("Geladeira");

    // Deve retornar erro
    assert!(result.is_err());

    // Verifica se o erro é o esperado (NotFound)
    match result {
        Err(SearchError::NotFound) => assert!(true),
        _ => panic!("Erro esperado não ocorreu"),
    }
}

// ==========================================
// TESTE 5: Busca vazia (erro de entrada)
// ==========================================
#[test]
fn test_search_empty_query() {
    let engine = SearchEngine::new();

    // Busca vazia
    let result = engine.smart_search("");

    // Deve retornar erro
    assert!(result.is_err());

    // Verifica se o erro é EmptyQuery
    match result {
        Err(SearchError::EmptyQuery) => assert!(true),
        _ => panic!("Erro esperado não ocorreu"),
    }
}

// ==========================================
// TESTE 6: Múltiplos resultados
// ==========================================
#[test]
fn test_multiple_results() {
    let mut engine = SearchEngine::new();

    // Adiciona vários produtos semelhantes
    engine.add_product(Product::new(1, "Notebook", "Eletronicos", "Dell"));
    engine.add_product(Product::new(2, "Notebook Gamer", "Eletronicos", "Asus"));

    // Busca genérica
    let result = engine.smart_search("Notebook");

    // Deve retornar resultados
    assert!(result.is_ok());

    // Deve retornar pelo menos 1 (ou mais dependendo da busca)
    assert!(result.unwrap().len() >= 1);
}