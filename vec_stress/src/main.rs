extern crate rand;
use rand::{thread_rng, Rng};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let recycler = Arc::new(Mutex::new(vec![Arc::new(RwLock::new(0i64))]));

    for _ in 0..100 {
        let thread_recycler = recycler.clone();
        thread::spawn(move || {
            let mut now = Instant::now();
            let mut local = Vec::new();
            loop {
                churn(&mut local, thread_recycler.lock().unwrap().deref_mut());

                if now.elapsed() >= Duration::from_millis(10) {
                    now = Instant::now();
                    local = Vec::new(); // drop all my refs, fail to recycle
                }
            }
        });
    }

    fn alloc(recycler: &mut Vec<Arc<RwLock<i64>>>) -> Arc<RwLock<i64>> {
        recycler.pop().unwrap_or(Arc::new(RwLock::new(0)))
    }

    fn free(recycler: &mut Vec<Arc<RwLock<i64>>>, val: Arc<RwLock<i64>>) -> () {
        recycler.push(val);
    }

    fn churn(
        local: &mut Vec<Arc<RwLock<i64>>>,
        recycler: &mut Vec<Arc<RwLock<i64>>>,
    ) -> (usize, i64) {
        let x = alloc(recycler);
        let val = *x.read().unwrap() + 1;
        *x.write().unwrap() = val;

        // don't let local get too big
        if local.len() == 100 {
            let from = thread_rng().gen_range(0, 99);
            let to = thread_rng().gen_range(from + 1, 100);

            let items = local.splice(from..to, vec![]);

            for item in items {
                free(recycler, item);
            }
        }

        local.push(x);

        (recycler.len(), val)
    }

    let mut now = Instant::now();
    let mut local = Vec::new();
    loop {
l        let (len, val) = churn(&mut local, recycler.lock().unwrap().deref_mut());
        if now.elapsed() >= Duration::from_millis(1000) {
            now = Instant::now();
            println!(
                "local.len() = {} recycler.len() = {} last val = {}",
                local.len(),
                len,
                val
            );
            local = Vec::new(); // drop all my refs, fail to recycle
        }
    }
}
