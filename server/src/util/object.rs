// RustyDO
//
// Copyright 2022. JungHyun Kim (jidoc01).
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use crate::prelude::*;

use std::{sync::{Arc}, any::{TypeId, Any}, collections::{HashMap}};

pub type EntityId = u64;


//pub type EntityRef<'a> = RefMut<'a, Entity>;

/*
impl EntityRef {
    pub fn new(entity: Entity) -> Self {
        Self {
            inner: Rc::new(RefCell::new(entity))
        }
    }

    pub fn push<T>(&mut self, v: T) -> bool
        where T:
            'static + Sync + Send {
        self
            .inner
            .borrow_mut()
            .push(v)
    }

    pub fn get<T>(&self) -> Option<&T>
        where T:
            'static + Sync + Send {
        None
    }

    pub fn get_mut<T>(&self) -> Option<&mut T>
        where T:
            'static + Sync + Send {
        self
            .inner
            .borrow_mut()
            .get_mut()
    }

    pub fn id(&self) -> EntityId {
        self.borrow().id()
    }
}*/


pub struct Ptr<T: ?Sized + 'static + Sync + Send> {
    inner: Arc<RwLock<T>>,
}

pub struct World {
    entities: HashMap<EntityId, Entity>,
    entity_id_counter: EntityId,
}

impl World {
    pub fn default() -> Self {
        Self {
            entities: HashMap::default(),
            entity_id_counter: EntityId::min_value(),
        }
    }

    fn get_new_id(&mut self) -> EntityId {
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;
        id
    }

    pub fn push(&mut self) -> EntityId {
        let k = self.get_new_id();
        let v = Entity::new(k);
        self.entities.insert(k, v);
        k
    }
    
    pub fn get(&self, k: &EntityId) -> Result<&Entity> {
        match self.entities.get(k) {
            Some(v) => Ok(v),
            None => bail!("An entity does not exist")
        }
    }
    
    pub fn get_mut(&mut self, k: &EntityId) -> Result<&mut Entity> {
        match self.entities.get_mut(k) {
            Some(v) => Ok(v),
            None => bail!("An entity does not exist")
        }
    }

    pub fn remove(&mut self, k: &EntityId) -> bool {
        self.entities
            .remove(k)
            .is_some()
    }
    
    pub fn values(&self) -> Vec<&Entity> {
        self
            .entities
            .values()
            .collect()
    }
    
    pub fn values_mut(&mut self) -> Vec<&mut Entity> {
        self
            .entities
            .values_mut()
            .collect()
    }

    pub fn select<T>(&self, f: T) -> Vec<&Entity> where T: Fn(&Entity) -> bool {
        self
            .entities
            .values()
            .filter(|&entity| f(entity))
            .collect()
    }

    pub fn select_mut<T>(&mut self, f: T) -> Vec<&mut Entity> where T: Fn(&Entity) -> bool {
        self
            .entities
            .values_mut()
            .filter(|entity| f(entity as &Entity))
            .collect()
    }

}

pub struct Entity {
    entity_id: EntityId,
    inner: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    fn new(id: EntityId) -> Self {
        Self {
            inner: HashMap::default(),
            entity_id: id,
        }
    }

    pub fn id(&self) -> EntityId {
        self.entity_id
    }

    /// Push a new value into the map.
    /// Returns false if the map already has the key. 
    pub fn push<T>(&mut self, v: T) -> bool
        where T:
            'static {
        let k = TypeId::of::<T>();
        self.inner
            .insert(k, Box::new(v))
            .is_none()
    }

    pub fn get<T>(&self) -> Result<&T>
        where T:
            'static {
        let type_id = TypeId::of::<T>();
        match self.inner.get(&type_id) {
            None => bail!("No component named {} in {}", std::any::type_name::<T>(), self.entity_id),
            Some(v) => {
                let v: Option<&T> = v.downcast_ref();
                match v {
                    Some(v) => Ok(v),
                    None => bail!("Internal error: could not downcast a component.")
                }
            }
        }
    }

    pub fn get_mut<T>(&mut self) -> Result<&mut T>
        where T:
            'static + Send {
        let type_id = TypeId::of::<T>();
        match self.inner.get_mut(&type_id) {
            None => bail!("No component named {} in {}", std::any::type_name::<T>(), self.entity_id),
            Some(v) => {
                let v: Option<&mut T> = v.downcast_mut();
                match v {
                    Some(v) => Ok(v),
                    None => bail!("Internal error: could not downcast a component.")
                }
            }
        }
    }

    pub fn remove<T: 'static>(&mut self) -> bool {
        let type_id = TypeId::of::<T>();
        self.inner
            .remove(&type_id)
            .is_some()
    }
}

impl<T: 'static + Sync + Send> Ptr<T> {
    pub fn new(base: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(base)),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            inner:self.inner.clone()
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().await
    }
    
    pub async fn lock(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().await
    }
}
