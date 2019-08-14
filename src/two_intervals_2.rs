#![feature(async_await)]

use std::time::Duration;
use tokio::timer::Interval;
use futures::join;

#[tokio::main]
async fn main() {

    let mut interval = Interval::new_interval(Duration::from_millis(3000));
    let mut interval2 = Interval::new_interval(Duration::from_millis(5000));

    let t1 = async move { 
        while interval.next().await.is_some() {
            println!("gagagagagaga");
        }
    };

    let t2 = async move { 
        while interval2.next().await.is_some() {
            println!("--->>>>>");
        }
    };

    join!(t1, t2);
}
