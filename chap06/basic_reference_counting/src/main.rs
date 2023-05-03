use std::sync::atomic::AtomicUsize;
use std::ptr::NonNull;
use std::ops::Deref;
use std::sync::atomic::{
    fence,
    Ordering::{
        Acquire,
        Relaxed,
        Release,
    },
};

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
        // aborting the process is not instant, so we use usize::MAX / 2 as the limit
        // it's impossible to have so many threads concurrently because they take space in memory
        if self.data().ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc {
            ptr: self.ptr,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                // reclaim exclusive ownership with Box::from_raw()
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_our_ark() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);
    struct DetectDrop;
    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }

    let x = Arc::new(("hello", DetectDrop));
    let y = x.clone();

    // send x to another thread and use it there
    let t = std::thread::spawn(move || {
        assert_eq!(x.0, "hello");
    });
    // in parallel, y should be still usable here
    assert_eq!(y.0, "hello");
    t.join().unwrap();

    // x should be dropped by now
    // but y is alive, so no drop should have happened.
    assert_eq!(NUM_DROPS.load(Relaxed), 0);

    drop(y);
    assert_eq!(NUM_DROPS.load(Relaxed), 1);
}