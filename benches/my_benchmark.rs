use std::ops::{Range, RangeBounds};
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::distributions::uniform::SampleRange;
use rand::Rng;
use spaceship::{Spaceship, SpaceshipBuilder};
use spaceship::part::CreatePartType;

fn dummy<T>(range: T, mut funds: f32, mut parts: Vec<(CreatePartType, u16, u16)>) -> (Spaceship, f32) where T: SampleRange<u16> {
    let mut ship = Spaceship::new(black_box("dummy").to_string());
    let mut builder = SpaceshipBuilder::new(&mut ship);

    let mut rng = rand::thread_rng();

    for _ in 0..rng.gen_range(range) {
        while let Some((part_type, size, level)) = parts.pop() {
            builder.add_part(part_type, size, level);
        }
    }

    while builder.costs() > funds {
        builder.pop_part();
    }

    builder.complete(&mut funds);
    (ship, funds)
}

pub fn my_benchmark(c: &mut Criterion) {
    c.bench_function("dummy-1-2/4", |b| b.iter( 
        || dummy(1..=2, black_box(1340999.0), vec![
            (CreatePartType::Cargo, 10, 1),
            (CreatePartType::Hull, 20, 14),
            (CreatePartType::LivingQuarters, 10, 2),
            (CreatePartType::SolarPanels, 10, 1),
        ])
    ));
    c.bench_function("dummy-6-8/8", |b| b.iter(
        || dummy(6..=8, black_box(1340999.0), vec![
            (CreatePartType::Cargo, 10, 1),
            (CreatePartType::Hull, 20, 14),
            (CreatePartType::LivingQuarters, 10, 2),
            (CreatePartType::SolarPanels, 10, 1),
            (CreatePartType::Hull, 20, 14),
            (CreatePartType::SolarPanels, 10, 1),
            (CreatePartType::LivingQuarters, 10, 2),
            (CreatePartType::SolarPanels, 10, 1),
        ])
    ));
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);