use future_bool::FutureBool;

macro_rules! sleep {
    ($sec:expr) => {
        ::tokio::time::sleep(::tokio::time::Duration::from_secs($sec)).await
    }
}

#[tokio::main]
async fn main() {
    let b: FutureBool = FutureBool::new(false);
    let mut tasks = Vec::new();

    for x in 0..20 {
        let clone = b.clone();
        if x % 2 == 0 {
            tasks.push(tokio::spawn(async move {
                clone.wait_true().await;
                eprintln!("{:2}: Yess", x);
            }));
        } else {
            tasks.push(tokio::spawn(async move {
                clone.wait_false().await;
                eprintln!("{:2}: Nope", x);
            }));
        }
    }
    
    sleep!(2);
    b.set();
    
    for task in tasks {
        let _ = task.await;
    }
}
