use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,

};

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};
use linked_list_allocator::LockedHeap;

//this counter is for the real info, real bytes allocated so that meminfo cna show live state. prev. tutroial had lockedheap,, whihc WOKRS, BUT we cant ask how much is in acutal usage. 
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

pub struct TrackingAllocator {
    inner: LockedHeap,
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.inner.alloc(layout);
        if !ptr.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator {
    inner: LockedHeap::empty(),
};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }
    unsafe {
        //instead of bare lockedheap before, its now a trackingalloc so we go throgh the .inner to get to and call other functions onit
        ALLOCATOR.inner.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

pub fn print_heap_stats(){
    let used = ALLOCATED.load(Ordering::SeqCst);
    crate::println!(
        "heap Size: {} KiB, used: {} bytes ({} KiB)",
        HEAP_SIZE/1024,
        used,
        used/1024
    )
}

pub fn allocated_bytes() -> usize{
    ALLOCATED.load(Ordering::SeqCst)
}