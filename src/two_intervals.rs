#![feature(async_await)]

use tokio::timer::Interval;
use std::time::Duration;

#[tokio::main]
async fn main() {

    let mut interval = Interval::new_interval(Duration::from_millis(3000));
    let mut interval2 = Interval::new_interval(Duration::from_millis(5000));

    tokio::spawn(async move { 
        while interval.next().await.is_some() {
            println!("gagagagagaga");
        }
    });

    tokio::spawn(async move { 
        while interval2.next().await.is_some() {
            println!("--->>>>>");
        }
    });

    loop {
    }
}
