// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

use std::{time::{Duration, Instant}, rc::Rc};
use crate::server::login::Server;
use super::*;

// pub type TimerCallback = fn(&mut Server, timer_entity_id: EntityId);
pub type TimerCallback = dyn FnMut(&mut Server, &EntityId) -> bool;

// TODO: Move it to component.rs?
/// It is immutable.
struct TimerInfo {
    pub callback: Rc<RefCell<TimerCallback>>,
    pub duration: Duration,
}

#[macro_export]
macro_rules! register_timer {
    ($world: expr, $duration: expr, $f: expr) => {
        {
            entity::timer::create($world, $duration, Rc::new(RefCell::new($f)));
        }
    };
}

#[macro_export]
/// It is used when a loop is implemented.
macro_rules! register_loop {
    ($world: expr, $f: expr) => {
        {
            register_timer!($world, Duration::from_millis(100), $f);
        }
    };
}

pub fn create(world: &mut World, duration: Duration, callback: Rc<RefCell<TimerCallback>>) -> &mut Entity {
    let id = world.push();
    let entity = world.get_mut(&id).unwrap();
    let instant = std::time::Instant::now();
    let timer_info = TimerInfo {
        callback: callback,
        duration: duration,
    };
    entity.push(EntityKind::Timer);
    entity.push(timer_info);
    entity.push(instant);
    entity
}

/// Make its instant sync to the current time.
fn proceed(world: &mut World, entity_id: &EntityId) {
    let entity = world.get_mut(entity_id).unwrap();
    let instant = Instant::now();
    entity.push(instant);
}

fn is_runnable(world: &World, timer_entity_id: &EntityId) -> bool {
    let timer = world
        .get(timer_entity_id)
        .unwrap();
    let timer_info = timer
        .get::<TimerInfo>()
        .unwrap();
    let instant = timer
        .get::<Instant>()
        .unwrap();
    let duration = timer_info.duration;
    let elapsed = instant.elapsed();
    if elapsed.ge(&duration) {
        true
    } else {
        false
    }
}

fn get_callback(world: &World, timer_eid: &EntityId) -> Rc<RefCell<TimerCallback>> {
    let timer = world
        .get(timer_eid)
        .unwrap();
    let timer_info = timer
        .get::<TimerInfo>()
        .unwrap();
    timer_info.callback.clone()
}

/// If it is a runnable timer, then run it.
pub fn tick(server: &mut Server, timer_eid: &EntityId) {
    if !is_runnable(&server.world, timer_eid) {
        return;
    }
    
    let callback_ptr = get_callback(&server.world, timer_eid);
    let mut callback = callback_ptr.borrow_mut();
    
    if callback(server, timer_eid) {
        proceed(&mut server.world, timer_eid);
    } else {
        server.world.remove(timer_eid);
    }
}