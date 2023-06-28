use std::sync::atomic::AtomicU32;

#[cfg(not(target_os = "linux"))]
compile_error!("Linux only. Sorry!");

pub fn wait(a: &AtomicU32, expected: u32) {
    // Refer to the futex (2) man page for the syscall signature.
    unsafe {
        libc::syscall(
            libc::SYS_futex, // The futex syscall.
            a as *const AtomicU32, // The atomic to operate on.
            libc::FUTEX_WAIT, // The futex operation.
            expected, // The expected value.
            std::ptr::null::<libc::timespec>(), // No timeout.
        );
    }
}

pub fn wake_one(a: &AtomicU32) {
    // Refer to the futex (2) man page for the syscall signature.
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            a as *const AtomicU32,
            libc::FUTEX_WAKE,
            1, // The number of threads to wake up.
        );
    }
}

fn main() {
    println!("Hello, world!");
}
