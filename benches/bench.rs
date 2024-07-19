use std::collections::{BTreeMap, HashMap};

use criterion::{criterion_group, criterion_main, Criterion};
use out_of_band::{Game, Monster};
use rand::Rng;

fn create_monsters(set_size: u32) -> Vec<Monster> {
    (1..=set_size)
        .map(|i| Monster {
            hp: 100,
            x: i as u32,
            y: i as u32,
        })
        .collect()
}

fn create_items<'a, I>(monsters: I) -> impl Iterator<Item = (u32, [u32; 4])> + 'a
where
    I: IntoIterator,
    I::Item: 'a,
    I::IntoIter: 'a,
    I::Item: std::ops::Deref<Target = Monster>,
{
    monsters.into_iter().map(|m| (m.x - 1, [1, 2, 3, 4]))
}

fn bench_access(c: &mut Criterion) {
    const ORDER_OF_MAGNITUDE: u32 = 6;
    for set_size in (1..=ORDER_OF_MAGNITUDE).map(|zeros| 10u32.pow(zeros)) {
        let monster_list = create_monsters(set_size);

        let held_items: HashMap<_, _> = create_items(&monster_list).collect();
        let game_hash = Game {
            monster_list: monster_list.clone(),
            held_items,
        };

        let held_items: BTreeMap<_, _> = create_items(&monster_list).collect();
        let game_btree = Game {
            monster_list,
            held_items,
        };

        const ACCESS_PER_ITER: u32 = 1000;
        let mut rng = rand::thread_rng();
        let indices: Vec<_> = (0..ACCESS_PER_ITER)
            .map(|_| rng.gen_range(0..set_size))
            .collect();

        c.bench_function(
            &format!("HashMap {ACCESS_PER_ITER} accesses/set size {set_size}"),
            |b| {
                b.iter(|| {
                    for i in &indices {
                        let (_m, _i) = game_hash.get_monster_info(*i);
                    }
                })
            },
        );

        c.bench_function(
            &format!("BTreeMap {ACCESS_PER_ITER} accesses/set size {set_size}"),
            |b| {
                b.iter(|| {
                    for i in &indices {
                        let (_m, _i) = game_btree.get_monster_info(*i);
                    }
                })
            },
        );
    }
}

criterion_group!(benches, bench_access);
criterion_main!(benches);
