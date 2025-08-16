use std::fs;

fn main() {
    let mut arguments = std::env::args().skip(1);
    // let key = arguments.next();
    let key = arguments.next().expect("key was not there");
    let value = arguments.next().unwrap();
    // match key {
    //     Some(val) => {
    //         println!("{}", val);
    //     }
    //     None => {
    //         println!("error");
    //     }
    // };
    // let key = arguments.next().unwrap();
    // unwrap if the thing is their return it or crash the program
    // println!("{}", key);
    // println!("{:?}", key);

    println!("the key is: {} and the value is: {}", key, value);

    // std::fs::write("./a.txt", contents);
    // let contents = "kartikey";
    let contents = format!("{}\t{}\n", key, value);
    fs::write("./a.txt", contents).unwrap();
    let database = Database::new();
}

struct Database {
    
}

impl Database {
    fn new() -> Database {
        Database {  }
    }
}