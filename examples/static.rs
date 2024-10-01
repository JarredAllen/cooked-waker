//! An example of how to work with statically-allocated wakers.

use cooked_waker::{IntoWaker, WakeRef};

use core::{
    sync::atomic::{AtomicU32, Ordering},
    task::Waker,
};

struct CustomWaker {
    wake_count: AtomicU32,
}
impl Drop for CustomWaker {
    fn drop(&mut self) {
        println!("Drop is never called");
    }
}

impl WakeRef for CustomWaker {
    fn wake_by_ref(&self) {
        let count = self.wake_count.fetch_add(1, Ordering::Relaxed);
        println!("wake by ref #{}", count);
    }
}

fn main() {
    println!("Hello, world!");
    static WAKER: CustomWaker = CustomWaker {
        wake_count: AtomicU32::new(0),
    };
    let waker1: Waker = WAKER.into_waker();

    let waker2 = waker1.clone();

    // Clones of a waker from `<&'static T as IntoWaker>` will share the data pointer
    println!("Waker1: {:?}", waker1);
    println!("Waker2: {:?}", waker2);

    waker1.wake_by_ref();
    waker1.wake();

    waker2.wake_by_ref();
    waker2.wake();
}
