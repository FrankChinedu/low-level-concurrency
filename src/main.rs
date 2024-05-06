use std::thread;
fn main() {
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let name = thread::current();
    let name = name.name().unwrap();
    let average = t.join().unwrap();

    println!("average: {}, name: {:?}", average, name)
}
// fn f() {
//     println!("Hello from another thread!");
//     let id = thread::current().id();
//     println!("This is my thread id: {id:?}");
// }
