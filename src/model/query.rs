use super::DB;
use async_graphql::Object;
use async_std::sync::{Arc, RwLock};
use std::collections::HashMap;

use super::{Big, Small};

pub struct Query {
    db: DB,
}

#[Object]
impl Query {
    #[graphql(skip)]
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    async fn big(&self, id: String) -> Option<Big> {
        let db = self.db.read().await;
        db.get(&id).map(|data| Big {
            data: data.to_owned(),
        })
    }

    async fn small(&self, id: String) -> Option<Small> {
        let db = self.db.read().await;
        db.get(&id)
            .map(|data| {
                if data.len() == 1 {
                    data[0].parse().ok().map(|data| Small { data })
                } else {
                    None
                }
            })
            .flatten()
    }
}
