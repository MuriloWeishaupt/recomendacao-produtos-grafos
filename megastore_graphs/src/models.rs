#[derive(Debug, Clone)] //Necessário para usar println!("{:?}", produto) no Main.rs
pub struct Produto {
    pub id: usize,
    pub nome: String,
    pub categoria: String,
    pub preco: f32,
    pub popularidade: u32,
}