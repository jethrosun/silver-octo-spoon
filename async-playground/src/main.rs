use futures::executor::block_on;
use std::time::Duration;
use tokio::time::delay_for;

async fn job() {
    println!("work!")
}

async fn do_job() {
    let work = unsafe {
        async_timer::Timed::platform_new_unchecked(job(), core::time::Duration::from_secs(200))
    };

    match work.await {
        Ok(_) => println!("I'm done!"),
        //You can retry by polling `expired`
        Err(expired) => println!("Job expired: {}", expired),
    }
}

async fn do_a_while() {
    let mut times: u8 = 0;
    let mut interval = async_timer::Interval::platform_new(core::time::Duration::from_secs(10));

    while times < 5 {
        job().await;
        interval.as_mut().await;
        times += 1;
    }
}

fn test_main() {
    let fut = do_a_while();
    block_on(fut);
}

#[tokio::main]
async fn main() {
    delay_for(Duration::from_millis(100)).await;
    println!("100 ms have elapsed");
}
