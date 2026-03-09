use crate::buffer::chunk::Chunk;

pub mod buffer;

fn main() {
    let test_str = ['\0'; 4];
    let mut test = Chunk::new(test_str, 0, 0);

    if let Err(e) = test.insert_char('a', 0) {
        println!("{}", "aaa");
    };
    test.insert_char('b', 1).unwrap();
    test.insert_char('b', 2).unwrap();
    test.insert_char('c', 3).unwrap();

    println!("{}", test.get_data());


    test.remove_char(3).unwrap();

    println!("{}", test.get_data());
}
