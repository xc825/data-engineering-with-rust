/*
* How would you modify the program to include more forks or philosophers?
* Can you implement a feature to dynamically add philosophers or forks during runtime?
* How would you measure the "hunger level" of each philosopher and implement a priority system?
*/
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Fork {
    id: u32,
    //mutex: Mutex<()>,
}

impl Clone for Fork {
    fn clone(&self) -> Fork {
        Fork { id: self.id }
    }
}

impl Copy for Fork {}

struct Forks {
    free_forks: Vec<Fork>,
    busy_forks: Vec<Fork>,
    condvar: Condvar,
}

impl Forks {
    fn new(n: u32) -> Forks {
        let mut free_forks = Vec::new();
        for i in 0..n {
            free_forks.push(Fork { id: i });
        }
        Forks {
            free_forks,
            busy_forks: Vec::new(),
            condvar: Condvar::new(),
        }
    }

    fn clone(&self) -> Forks {
        Forks {
            free_forks: self.free_forks.clone(),
            busy_forks: self.busy_forks.clone(),
            condvar: Condvar::new(),
        }
    }

    fn get_two_forks(&mut self) -> Option<(Fork, Fork)> {
        if self.free_forks.len() < 2 {
            return None;
        }
        let left_fork = self.free_forks.pop().unwrap();
        let right_fork = self.free_forks.pop().unwrap();
        self.busy_forks.push(left_fork.clone());
        self.busy_forks.push(right_fork.clone());
        Some((left_fork, right_fork))
    }

    fn put_both_forks(&mut self, left_fork: Fork, right_fork: Fork) {
        self.free_forks.push(left_fork);
        self.free_forks.push(right_fork);
        self.busy_forks
            .retain(|f| f.id != left_fork.id && f.id != right_fork.id);
    }
}

struct Philosopher {
    id: u32,
    name: String,
    left_fork: Option<Fork>,
    right_fork: Option<Fork>,
}

impl Philosopher {
    fn new(id: u32, name: &str) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            left_fork: None,
            right_fork: None,
        }
    }

    fn eat(&self, forks: Arc<Mutex<Forks>>) {
        let mut forks = forks.lock().unwrap();
        let two_forks = forks.clone().get_two_forks();
        let (left_fork, right_fork) = match two_forks {
            Some((fork1, fork2)) => (fork1, fork2),
            None => {
                println!("Philosopher {} can't eat, not enough forks", self.id);
                return;
            }
        };
        println!("Philosopher {} is eating", self.id);
        thread::sleep(Duration::from_secs(3));
        forks.put_both_forks(left_fork, right_fork);
        println!("Philosopher {} is done eating", self.id);
        forks.condvar.notify_one();
    }
}

fn main() {
    let forks = Arc::new(Mutex::new(Forks::new(5)));
    let philosophers = vec![
        Philosopher::new(0, "Aristotle"),
        Philosopher::new(1, "Plato"),
        Philosopher::new(2, "Socrates"),
        Philosopher::new(3, "Pythagoras"),
        Philosopher::new(4, "Heraclitus"),
    ];
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let forks = Arc::clone(&forks);
            thread::spawn(move || {
                p.eat(forks);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
