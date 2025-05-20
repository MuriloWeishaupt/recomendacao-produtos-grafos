pub mod models;
pub mod recommendation;
pub mod search;
pub mod graph_utils;

pub use recommendation::{
    recommended_products,
};

pub use search::{
    search_products,
    search_by_name,
    search_by_category,
};

pub use graph_utils::{
    connect_similar_products,

};
