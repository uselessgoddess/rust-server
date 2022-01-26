use super::DB;
use super::{Big, Small};
use crate::DbCtx;
use async_graphql::{Context, Object};
use async_std::sync::{Arc, RwLock};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn big(&self, ctx: &Context<'_>, id: String, mut data: Big) -> Option<Vec<String>> {
        let mut db = ctx.data_unchecked::<DbCtx>().write().await;
        let mut before = None;
        let entry = db.entry(id);

        match entry {
            Entry::Occupied(mut o) => {
                std::mem::swap(&mut data.data, &mut o.get_mut());
                before = Some(data.data);
            }
            Entry::Vacant(v) => {
                v.insert(data.data);
            }
        }
        before
    }

    async fn small(&self, ctx: &Context<'_>, id: String, data: Small) -> Option<Vec<String>> {
        let mut db = ctx.data_unchecked::<DbCtx>().write().await;
        let mut before = None;
        let entry = db.entry(id);

        let mut data = vec![data.data.to_string()];
        match entry {
            Entry::Occupied(mut o) => {
                std::mem::swap(&mut data, &mut o.get_mut());
                before = Some(data);
            }
            Entry::Vacant(v) => {
                v.insert(data);
            }
        }
        before
    }
}
