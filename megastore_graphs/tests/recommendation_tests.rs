#[cfg(test)]
mod tests {
    use petgraph::graph::Graph;
    use petgraph::Undirected;
    use megastore_graphs::Produto;
    use megastore_graphs::{connect_similar_products, calculate_similarity_weight, search_by_name, search_by_category};

    #[test]
    fn test_conexao_de_produtos_similares() {
        let mut grafo: Graph<Produto, (), Undirected> = Graph::new_undirected();

        let p1 = Produto { id: 1, nome: "Mouse Gamer".into(), categoria: "Periféricos".into(), preco: 200.0, popularidade: 30 };
        let p2 = Produto { id: 2, nome: "Teclado Gamer".into(), categoria: "Periféricos".into(), preco: 210.0, popularidade: 27 };
        let p3 = Produto { id: 3, nome: "Notebook".into(), categoria: "Informática".into(), preco: 3000.0, popularidade: 80 };

        let n1 = grafo.add_node(p1);
        let n2 = grafo.add_node(p2);
        let n3 = grafo.add_node(p3);

        connect_similar_products(&mut grafo);

        assert!(grafo.contains_edge(n1, n2));
        assert!(!grafo.contains_edge(n1, n3));
        assert!(!grafo.contains_edge(n2, n3));
    }

    #[test]
    fn test_calculate_similarity_weight() {
        let produto1 = Produto { id: 1, nome: "Produto A".into(), categoria: "Categoria X".into(), preco: 100.0, popularidade: 50 };
        let produto2 = Produto { id: 2, nome: "Produto B".into(), categoria: "Categoria X".into(), preco: 105.0, popularidade: 55 };

        let peso = calculate_similarity_weight(&produto1, &produto2);
        assert_eq!(peso, 9);
    }

    #[test]
    fn test_search_by_existing_name() {
        let mut grafo = Graph::<Produto, (), Undirected>::new_undirected();

        let p = Produto { id: 1, nome: "Câmera HD".into(), categoria: "Eletrônicos".into(), preco: 500.0, popularidade: 40 };
        let node = grafo.add_node(p.clone());

        let mut mapa_nome = std::collections::HashMap::new();
        mapa_nome.insert(p.nome.clone(), vec![node]);

        let resultados = search_by_name(&grafo, &mapa_nome, "Câmera HD");
        assert_eq!(resultados.len(), 1);
        assert_eq!(resultados[0].nome, "Câmera HD");
    }

    #[test]
    fn test_search_by_nonexistent_name() {
        let grafo = Graph::<Produto, (), Undirected>::new_undirected();
        let mapa_nome = std::collections::HashMap::new();

        let resultados = search_by_name(&grafo, &mapa_nome, "Produto Fantasma");
        assert!(resultados.is_empty());
    }

    #[test]
    fn test_search_by_category() {
        let mut grafo = Graph::<Produto, (), Undirected>::new_undirected();

        let p = Produto { id: 10, nome: "Fone Bluetooth".into(), categoria: "Áudio".into(), preco: 300.0, popularidade: 60 };
        let node = grafo.add_node(p.clone());

        let mut mapa_categoria = std::collections::HashMap::new();
        mapa_categoria.insert(p.categoria.clone(), vec![node]);

        let resultados = search_by_category(&grafo, &mapa_categoria, "Áudio");
        assert_eq!(resultados.len(), 1);
        assert_eq!(resultados[0].nome, "Fone Bluetooth");
    }
}
