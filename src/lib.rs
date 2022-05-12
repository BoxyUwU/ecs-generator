use bevy_ecs::prelude::*;
use rand::{prelude::SmallRng, Rng, SeedableRng};

#[derive(Component)]
struct EmptyColumn<const N: u8>;

pub struct EmptyArchetypeMaker(u32);
impl EmptyArchetypeMaker {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn advance_archetype(&mut self) {
        self.0 += 1;
    }

    pub fn entity_in_current_archetype(&mut self, world: &mut World, b: impl Bundle) -> Entity {
        let mut e = world.spawn();
        e.insert_bundle(b);
        for idx in (0_u8..32).filter(|idx| (self.0 & (1 << idx)) > 0) {
            match idx {
                0 => drop(e.insert(EmptyColumn::<0>)),
                1 => drop(e.insert(EmptyColumn::<1>)),
                2 => drop(e.insert(EmptyColumn::<2>)),
                3 => drop(e.insert(EmptyColumn::<3>)),
                4 => drop(e.insert(EmptyColumn::<4>)),
                5 => drop(e.insert(EmptyColumn::<5>)),
                6 => drop(e.insert(EmptyColumn::<6>)),
                7 => drop(e.insert(EmptyColumn::<7>)),
                8 => drop(e.insert(EmptyColumn::<8>)),
                9 => drop(e.insert(EmptyColumn::<9>)),
                10 => drop(e.insert(EmptyColumn::<10>)),
                11 => drop(e.insert(EmptyColumn::<11>)),
                12 => drop(e.insert(EmptyColumn::<12>)),
                13 => drop(e.insert(EmptyColumn::<13>)),
                14 => drop(e.insert(EmptyColumn::<14>)),
                15 => drop(e.insert(EmptyColumn::<15>)),
                16 => drop(e.insert(EmptyColumn::<16>)),
                17 => drop(e.insert(EmptyColumn::<17>)),
                18 => drop(e.insert(EmptyColumn::<18>)),
                19 => drop(e.insert(EmptyColumn::<19>)),
                20 => drop(e.insert(EmptyColumn::<20>)),
                21 => drop(e.insert(EmptyColumn::<21>)),
                22 => drop(e.insert(EmptyColumn::<22>)),
                23 => drop(e.insert(EmptyColumn::<23>)),
                24 => drop(e.insert(EmptyColumn::<24>)),
                25 => drop(e.insert(EmptyColumn::<25>)),
                26 => drop(e.insert(EmptyColumn::<26>)),
                27 => drop(e.insert(EmptyColumn::<27>)),
                28 => drop(e.insert(EmptyColumn::<28>)),
                29 => drop(e.insert(EmptyColumn::<29>)),
                30 => drop(e.insert(EmptyColumn::<30>)),
                31 => drop(e.insert(EmptyColumn::<31>)),
                _ => unreachable!(),
            }
        }
        e.id()
    }

    pub fn null_archetype(&mut self, world: &mut World) {
        let archetypes_pre = world.archetypes().len();
        self.advance_archetype();
        let e = self.entity_in_current_archetype(world, ());
        world.despawn(e);
        assert_eq!(archetypes_pre + 1, world.archetypes().len());
    }
}

#[derive(Copy, Clone)]
pub struct WorldConstructArgs {
    pub archetype_fragmentation: f64,
    // FIXME: archetype_distribution
    pub archetype_count: u32,

    // FIXME: entity_distribution
    pub entity_count_per_archetype_avg: u32,
}

#[derive(Component)]
pub struct Data(u64);

pub fn construct_world(args: WorldConstructArgs, seed: u64) -> World {
    // println!("seed: {seed}");
    assert!(args.archetype_fragmentation >= 0.0 && args.archetype_fragmentation <= 1.0);
    assert!(args.entity_count_per_archetype_avg > 0);

    let mut rng = SmallRng::seed_from_u64(seed);
    let mut world = World::new();
    let mut null_archetype_maker = EmptyArchetypeMaker::new();
    let mut data_archetype_maker = EmptyArchetypeMaker::new();

    for _ in 0..args.archetype_count {
        // println!("archetype: {i}");
        let matches = rng.gen_bool(args.archetype_fragmentation);
        match matches {
            false => {
                // println!("doesnt match");
                // println!("null_archetype_maker:{}", null_archetype_maker.0);
                null_archetype_maker.null_archetype(&mut world);
            }
            true => {
                // println!("matches");
                // println!("data_archetype_maker:{}", data_archetype_maker.0);

                for _ in 0..args.entity_count_per_archetype_avg {
                    data_archetype_maker.entity_in_current_archetype(&mut world, (Data(100),));
                }
                data_archetype_maker.advance_archetype();
            }
        }
    }

    // + 2, because of the `[]` archetype and the resource archetype
    assert_eq!(args.archetype_count as usize + 2, world.archetypes().len());

    world
}
