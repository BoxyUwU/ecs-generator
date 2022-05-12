use ecs_generator::{construct_world, WorldConstructArgs};
use rand::Rng;

fn main() {
    for _ in 0..10000 {
        let args = WorldConstructArgs {
            archetype_fragmentation: rand::thread_rng().gen_range(0.01..1.0),
            archetype_count: rand::thread_rng().gen_range(20..1000),
            entity_count: rand::thread_rng().gen_range(1..5000),
        };
        construct_world(args, rand::thread_rng().gen());
    }
}
