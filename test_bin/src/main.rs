use ecs_generator::{construct_world, WorldConstructArgs};
use rand::Rng;

fn main() {
    for _ in 0..10000 {
        let archetype_count = rand::thread_rng().gen_range(20..1000);
        let args = WorldConstructArgs {
            archetype_count,
            matched_archetype_count: rand::thread_rng().gen_range(1..=archetype_count),
            entity_count: rand::thread_rng().gen_range(1..5000),
        };
        construct_world(args, rand::thread_rng().gen());
    }
}
