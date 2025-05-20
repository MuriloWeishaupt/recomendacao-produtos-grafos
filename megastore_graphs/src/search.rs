use crate::models::Produto;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;

pub fn search_products(grafo: &Graph<Produto, (), Undirected>, termo: &str) {
    let termo = termo.to_lowercase();
    let mut encontrados = 0;

    for node_index in grafo.node_indices() {
        let produto = &grafo[node_index];
        if produto.nome.to_lowercase().contains(&termo) || produto.categoria.to_lowercase().contains(&termo) {
            println!("Encontrado: {:?}", produto);
            encontrados += 1;
        }
    }

    if encontrados == 0 {
        println!("Nenhum produto encontrado com o termo: '{}'", termo);
    }
}

pub fn search_by_name(
    grafo: &Graph<Produto, (), Undirected>,
    mapa: &HashMap<String, Vec<NodeIndex>>,
    nome: &str,
) -> Vec<Produto> {
    if let Some(indices) = mapa.get(nome) {
        indices.iter().map(|idx| grafo[*idx].clone()).collect()
    } else {
        Vec::new()
    }
}



pub fn search_by_category(
    grafo: &Graph<Produto, (), Undirected>,
    mapa: &HashMap<String, Vec<NodeIndex>>,
    categoria: &str,
) -> Vec<Produto> {
    if let Some(indices) = mapa.get(categoria) {
        indices.iter().map(|idx| grafo[*idx].clone()).collect()
    } else {
        Vec::new()
    }
}

