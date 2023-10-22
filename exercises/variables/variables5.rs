// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let mut number = 3; // don't rename this variable
    let number = 3;   //可以用遮蔽（shadow）来覆盖前面的变量。,让他变为整形，因为他本来是字符型的
    println!("Number plus two is : {}", number + 2);
}
