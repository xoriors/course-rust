/*
Task: Implement an async worker pool that executes tasks concurrently using Tokio.
Requirements:

    The pool should spawn N worker threads.
    Implement a method submit_task(task: fn() -> ()) that executes a function in a worker.
    Use tokio::sync::mpsc to queue tasks.
    Use tokio::task::spawn for execution.
*/
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, mpsc};
use tokio::task;
use tokio::time::sleep;

struct WorkerPool {
    tx: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl WorkerPool {
    async fn new(worker_count: usize) -> Self {
        // Initialize pool
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send + 'static>>(100); // Channel for task submission
        let rx = Arc::new(Mutex::new(rx)); // Wrap in Mutex for shared access

        for _ in 0..worker_count {
            let rx_clone = rx.clone();
            task::spawn(async move {
                while let Some(task) = rx_clone.lock().await.recv().await {
                    tokio::task::spawn(async move {
                        task() // Execute the task
                    })
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("Task execution failed: {:?}", e);
                    });
                }
            });
        }

        WorkerPool { tx }
    }

    async fn submit_task(&self, task: fn() -> ()) {
        self.tx
            .clone()
            .send(Box::new(task))
            .await
            .unwrap_or_else(|e| {
                eprintln!("Failed to submit task: {:?}", e);
            });
    }
}

#[tokio::main]
async fn main() {
    let pool = WorkerPool::new(4).await;

    pool.submit_task(|| println!("Task 1 executed")).await;
    pool.submit_task(|| println!("Task 2 executed")).await;

    sleep(Duration::from_secs(1)).await;
}
