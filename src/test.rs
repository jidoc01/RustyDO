
use crate::prelude::*;

#[derive(Event)]
struct Tick;

#[derive(Component)]
struct Name(String);

#[test]
fn empty_iter_test() {
    let mut world = World::new();
    println!("start");
    world.add_handler(|_: Receiver<Tick>, mut fetcher: Fetcher<(EntityId, &mut Name)>| {
        fetcher.iter_mut().for_each(|(e, name)| {
            println!("{}", name.0);
        });
    });
    let e = world.spawn();
    world.insert(e, Name("hello".into()));
    world.send(Tick);
    world.despawn(e);
    world.send(Tick);
}
