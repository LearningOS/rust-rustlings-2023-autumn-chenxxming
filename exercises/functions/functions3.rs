// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.



fn main() {
    let mut x : u32 =4 ;   //可以声明一个与函数里面的变量类型一样的东西，然后放进去
    call_me(x);
}

fn call_me(num: u32) {  //可以把里面的类型直接变为i32，因为默认类型就是i32；
    for i in 0..num {   //0..num表示的是[);         0..=num表示的是[]
        println!("Ring! Call number {}", i + 1);
    }
}
