use std::sync::atomic::AtomicUsize;
use std::ptr::NonNull;
use std::ops::Deref;
use std::sync::atomic::Ordering::Relaxed;

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

// Arc represents shared ownership, so doesn't implement DerefMut
impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc {
            ptr: self.ptr,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
