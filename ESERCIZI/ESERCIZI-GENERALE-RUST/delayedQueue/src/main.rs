/*
    DelayedTaskScheduler

    Devi implementare una struttura dati chiamata "DelayedTaskScheduler" che consenta l'esecuzione ritardata di compiti (tasks) in un
    sistema concorrente. I compiti avranno un ritardo specificato in millisecondi prima dell'esecuzione. La struttura deve essere thread-safe e
    garantire che i compiti vengano eseguiti in base al ritardo specificato.

    **Descrizione:**
    La "DelayedTaskScheduler" dovrebbe consentire l'aggiunta di compiti con un ritardo specificato. I compiti dovrebbero
    essere eseguiti in ordine di ritardo, in modo che quelli con il ritardo minore vengano eseguiti prima. Se più compiti
    hanno lo stesso ritardo, possono essere eseguiti in qualsiasi ordine.

    La struttura deve offrire i seguenti metodi:

    1. `schedule_task(task: Task, delay: Duration)` - Aggiunge un compito con il ritardo specificato.
    2. `run_next_task()` - Esegue il compito successivo con il ritardo minore. Se non ci sono compiti pronti per l'esecuzione, attende senza consumare cicli di CPU fino a quando un compito è pronto.

    **Requisiti:**
    1. La struttura deve essere thread-safe per consentire l'aggiunta e l'esecuzione di compiti da parte di diversi thread contemporaneamente.
    2. I compiti devono essere eseguiti nell'ordine in base al ritardo specificato.
    3. Il metodo `run_next_task()` deve attendere se non ci sono compiti pronti per l'esecuzione.
    4. Deve essere possibile chiudere la struttura in modo controllato per terminare l'attesa e l'esecuzione dei compiti rimanenti.
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

type Task = Box<dyn FnOnce() + Send + 'static>;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct DelayTask<Task> {
    task: Task,
    delay: Duration,
}

struct DelayedTaskScheduler<Task> {
    tasks: Mutex<BinaryHeap<DelayTask<Task>>>,
    cvar: Condvar,
}

impl<Task> DelayedTaskScheduler<Task> {
    fn new() -> Self {
        Self {
            tasks: Mutex::new(BinaryHeap::new()),
            cvar: Condvar::new(),
        }
    }

    fn schedule_task(&self, task: Task, delay: Duration) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(DelayTask { task, delay });
        self.cvar.notify_all();
    }

    fn run_text_task(&self) {
        loop {
            let mut tasks = self.tasks.lock().unwrap();
            tasks = self
                .cvar
                .wait_while(tasks, |value| value.len() == 0)
                .unwrap();

            if let Some(delay_task) = tasks.pop() {
                tasks = self.cvar.wait_timeout(tasks, delay_task.delay).unwrap().0;
                (delay_task.task)();
            }

            if tasks.len() == 0 {
                break;
            }
        }
    }
}

fn main() {
    let scheduler = Arc::new(DelayedTaskScheduler::new());

    // Example: Schedule three tasks with different delays
    scheduler.schedule_task(
        Box::new(|| println!("Task 1 executed")),
        Duration::from_secs(2),
    );
    scheduler.schedule_task(
        Box::new(|| println!("Task 2 executed")),
        Duration::from_secs(1),
    );
    scheduler.schedule_task(
        Box::new(|| println!("Task 3 executed")),
        Duration::from_secs(3),
    );

    // Execute tasks
    let cloned_scheduler = Arc::clone(&scheduler);
    thread::spawn(move || {
        cloned_scheduler.run_next_task();
    })
    .join()
    .unwrap();

    let cloned_scheduler = Arc::clone(&scheduler);
    thread::spawn(move || {
        cloned_scheduler.run_next_task();
    })
    .join()
    .unwrap();

    let cloned_scheduler = Arc::clone(&scheduler);
    thread::spawn(move || {
        cloned_scheduler.run_next_task();
    })
    .join()
    .unwrap();
}
