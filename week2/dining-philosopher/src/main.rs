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

    fn get_two_forks(&mut self, forks: Arc<Mutex<Forks>>) -> Result<(), &'static str>{
        let mut forks = forks.lock().unwrap();
        if forks.free_forks.len() < 2 {
            Err("Not enough forks")
        } else {
            self.left_fork = Some(forks.free_forks.pop().unwrap().clone());
            self.right_fork = Some(forks.free_forks.pop().unwrap().clone());
            forks.busy_forks.push(self.left_fork.unwrap().clone());
            forks.busy_forks.push(self.right_fork.unwrap().clone());
            Ok(())
        }
    }

    fn put_both_forks(&mut self, forks: Arc<Mutex<Forks>>) {
        let mut forks = forks.lock().unwrap();
        forks.free_forks.push(self.left_fork.unwrap());
        forks.free_forks.push(self.right_fork.unwrap());
        forks.busy_forks.retain(|f| f.id != self.left_fork.unwrap().id && f.id != self.right_fork.unwrap().id);
    }

    fn eat(&mut self, forks: Arc<Mutex<Forks>>) {
        if self.get_two_forks(forks.clone()).is_err() {
            println!("Philosopher {} is hungry", self.id);
        }
        else {
            println!("Philosopher {} is eating with forks {}, {}",
            self.id,
            self.left_fork.unwrap().id,
            self.right_fork.unwrap().id);
            thread::sleep(Duration::from_secs(2));
            self.put_both_forks(forks.clone());
            println!("Philosopher {} is done eating", self.id);
        }
        //forks.condvar.notify_one();
    }
}

fn main() {
    let forks = Arc::new(Mutex::new(Forks::new(5)));
    let philosophers = vec![
        Philosopher::new(0, "Aristotle "),
        Philosopher::new(1, "Plato     "),
        Philosopher::new(2, "Socrates  "),
        Philosopher::new(3, "Pythagoras"),
        Philosopher::new(4, "Heraclitus"),
    ];
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|mut p| {
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
