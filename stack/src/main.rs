use std::io;

use stack::Stack;
pub mod stack;

fn main() {
    println!("Begin of the program \n");
    let mut buf: String = String::new();
    let (mut integer_input, mut remainder): (i32, Option<i32>);

    let mut st: Stack = Stack::new();

    println!("Type some value bellow...     ");
    io::stdin().read_line(&mut buf)
        .ok()
        .expect("Couldn't read line");

    integer_input = buf.trim().parse::<i32>().unwrap();

    while integer_input != 0 {
        remainder = Some(integer_input % 2);
        st.push(remainder);
        integer_input = integer_input / 2;
    }

    while !st.is_empty() {
        remainder = st.pop();
        println!("{:?}", remainder.unwrap_or(0));
    }

    println!("\n End of the program");

}
