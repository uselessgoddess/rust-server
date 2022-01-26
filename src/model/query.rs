use super::DB;
use crate::DbCtx;
use async_graphql::{Context, Object};
use async_std::sync::{Arc, RwLock};
use std::collections::HashMap;

use super::{Big, Small};

pub struct Query;

#[Object]
impl Query {
    async fn big(&self, ctx: &Context<'_>, id: String) -> Option<Big> {
        let db = ctx.data_unchecked::<DbCtx>().read().await;
        db.get(&id).map(|data| Big {
            data: data.to_owned(),
        })
    }

    async fn small(&self, ctx: &Context<'_>, id: String) -> Option<Small> {
        let db = ctx.data_unchecked::<DbCtx>().read().await;
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

    async fn random(&self) -> i32 {
        228
    }
}
