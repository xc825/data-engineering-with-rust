fn function_with_ten_parameters(
    param1: i32,
    param2: i32,
    param3: i32,
    param4: i32,
    param5: i32,
    param6: i32,
    param7: i32,
    param8: i32,
    param9: i32,
    param10: i32,
) -> i32 {
    param1 + param2 + param3 + param4 + param5 + param6 + param7 + param8 + param9 + param10
}

fn main() {
    println!("Hello, world!");
    println!(
        "sum: {:?}",
        function_with_ten_parameters(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
    );
}
