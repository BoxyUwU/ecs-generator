use criterion::*;
use ecs_generator::WorldConstructArgs;

mod single_iter;

fn bench_single_iter(c: &mut Criterion) {
    for entity_count in [100, 400, 800, 4000, 8000] {
        let mut group = c.benchmark_group(format!("single_iter/e{}", entity_count));
        group.warm_up_time(std::time::Duration::from_millis(500));
        group.measurement_time(std::time::Duration::from_secs(5));

        for (archetype_count, line_name) in [
            (25, "A25"),
            (100, "B100"),
            (200, "C200"),
            (500, "D500"),
            (1000, "E1000"),
        ] {
            let divd_count = archetype_count / 5;
            for (matched_archetype_count, id) in [
                (divd_count, "20"),
                (divd_count * 2, "40"),
                (divd_count * 3, "60"),
                (divd_count * 4, "80"),
                (archetype_count, "100"),
            ] {
                let args = WorldConstructArgs {
                    matched_archetype_count,
                    archetype_count,
                    entity_count,
                };

                group.bench_with_input(
                    BenchmarkId::new(format!("{}", line_name), id),
                    &args,
                    |b, args| {
                        let mut bench = single_iter::Benchmark::new(args.clone());
                        b.iter(move || bench.run());
                    },
                );
            }
        }
    }
}

criterion_group!(benchmarks, bench_single_iter);
criterion_main!(benchmarks);
