use async_graphql::{InputObject, SimpleObject};
use async_std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "BigInput")]
pub struct Big {
    pub data: Vec<String>,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "SmallInput")]
pub struct Small {
    pub data: u64,
}

type RawDB = HashMap<String, Vec<String>>;
pub type DB = Arc<RwLock<RawDB>>;
