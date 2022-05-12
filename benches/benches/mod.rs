use criterion::*;
use ecs_generator::WorldConstructArgs;

mod single_iter;

fn bench_single_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_insert");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));
    for archetype_fragmentation_mul in 1..=20 {
        let frag = 0.05 * (archetype_fragmentation_mul as f64);
        let frag = f64::min(1.0, frag);
        for archetype_count in [50, 100, 150, 200, 500, 1000] {
            for entity_count_per_archetype_avg in [10, 20, 30, 40, 100, 200] {
                let args = WorldConstructArgs {
                    archetype_fragmentation: frag,
                    archetype_count,
                    entity_count_per_archetype_avg,
                };

                group.bench_function(
                    &format!(
                        "arch_frag:{}, arch_count:{}, entity_count_per_archetype_avg:{}",
                        frag, archetype_count, entity_count_per_archetype_avg
                    ),
                    |b| {
                        let mut bench = single_iter::Benchmark::new(args);
                        b.iter(move || bench.run());
                    },
                );
            }
        }
    }
}

criterion_group!(benchmarks, bench_single_iter);
criterion_main!(benchmarks);
