use queue::Queue;
pub mod queue;

fn main() {
    let mut qu: Queue = Queue::new();
    let mut elem: i32;

    qu.insert(12345678);
    qu.insert(668);
    qu.insert(438754);
    qu.insert(872);
    qu.insert(2);
    qu.insert(292910);
    
    println!("Welcome to Tomé Bank - here your money is more rentable");
    println!("********************************************************");

    while !qu.is_empty() {
        elem = qu.remove();
        println!("Current queue position: {}", elem);
    }
}
