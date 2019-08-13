#![feature(async_await)]

use tokio::timer::Interval;
use std::time::Duration;

#[tokio::main]
async fn main() {

    let mut interval = Interval::new_interval(Duration::from_millis(3000));

    while interval.next().await.is_some() {
        println!("gagagagagaga");
    }
}
