use anyhow::Context;
use error_log::ErrorLogAnyhow;
use std::sync::{Arc, Mutex};
use tokio::{spawn as tspawn, task::JoinHandle};

#[tokio::main]
async fn main() {
    let err_log = Arc::new(Mutex::new(ErrorLogAnyhow::new_anyhow()));
    err_log
        .lock()
        .unwrap()
        .instant_display(true)
        .delimiter("\n");

    let mut tasks: Vec<JoinHandle<i32>> = vec![
        run(err_log.clone(), 0),
        run(err_log.clone(), 1),
        run(err_log.clone(), 2),
        run(err_log.clone(), 3),
        run(err_log.clone(), 4),
        run(err_log.clone(), 5),
    ];
    err_log.lock().unwrap().display_take();
    let mut i;
    while !tasks.is_empty() {
        i = 0;
        while i < tasks.len() {
            if tasks[i].is_finished() {
                tasks.remove(i);
                // do stuff with value
            }
            i += 1;
        }
    }
}

fn run(err_log: Arc<Mutex<ErrorLogAnyhow<i32>>>, id: u8) -> JoinHandle<i32> {
    tspawn(async move {
        if let Some(out) = err_log.lock().unwrap().push_result(
            "abc"
                .parse::<i32>()
                .with_context(|| format!("Error in thread {id}")),
        ) {
            return out;
        }
        if let Some(out) = err_log.lock().unwrap().push_result(
            "123"
                .parse::<i32>()
                .with_context(|| format!("Error in thread {id}")),
        ) {
            return out;
        }
        32
    })
}
