/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use clap::{App, Arg};
use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;

fn main() {
    let matches = App::new("Fruit Salad Maker")
        .arg(
            Arg::new("fruits")
                .long("fruits")
                .value_name("FRUITS")
                //.about("Sets a custom list of fruits")
                .takes_value(true)
                .multiple_values(true),
        )
        .arg(
            Arg::new("quantity")
                .short('q')
                .long("quantity")
                .value_name("QUANTITY")
                //.about("Sets the quantity of fruits to add")
                .takes_value(true)
                .default_value("0")
                .validator(|s| {
                    s.parse::<i32>()
                        .map(|_| ())
                        .map_err(|_| String::from("Quantity must be a valid integer"))
                }),
        )
        .get_matches();

    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    if let Some(values) = matches.values_of("fruits") {
        fruit.extend(values);
    }

    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    println!(
        "Random fruit: {:?}",
        fruit.choose(&mut rng).unwrap_or(&"Augļu nav!")
    );

    println!("Fruit Salad:");
    let mut q = matches
        .value_of("quantity")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    if q > fruit.len() as i32 {
        q = fruit.len() as i32;
    }
    for i in 0..q {
        if i != q - 1 {
            print!("{}, ", fruit.pop().unwrap());
        } else {
            println!("{}", fruit.pop().unwrap());
        }
    }
}
