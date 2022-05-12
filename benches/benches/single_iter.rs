use bevy_ecs::prelude::*;
use criterion::black_box;
use ecs_generator::{construct_world, Data, WorldConstructArgs};

pub struct Benchmark(World, QueryState<&'static Data>);

impl Benchmark {
    pub fn new(args: WorldConstructArgs) -> Self {
        let mut world = construct_world(args, 89745387934559847);
        let state = world.query::<&Data>();
        Benchmark(world, state)
    }

    pub fn run(&mut self) {
        self.1.for_each(&mut self.0, |data| {
            black_box(data);
        });
    }
}
