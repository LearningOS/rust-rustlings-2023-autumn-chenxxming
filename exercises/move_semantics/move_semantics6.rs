// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership                    //获得所有权，引用是获得不了所有权的//& 符号即是引用，它们允许你使用值，但是不获取所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
