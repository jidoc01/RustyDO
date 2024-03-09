pub mod account;
pub mod board;

use std::{borrow::Borrow, cell::{RefCell, UnsafeCell}};

use crate::{prelude::*, world::WorldHelper};
use polodb_core::{bson::Document, Database};
use serde::{de::DeserializeOwned, Serialize};

const STORAGE_PATH: &str = "db.polo";

#[derive(Component)]
pub struct Storage {
    db: Database,
}

#[derive(Event)]
pub struct SaveEvent;

fn get_type_id_string<T: 'static>() -> String {
    format!("{:?}", std::any::TypeId::of::<T>())
}

impl Storage {
    pub fn find_one<T: 'static + Serialize + DeserializeOwned>(&self, filter: impl Into<Option<Document>>) -> Option<T> {
        let coll_name = get_type_id_string::<T>();
        let coll = self.db.collection::<T>(&coll_name);
        coll.find_one(filter).unwrap()
    }

    pub fn update_one_with_query<T: 'static + Serialize>(&self, filter: impl Into<Document>, query: impl Into<Document>) {
        let coll_name = get_type_id_string::<T>();
        let coll = self.db.collection::<T>(&coll_name);
        coll.update_one(filter.into(), query.into()).unwrap();
    }

    /// This costs more than `update_one_with_query` since it replaces the whole document.
    pub fn update_one_with_replacement<T: 'static + Serialize>(&self, filter: impl Into<Document>, update: impl Serialize) {
        let coll_name = get_type_id_string::<T>();
        let coll = self.db.collection::<T>(&coll_name);
        let doc = to_document(&update).unwrap();
        let query = doc! {
            "$set": doc
        };
        coll.update_one(filter.into(), query).unwrap();
    }

    pub fn insert_one<T: 'static + Serialize>(&self, doc: impl Borrow<T>) {
        let coll_name = get_type_id_string::<T>();
        let coll = self.db.collection::<T>(&coll_name);
        coll.insert_one(doc).unwrap();
    }
}

impl Default for Storage {
    fn default() -> Self {
        let db = Database::open_file(STORAGE_PATH).unwrap();
        Self {
            db,
        }
    }
}

pub fn init(world_helper: &mut WorldHelper) {
    world_helper
        .add_component::<Storage>();
    world_helper
        .spawn_single(Storage::default());
}