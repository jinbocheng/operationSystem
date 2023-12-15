pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

// os/src/mm/mod.rs

mod heap_allocator;

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}

