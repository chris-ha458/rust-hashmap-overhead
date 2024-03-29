use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

static ALLOC: AtomicUsize = AtomicUsize::new(0);
static DEALLOC: AtomicUsize = AtomicUsize::new(0);

pub struct TrackingAllocator;

pub fn reset() {
    ALLOC.store(0, SeqCst);
    DEALLOC.store(0, SeqCst);
}

pub fn record_alloc(layout: Layout) {
    ALLOC.fetch_add(layout.size(), SeqCst);
}

pub fn record_dealloc(layout: Layout) {
    DEALLOC.fetch_add(layout.size(), SeqCst);
}

pub fn stats() -> Stats {
    let alloc: usize = ALLOC.load(SeqCst);
    let dealloc: usize = DEALLOC.load(SeqCst);
    let diff = (alloc as isize) - (dealloc as isize);

    Stats {
        alloc,
        dealloc,
        diff,
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc(layout);
        record_alloc(layout);
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        record_dealloc(layout);
        System.dealloc(ptr, layout);
    }
}

pub struct Stats {
    pub alloc: usize,
    pub dealloc: usize,
    pub diff: isize,
}
