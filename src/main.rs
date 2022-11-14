use blog_alloc::{alloc, Stats, TrackingAllocator};

#[global_allocator]
static ALLOC: TrackingAllocator = TrackingAllocator;

pub fn run_and_track<T>(name: &str, f: impl FnOnce() -> T) {
    alloc::reset();
    alloc::enable();

    let t = f();

    alloc::disable();

    let Stats {
        alloc,
        dealloc,
        diff,
    } = alloc::stats();
    println!("==========");
    println!("experiment {name}");
    println!("alloced={alloc}, dealloced={dealloc}, diff={diff}");
    println!();

    drop(t);
}

fn main() {
    run_and_track("nothing", || {});
    run_and_track("format with string", || format!("Hello, {}!!!", "world"));
    run_and_track("format with int", || format!("Hello, {}!!!", 123));
}
