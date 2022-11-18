use std::collections::{HashMap, BTreeMap};
use rand::{thread_rng, Fill, Rng};

use blog_alloc::{alloc, Stats, TrackingAllocator};

#[global_allocator]
static ALLOC: TrackingAllocator = TrackingAllocator;

pub fn run_and_track<T>(name: &str, meta: &str, f: impl FnOnce() -> T) {
    alloc::reset();
    alloc::enable();

    let t = f();

    alloc::disable();

    let Stats {
        alloc,
        dealloc,
        diff,
    } = alloc::stats();
    println!("{name},{meta},{alloc},{dealloc},{diff}");

    drop(t);
}

#[derive(Clone, Copy)]
pub struct DummyData {
    pub data: [u8; 100],
}

/// Generates lists of random elements with the length specified.
pub fn generate_keys_values(len: usize) -> Vec<(u64, DummyData)> {
    let mut rng = thread_rng();

    let mut pairs = Vec::with_capacity(len);

    for _ in 0..len {
        let mut data: [u8; 100] = [0; 100];
        data.try_fill(&mut rng).expect("filling data should work");
        let val = DummyData { data };

        let key = rng.gen();

        pairs.push((key, val));
    }

    pairs
}

fn main() {
    let large_pairs = generate_keys_values(1_000_000);
    println!("generated data");
    println!();

    let sizes: [usize; 9] = [0, 10, 100, 1_000, 10_000, 50_000, 100_000, 500_000, 1_000_000];

    println!("name,size,alloced,dealloced,diff");
    for size in sizes {
        run_and_track("hashmap", &size.to_string(), || {
            let mut m = HashMap::<u64, DummyData>::new();

            for (key, val) in &large_pairs[..size] {
                m.insert(*key, *val);
            }

            m
        });

        run_and_track("btreemap", &size.to_string(), || {
            let mut m = BTreeMap::<u64, DummyData>::new();

            for (key, val) in &large_pairs[..size] {
                m.insert(*key, *val);
            }

            m
        });

        run_and_track("vec-pair", &size.to_string(), || {
            let mut k: Vec<u64> = Vec::with_capacity(size);
            let mut v: Vec<DummyData> = Vec::with_capacity(size);

            for (key, val) in &large_pairs[..size] {
                k.push(*key);
                v.push(*val);
            }

            (k, v)
        });
    }
}
