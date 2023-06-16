use std::pin::{Pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::{time::Duration};
use std::future::{Future};

#[derive(Clone,Copy)]
enum TimerState {
    Init(Duration),
    Running,
    Ready,
}
struct Delay(Arc<Mutex<TimerState>>);

impl Delay {
    fn in_secs(delay:u64) -> Delay {
        Delay(Arc::new(Mutex::new(TimerState::Init(Duration::from_secs(delay)))))
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        let state = self.0.clone();
        match &mut *state.lock().unwrap() {
            TimerState::Init(x) => {
                *state.lock().unwrap() = TimerState::Running;
                thread::spawn(move || {
                    thread::sleep(x);
                    cx.waker().clone().wake();
                    *state.lock().unwrap() = TimerState::Ready;
                });
                Poll::Pending
            }
            TimerState::Running => 
            {
                cx.waker().clone().wake();
                Poll::Pending
            }
            TimerState::Ready => Poll::Ready(())
        }

    }
}


async fn greet(n: u64) {
    println!("Hi! {}",n);
    Delay::in_secs(n).await;
    println!("Bye! {}",n);
}

fn main() {
    futures::executor::block_on( async {
        futures::join!(greet(1),greet(2),greet(3));
    })
}
