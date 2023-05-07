use std::sync::atomic::AtomicUsize;
use std::cell::UnsafeCell;
use std::ptr::NonNull;

struct ArcData<T> {
    /// number of `Arc`s
    data_ref_count: AtomicUsize,
    /// number of `Arc`s and `Weak`s combined
    alloc_ref_count: AtomicUsize,
    /// the data. `None` if there's only weak pointers left.
    data: UnsafeCell<Option<T>>,
}

pub struct Arc<T> {
    weak: Weak<T>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Sync + Send> Send for Weak<T> {}
unsafe impl<T: Sync + Send> Sync for Weak<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    alloc_ref_count: AtomicUsize::new(1),
                    data_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

fn main() {
    println!("Hello, world!");
}
