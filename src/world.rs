use std::time::Instant;

use crate::prelude::*;

pub struct WorldHelper {
    world: World,
    task_handler: PeriodicTaskHandler,
}

impl WorldHelper {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            task_handler: PeriodicTaskHandler::new(),
        }
    }

    pub fn spawn_single<T: Component>(&mut self, component: T) -> EntityId {
        let e = self.world.spawn();
        self.world.insert(e, component);
        e
    }

    pub fn execute(&mut self) {
        self.task_handler.execute(&mut self.world);
    }

    pub fn add_task<T: TaskExecutable + 'static>(&mut self, task: T) -> &mut Self {
        self.task_handler.register(task);
        self
    }

    pub fn add_system<S, M>(&mut self, system: S) -> &mut Self where S: IntoSystem<M> {
        self.world.add_system(system);
        self
    }

    pub fn add_event<E: Event>(&mut self) -> &mut Self {
        self.world.add_event::<E>();
        self
    }
}

pub trait TaskExecutable {
    const DURATION: Duration;
    fn init(&mut self, world: &mut World);
    fn execute(&mut self, world: &mut World);
}

trait TaskExecutableSafe {
    fn duration(&self) -> Duration;
    fn init(&mut self, world: &mut World);
    fn execute(&mut self, world: &mut World);
}

impl<T: ?Sized + TaskExecutable> TaskExecutableSafe for T {
    fn init(&mut self, world: &mut World) {
        <T as TaskExecutable>::init(self, world);
    }
    fn duration(&self) -> Duration {
        <T as TaskExecutable>::DURATION
    }
    fn execute(&mut self, world: &mut World) {
        <T as TaskExecutable>::execute(self, world);
    }
}

struct PeriodicTask {
    pub last_time: Instant,
    pub task: Box<dyn TaskExecutableSafe>,
}

impl PeriodicTask {
    fn new<T: TaskExecutable + 'static>(task: T) -> Self {
        Self {
            last_time: Instant::now(),
            task: Box::new(task),
        }
    }
}

struct PeriodicTaskHandler {
    tasks: Vec<PeriodicTask>,
}

impl PeriodicTaskHandler {
    pub fn new() -> Self {
        Self {
            tasks: vec![], /* TODO: priority */
        }
    }

    pub fn execute(&mut self, world: &mut World) {
        for task in self.tasks.iter_mut() {
            let inner_task = task.task.as_mut();
            if task.last_time.elapsed() >= inner_task.duration() {
                inner_task.execute(world);
                task.last_time = Instant::now(); /* TODO: update the last time only once */
            }
        }
    }

    pub fn register<T: TaskExecutable + 'static>(&mut self, task: T) {
        self.tasks.push(PeriodicTask::new(task));
        self.tasks.last_mut().unwrap().task.init(&mut World::new());
    }
}