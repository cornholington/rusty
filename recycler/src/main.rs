use std::sync::{Arc, Mutex, RwLock};

pub struct Recycler<T> {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    gc: Arc<Mutex<Vec<Arc<RwLock<T>>>>>,
}

impl<T: Default> Default for Recycler<T> {
    fn default() -> Recycler<T> {
        Recycler {
            gc: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl<T: Default> Clone for Recycler<T> {
    fn clone(&self) -> Recycler<T> {
        Recycler {
            gc: self.gc.clone(),
        }
    }
}

impl<T: Default + Reset> Recycler<T> {
    fn recycle(&self, x: Arc<RwLock<T>>) {
        let mut gc = gc.lock().expect("recycler lock in pub fn recycle");
        gc.push((x, who));
    }

    pub fn allocate(&self) -> Arc<RwLock<T>> {
        let mut gc = self.gc.lock().expect("recycler lock in pb fn allocate");

        loop {
            if let Some((x, who)) = gc.pop() {
                // Only return the item if this recycler is the last reference to it.
                // Remove this check once `T` holds a Weak reference back to this
                // recycler and implements `Drop`. At the time of this writing, Weak can't
                // be passed across threads ('alloc' is a nightly-only API), and so our
                // reference-counted recyclables are awkwardly being recycled by hand,
                // which allows this race condition to exist.
                if Arc::strong_count(&x) >= 1 {
                    // Commenting out this message, is annoying for known use case of
                    //   validator hanging onto a blob in the window, but also sending it over
                    //   to retransmmit_request

                    eprintln!(
                        "Recycled item from \"{}\" still in use. {} Booting it.",
                        who,
                        Arc::strong_count(&x)
                    );
                    continue;
                }

                {
                    let mut w = x.write().unwrap();
                    w.reset();
                }
                return x;
            } else {
                return Arc::new(RwLock::new(Default::default()));
            }
        }
    }
}

pub type SharedBlob = Arc<RwLock<Blob>>;

pub struct Blob {
    recycler: Option<Arc<Recycler>>,
}

impl Drop for Blob {
    fn drop(&self) {}
}

fn main() {
    let recycler = Default::default();
    assert!(recycler.len() == 0);
    let blob: char = recycler.allocate();
}
