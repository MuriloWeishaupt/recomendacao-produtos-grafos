mod models;
use models::Produto;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;

fn search_products(grafo: &Graph<Produto, (), Undirected>, termo: &str) {
    let termo = termo.to_lowercase();
    let mut encontrados = 0;

    for node_index in grafo.node_indices() {
        let produto = &grafo[node_index];
        let nome = produto.nome.to_lowercase();
        let categoria = produto.categoria.to_lowercase();

        if nome.contains(&termo) || categoria.contains(&termo) {
            println!("Encontrado: {:?}", produto);
            encontrados += 1;
        }
    }

    if encontrados == 0 {
        println!("Nenhum produto encontrado com o termo: '{}'", termo);
    }
}

fn recommended_products(grafo: &Graph<Produto, (), Undirected>, produto_id: usize) {
    for node_index in grafo.node_indices() {
        if grafo[node_index].id == produto_id {
            println!("Recomendações para o produto: {:?}", grafo[node_index]);
           
            //Coleta vizinhos em um vetor
            let mut vizinhos: Vec<_> = grafo
                .neighbors(node_index)
                .map(|vizinho_idx| &grafo[vizinho_idx])
                .collect();


            //Ordena por popularidade decrescente
            vizinhos.sort_by(|a, b| b.popularidade.cmp(&a.popularidade));

            //Imprime os produtos ordenados
            for produto in vizinhos {
                println!("Produto recomendado: {:?} (Popularidade: {})", produto, produto.popularidade)
            }

            return;
        }
    }
    
    println!("produto com id {} não encontrado", produto_id);
}

fn search_by_name(grafo: &Graph<Produto, (), Undirected>, mapa: &HashMap<String, Vec<NodeIndex>>, nome: &str) {
    if let Some(indices) = mapa.get(nome) {
        for idx in indices {
            println!("Produto encontrado: {:?}", grafo[*idx]);
        }
    } else {
        println!("Nenhum produto encontrado com nome: {}", nome);
    }
}

fn search_by_category(grafo: &Graph<Produto, (), Undirected>, mapa: &HashMap<String, Vec<NodeIndex>>, categoria: &str) {
    if let Some(indices) = mapa.get(categoria) {
        for idx in indices {
            println!("Produto da categoria '{}': {:?}", categoria, grafo[*idx]);
        }
    } else {
        println!("Nenhum produto encontrado na categoria: {}", categoria);
    }
}

fn connect_similar_products(grafo: &mut Graph<Produto, (), Undirected>) {
    let nodes: Vec<NodeIndex> = grafo.node_indices().collect();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let a = &grafo[nodes[i]];
            let b = &grafo[nodes[j]];
            let mesma_categoria = a.categoria == b.categoria;
            let popularidade_proxima = (a.popularidade as i32 - b.popularidade as i32).abs() <= 10;
            if mesma_categoria || popularidade_proxima {
                grafo.add_edge(nodes[i], nodes[j], ());
            }
        }
    }
}

fn main() {
    let mut grafo: Graph<Produto, (), Undirected> = Graph::new_undirected();

    let produto1 = Produto {
        id: 1,
        nome: "Mouse Gamer".to_string(),
        categoria: "Periféricos".to_string(),
        preco: 199.90,
        popularidade: 25,
    };

    let produto2 = Produto {
        id: 2,
        nome: "Headset Gamer".to_string(),
        categoria: "Periféricos".to_string(),
        preco: 539.90,
        popularidade: 15,
    };

    let produto3 = Produto {
        id: 3,
        nome: "Teclado sem fio".to_string(),
        categoria: "Periféricos".to_string(),
        preco: 1299.90,
        popularidade: 40,
    };

    let produto4 = Produto {
        id: 4,
        nome: "Monitor Full HD 24 polegadas".to_string(),
        categoria: "Periféricos".to_string(),
        preco: 899.90,
        popularidade: 30,
    };

    let produto5 = Produto {
        id: 5,
        nome: "Placa de Vídeo RTX 3060".to_string(),
        categoria: "Hardware".to_string(),
        preco: 1999.90,
        popularidade: 45,
    };

    let produto6 = Produto {
        id: 6,
        nome: "SSD 1TB NVMe".to_string(),
        categoria: "Armazenamento".to_string(),
        preco: 499.90,
        popularidade: 35,
    };

    let produto7 = Produto {
        id: 7,
        nome: "Gabinete Gamer RGB".to_string(),
        categoria: "Gabinetes".to_string(),
        preco: 389.90,
        popularidade: 20,
    };

    let produto8 = Produto {
        id: 8,
        nome: "Fonte 600W 80 Plus".to_string(),
        categoria: "Hardware".to_string(),
        preco: 359.90,
        popularidade: 28,
    };

    let mut mapa_nome: HashMap<String, Vec<NodeIndex>> = HashMap::new();
    let mut mapa_categoria: HashMap<String, Vec<NodeIndex>> = HashMap::new();

    let node1 = grafo.add_node(produto1);
    mapa_nome.entry(grafo[node1].nome.clone()).or_default().push(node1);
    mapa_categoria.entry(grafo[node1].categoria.clone()).or_default().push(node1);

    let node2 = grafo.add_node(produto2);
    mapa_nome.entry(grafo[node2].nome.clone()).or_default().push(node2);
    mapa_categoria.entry(grafo[node2].categoria.clone()).or_default().push(node2);

    let node3 = grafo.add_node(produto3);
    mapa_nome.entry(grafo[node3].nome.clone()).or_default().push(node3);
    mapa_categoria.entry(grafo[node3].categoria.clone()).or_default().push(node3);

    let node4 = grafo.add_node(produto4);
    mapa_nome.entry(grafo[node4].nome.clone()).or_default().push(node4);
    mapa_categoria.entry(grafo[node4].categoria.clone()).or_default().push(node4);

    let node5 = grafo.add_node(produto5);
    mapa_nome.entry(grafo[node5].nome.clone()).or_default().push(node5);
    mapa_categoria.entry(grafo[node5].categoria.clone()).or_default().push(node5);

    let node6 = grafo.add_node(produto6);
    mapa_nome.entry(grafo[node6].nome.clone()).or_default().push(node6);
    mapa_categoria.entry(grafo[node6].categoria.clone()).or_default().push(node6);

    let node7 = grafo.add_node(produto7);
    mapa_nome.entry(grafo[node7].nome.clone()).or_default().push(node7);
    mapa_categoria.entry(grafo[node7].categoria.clone()).or_default().push(node7);

    let node8 = grafo.add_node(produto8);
    mapa_nome.entry(grafo[node8].nome.clone()).or_default().push(node8);
    mapa_categoria.entry(grafo[node8].categoria.clone()).or_default().push(node8);

    // Conectar automaticamente produtos similares
    connect_similar_products(&mut grafo);

    println!("\n--- Produtos no grafo ---");
    for node_index in grafo.node_indices() {
        println!("{:?}", grafo[node_index]);
    }

    println!("\n--- Resultados da busca ---");
    search_products(&grafo, "gamer");

    println!("\n--- Recomendação de produtos ---");
    recommended_products(&grafo, 1);

    println!("\n--- Busca por nome ---");
    search_by_name(&grafo, &mapa_nome, "Mouse Gamer");

    println!("\n--- Busca por categoria ---");
    search_by_category(&grafo, &mapa_categoria, "Periféricos");
}
