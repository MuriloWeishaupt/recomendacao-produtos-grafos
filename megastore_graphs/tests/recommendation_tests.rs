#[cfg(test)]
mod tests {
    use petgraph::graph::Graph;
    use petgraph::Undirected;
    use crate::models::Produto;
    use crate::{connect_similar_products};

    #[test]
    fn test_conexao_de_produtos_similares() {
        let mut grafo: Graph<Produto, (), Undirected> = Graph::new_undirected();

        // Produtos de teste
        let p1 = Produto {
            id: 1,
            nome: "Mouse Gamer".into(),
            categoria: "Periféricos".into(),
            preco: 200.0,
            popularidade: 30,
        };

        let p2 = Produto {
            id: 2,
            nome: "Teclado Gamer".into(),
            categoria: "Periféricos".into(),
            preco: 210.0,
            popularidade: 27,
        };

        let p3 = Produto {
            id: 3,
            nome: "Notebook".into(),
            categoria: "Informática".into(),
            preco: 3000.0,
            popularidade: 80,
        };

        // Adiciona os produtos ao grafo
        let n1 = grafo.add_node(p1);
        let n2 = grafo.add_node(p2);
        let n3 = grafo.add_node(p3);

        // Conecta produtos similares
        connect_similar_products(&mut grafo);

        // Testa se os produtos certos foram conectados
        assert!(grafo.contains_edge(n1, n2), "Mouse e Teclado gamer deveriam estar conectados");
        assert!(!grafo.contains_edge(n1, n3), "Mouse e Notebook não deveriam estar conectados");
        assert!(!grafo.contains_edge(n2, n3), "Teclado e Notebook não deveriam estar conectados");
    }
}
