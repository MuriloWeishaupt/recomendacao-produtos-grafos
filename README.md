# MegaStore Graphs - Sistema de Recomendação de Produtos

## Descrição

MegaStore Graphs é um sistema simples de recomendação e busca de produtos para um e-commerce fictício, desenvolvido em Rust. Utiliza a estrutura de dados **grafo** para conectar produtos similares e fornecer recomendações inteligentes baseadas nessas conexões.

O projeto permite:

- Busca de produtos por nome
- Busca de produtos por categoria
- Recomendações de produtos similares a partir do ID de um produto

## Funcionalidades

- Construção do grafo de produtos com conexões entre produtos similares
- Mapas auxiliares para busca rápida por nome e categoria
- Sistema de recomendação baseado em conexões no grafo
- Interface via linha de comando para interação com o sistema

## Como usar

### Pré-requisitos

- Rust instalado ([rustup.rs](https://rustup.rs))
- Cargo (gerenciador de pacotes do Rust)

### Rodando o projeto

No terminal, clone o repositório e execute:

```bash
cargo run -- <comando> <argumento>
```

### Comandos disponíveis
```
  Comando	               Argumento        	          Descrição
  search-nome	           nome do produto	                  Busca produtos pelo nome exato
  search-categoria	     nome da categoria	                  Busca produtos por categoria exata
  recomendar	           ID do produto (número)	          Recomenda produtos similares
```

Exemplos
```
Buscar produto por nome:

cargo run -- search-nome "Mouse Gamer"
```
```
Buscar produto por categoria:

cargo run -- search-categoria "Periféricos"
```
```
Recomendar produtos similares pelo ID:

cargo run -- recomendar 1
```



### Estrutura do projeto
      > src
          > bin
             - main.rs
          - graph_utils.rs
          - lib.rs
          - models.rs
          - recommendation.rs
          - search.rs
          > target
          > tests
             - recommendation_tests.rs
           Cargo.lock
           Cargo.toml
           README.md

### Como funciona o sistema
   - Os produtos são carregados e adicionados ao grafo como nós.
  
   - Produtos similares são conectados por arestas (ligações).
  
   - Mapas (HashMap) indexam os produtos por nome e categoria para buscas rápidas.
  
   - As buscas retornam produtos que correspondem exatamente ao termo informado.
  
   - A recomendação utiliza o grafo para encontrar produtos conectados ao produto indicado.

