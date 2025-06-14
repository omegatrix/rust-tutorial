use std::{
    pin::{self, Pin, pin},
    thread::sleep,
    time::Duration,
};

use trpl::{Either, Html, ReceiverStream, Stream, StreamExt};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}

fn main() {
    // Web scrapper
    // let args: Vec<String> = std::env::args().collect();

    // trpl::run(async {
    //     let title_fut_1 = page_title(&args[1]);
    //     let title_fut_2 = page_title(&args[2]);

    //     let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };

    //     println!("{url} returned first");
    //     match maybe_title {
    //         Some(title) => println!("Its page title is: '{title}'"),
    //         None => println!("Its title could not be parsed."),
    //     }
    // })

    // Threads
    // trpl::run(async {
    //     trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("hi number {i} from the second task!");
    //         sleep(Duration::from_millis(500));
    //     }
    // });

    // Futures
    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel();

    //     let tx1 = tx.clone();
    //     let tx1_fut = pin!(async move {
    //         let vals = vec![
    //             String::from("hi"),
    //             String::from("from"),
    //             String::from("the"),
    //             String::from("future"),
    //         ];

    //         for val in vals {
    //             tx1.send(val).unwrap();
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     let rx_fut = pin!(async {
    //         while let Some(value) = rx.recv().await {
    //             println!("received '{value}'");
    //         }
    //     });

    //     let tx_fut = pin!(async move {
    //         let vals = vec![
    //             String::from("more"),
    //             String::from("messages"),
    //             String::from("for"),
    //             String::from("you"),
    //         ];

    //         for val in vals {
    //             tx.send(val).unwrap();
    //             trpl::sleep(Duration::from_millis(1500)).await;
    //         }
    //     });

    //     let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

    //     trpl::join_all(futures).await;

    //     let slow = async {
    //         println!("'slow' started.");
    //         trpl::sleep(Duration::from_millis(100)).await;
    //         println!("'slow' finished.");
    //     };

    //     let fast = async {
    //         println!("'fast' started.");
    //         trpl::sleep(Duration::from_millis(50)).await;
    //         println!("'fast' finished.");
    //     };

    //     trpl::race(slow, fast).await;
    // });

    // trpl::run(async {
    //     let slow = async {
    //         trpl::sleep(Duration::from_secs(5)).await;
    //         "Finally finished"
    //     };

    //     match timeout(slow, Duration::from_secs(2)).await {
    //         Ok(message) => println!("Succeeded with '{message}'"),
    //         Err(duration) => {
    //             println!("Failed after {} seconds", duration.as_secs())
    //         }
    //     }
    // });

    // Streams
    // trpl::run(async {
    //     let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //     let iter = values.iter().map(|n| n * 2);
    //     let mut stream = trpl::stream_from_iter(iter);
    //     let mut filtered = stream.filter(|val| val % 3 == 0 || val % 5 == 0);

    //     while let Some(value) = filtered.next().await {
    //         println!("The value was: {value}");
    //     }
    // });

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);
    });
}
