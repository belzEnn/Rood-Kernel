use linked_list_allocator::LockedHeap;

// Heap size 1MB
const HEAP_SIZE: usize = 1024 * 1024;

// Static buffer for heap
static mut HEAP_MEM: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

// Global allocator
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// Initialize heap
pub unsafe fn init() {
    ALLOCATOR.lock().init(HEAP_MEM.as_mut_ptr(), HEAP_SIZE);
}
