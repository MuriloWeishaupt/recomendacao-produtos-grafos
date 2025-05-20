use crate::models::Produto;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;

pub fn connect_similar_products(grafo: &mut Graph<Produto, (), Undirected>) {
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
