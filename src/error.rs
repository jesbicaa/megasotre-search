use std::fmt;

// Enum que define os possíveis erros do sistema de busca
#[derive(Debug)]
pub enum SearchError {
    EmptyQuery, // Quando a busca está vazia
    NotFound,   // Quando nenhum resultado é encontrado
}

// Implementação para exibir o erro de forma amigável
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchError::EmptyQuery => write!(f, "A busca não pode estar vazia"),
            SearchError::NotFound => write!(f, "Nenhum produto encontrado"),
        }
    }
}