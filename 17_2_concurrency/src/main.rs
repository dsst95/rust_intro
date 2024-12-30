use std::future::Future;
use std::pin::{pin, Pin};
use std::time::{Duration, Instant};

use trpl::Either;

fn main() {
  trpl::run(async {
    let fut1 = async {
      for i in 1..10 {
        println!("hi number {i} from the first task!");
        trpl::sleep(Duration::from_millis(500)).await;
      }
    };

    let fut2 = async {
      for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
      }
    };

    trpl::join(fut1, fut2).await;

    let (tx, mut rx) = trpl::channel();

    let tx1 = tx.clone();
    let tx1_fut = pin!(async move {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
      ];

      for val in vals {
        tx1.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
      }
    });

    let rx_fut = pin!(async {
      while let Some(value) = rx.recv().await {
        println!("received '{value}'");
      }
    });

    let tx_fut = pin!(async move {
      let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
      ];

      for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(1500)).await;
      }
    });

    let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

    trpl::join_all(futures).await;

    let slow = async {
      println!("'slow' started.");
      trpl::sleep(Duration::from_millis(100)).await;
      println!("'slow' finished.");
    };

    let fast = async {
      println!("'fast' started.");
      trpl::sleep(Duration::from_millis(50)).await;
      println!("'fast' finished.");
    };

    trpl::race(slow, fast).await;

    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    async {
      for _ in 1..1000 {
        trpl::sleep(one_ns).await;
      }
    }
    .await;
    let time = Instant::now() - start;
    println!(
      "'sleep' version finished after {} seconds.",
      time.as_secs_f32()
    );

    let start = Instant::now();
    async {
      for _ in 1..1000 {
        trpl::yield_now().await;
      }
    }
    .await;
    let time = Instant::now() - start;
    println!(
      "'yield' version finished after {} seconds.",
      time.as_secs_f32()
    );

    let slow = async {
      trpl::sleep(Duration::from_secs(5)).await;
      "Finally finished"
    };

    match timeout(slow, Duration::from_secs(2)).await {
      Ok(message) => println!("Succeeded with '{message}'"),
      Err(duration) => {
        println!("Failed after {} seconds", duration.as_secs())
      }
    }
  });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
  match trpl::race(future_to_try, trpl::sleep(max_time)).await {
    Either::Left(output) => Ok(output),
    Either::Right(_) => Err(max_time),
  }
}
