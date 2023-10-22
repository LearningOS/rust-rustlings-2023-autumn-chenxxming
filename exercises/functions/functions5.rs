// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.



fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
     num * num     //语句执行时不会返回值，但表达式返回时会返回一个值 ；这个题可以直接讲分号去掉，或者加一个return在前面
}