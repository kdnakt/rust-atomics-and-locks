use std::sync::atomic::AtomicUsize;
use std::ptr::NonNull;
use std::ops::Deref;

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

// Not a Box, which is exclusive ownership.
// We use pointer for shared ownership.
pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from( // turn it into a pointer
                Box::leak( // give up our exclusive ownership
                    // new allocation
                    Box::new(ArcData {
                        ref_count: AtomicUsize::new(1),
                        data,
                    })
                )
            ),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

fn main() {
    println!("Hello, world!");
}
