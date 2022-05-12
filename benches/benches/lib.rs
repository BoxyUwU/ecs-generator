use criterion::*;
use ecs_generator::WorldConstructArgs;

mod single_iter;

fn bench_single_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_iter");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(4));
    for entity_count in [100, 400, 800, 4000, 8000] {
        for archetype_count in [25, 100, 200, 500, 1000] {
            let divd_count = archetype_count / 4;
            for matched_archetype_count in [
                1,
                divd_count,
                divd_count * 2,
                divd_count * 3,
                archetype_count,
            ] {
                let args = WorldConstructArgs {
                    matched_archetype_count,
                    archetype_count,
                    entity_count,
                };

                group.bench_function(
                    &format!(
                        "matched_arch_count:{}, arch_count:{}, entity_count:{}",
                        matched_archetype_count, archetype_count, entity_count
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
