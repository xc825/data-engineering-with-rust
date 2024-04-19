fn remove_last_fruit<'a>(fruits: &mut Vec<&'a str>) -> &'a str {
    let removed = fruits.pop().unwrap();
    if fruits.is_empty() {
        panic!("No fruits to remove");
    } else {
        println!("removed: {}", removed);
    }
    removed
}

fn sort_fruits(fruits: &mut [&str]) {
    fruits.sort();
}

fn fruits_occurrences(fruits: &[&str]) {
    let mut fruits_occurrences: std::collections::HashMap<&str, i32> =
        std::collections::HashMap::new();
    for fruit in fruits.iter() {
        let count = fruits_occurrences.entry(fruit).or_insert(0);
        *count += 1;
    }
    for (fruit, count) in fruits_occurrences.iter() {
        println!("{}: {}", fruit, count);
    }
}

fn main() {
    let mut fruits: Vec<&str> = vec![
        "apple",
        "banana",
        "cherry",
        "blueberry",
        "orange",
        "kiwi",
        "pineapple",
    ];
    println!("Original fruits:{:?}", &fruits);
    fruits.push("strawberry");
    let last_num = fruits.len() - 1;
    for (i, fruit) in fruits.iter().enumerate() {
        if i == last_num {
            println!("{}.", fruit);
        } else {
            print!("{}, ", fruit);
        }
    }
    remove_last_fruit(&mut fruits);
    println!("Fruits after removing the last fruit: {:?}", &fruits);

    sort_fruits(&mut fruits);
    println!("Fruits after sorting: {:?}", &fruits);

    fruits.push("apple");
    fruits_occurrences(&fruits);
}
