mod models;
// pub use models::Produto;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use megastore_graphs::models::Produto;
use megastore_graphs::{connect_similar_products, search_products, search_by_name, search_by_category, recommended_products};

// Função principal
fn main() {
    let mut grafo: Graph<Produto, (), Undirected> = Graph::new_undirected();
    let mut mapa_nome: HashMap<String, Vec<NodeIndex>> = HashMap::new();
    let mut mapa_categoria: HashMap<String, Vec<NodeIndex>> = HashMap::new();

    // Produtos base
    let produtos = vec![
        Produto { id: 1, nome: "Mouse Gamer".into(), categoria: "Periféricos".into(), preco: 199.90, popularidade: 25 },
        Produto { id: 2, nome: "Headset Gamer".into(), categoria: "Periféricos".into(), preco: 539.90, popularidade: 15 },
        Produto { id: 3, nome: "Teclado sem fio".into(), categoria: "Periféricos".into(), preco: 1299.90, popularidade: 40 },
        Produto { id: 4, nome: "Notebook Lenovo".into(), categoria: "Informática".into(), preco: 2999.90, popularidade: 60 },
        Produto { id: 5, nome: "Webcam HD".into(), categoria: "Periféricos".into(), preco: 250.00, popularidade: 22 },
        Produto { id: 6, nome: "Monitor 24\"".into(), categoria: "Informática".into(), preco: 849.90, popularidade: 35 },
        Produto { id: 7, nome: "Cadeira Gamer".into(), categoria: "Móveis".into(), preco: 899.99, popularidade: 28 },
        Produto { id: 8, nome: "SSD 1TB".into(), categoria: "Armazenamento".into(), preco: 599.90, popularidade: 32 },
        Produto { id: 9, nome: "HD Externo 2TB".into(), categoria: "Armazenamento".into(), preco: 379.90, popularidade: 27 },
        Produto { id: 10, nome: "Placa de Vídeo RTX 3060".into(), categoria: "Hardware".into(), preco: 2499.00, popularidade: 50 },
        Produto { id: 11, nome: "Fonte 750W".into(), categoria: "Hardware".into(), preco: 499.90, popularidade: 20 },
        Produto { id: 12, nome: "Mesa Digitalizadora".into(), categoria: "Design".into(), preco: 699.00, popularidade: 18 },
        Produto { id: 13, nome: "Mousepad RGB".into(), categoria: "Periféricos".into(), preco: 129.90, popularidade: 22 },
        Produto { id: 14, nome: "MacBook Air M1".into(), categoria: "Informática".into(), preco: 6999.00, popularidade: 45 },
        Produto { id: 15, nome: "Impressora Multifuncional".into(), categoria: "Impressão".into(), preco: 799.00, popularidade: 30 },
        Produto { id: 16, nome: "Cabo HDMI 2m".into(), categoria: "Acessórios".into(), preco: 29.90, popularidade: 12 },
        Produto { id: 17, nome: "Adaptador USB-C".into(), categoria: "Acessórios".into(), preco: 39.90, popularidade: 10 },
    ];

    // Inserção no grafo e mapas
    for produto in produtos {
        let node = grafo.add_node(produto);
        let nome = grafo[node].nome.clone();
        let categoria = grafo[node].categoria.clone();

        mapa_nome.entry(nome).or_default().push(node);
        mapa_categoria.entry(categoria).or_default().push(node);
    }

    // Conecta os produtos semelhantes após todos serem inseridos
    connect_similar_products(&mut grafo);

    // Testes de funcionalidades
    println!("\n--- Produtos no Grafo ---");
    for node_index in grafo.node_indices() {
        println!("{:?}", grafo[node_index]);
    }

    println!("\n--- Busca por termo 'gamer' ---");
    search_products(&grafo, "gamer");

    println!("\n--- Busca por nome exato ---");
    search_by_name(&grafo, &mapa_nome, "Mouse Gamer");

    println!("\n--- Busca por categoria ---");
    search_by_category(&grafo, &mapa_categoria, "Periféricos");

    println!("\n--- Recomendações para produto com ID 1 ---");
    recommended_products(&grafo, 1);
}
