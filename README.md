# future-bool
A bool one can `await` the changes.

## Example usage
```rust
use future_bool::FutureBool;

#[tokio::main]
async fn main() {
    let run = FutureBool::new(false);
    let run_clone = run.clone();

    let task = tokio::spawn(async move {
    	loop {
	    // When run is false, some_async_fn will be interrupted.
	    tokio::select! {
	    	_ = run_clone.wait_false() => break,
		_ = some_async_fn() => {}
	    };
	}
    });

    // ... some other task sets run to false with 
    // `run.unset()`

    task.await;
}
```
