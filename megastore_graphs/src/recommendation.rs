use crate::models::Produto;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;

pub fn recommended_products(grafo: &Graph<Produto, (), Undirected>, produto_id: usize) {
    if let Some((idx, produto_base)) = grafo
        .node_indices()
        .map(|i| (i, &grafo[i]))
        .find(|(_, p)| p.id == produto_id)
    {
        println!("Recomendando para: {:?}", produto_base);

        let mut candidatos: Vec<(&Produto, u32)> = grafo
            .node_indices()
            .filter(|&i| i != idx)
            .map(|i| {
                let p = &grafo[i];
                let peso = calculate_similarity_weight(produto_base, p);
                (p, peso)
            })
            .filter(|&(_, peso)| peso > 0)
            .collect();

        candidatos.sort_by(|a, b| b.1.cmp(&a.1));

        if candidatos.is_empty() {
            println!("Nenhum produto similar encontrado.");
        } else {
            println!("Produtos recomendados:");
            for (produto, peso) in candidatos.iter().take(5) {
                println!("- {:?} [similaridade: {}]", produto, peso);
            }
        }
    } else {
        println!("Produto com id {} não encontrado", produto_id);
    }
}

pub fn calculate_similarity_weight(a: &Produto, b: &Produto) -> u32 {
    let mut peso = 0;
    if a.categoria == b.categoria { peso += 5; }
    if (a.popularidade as i32 - b.popularidade as i32).abs() <= 10 { peso += 3; }
    if (a.preco - b.preco).abs() <= 200.0 { peso += 1; }
    peso
}
