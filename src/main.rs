use std::collections::HashMap;
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
    // let database = Database::new().expect("database::new() crash");
}

struct Database {
    // map: std::collections::HashMap,
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // let content = match fs::read_to_string("./a.txt") {
        //     Ok(val) => println!("{}", val),
        //     Err(err) => return Result::Err(err),
        // };
        let mut map = HashMap::new();
        let content = fs::read_to_string("./a.txt")?;
        for line in content.lines() {
            let (key, value) = line.split_once('\t').expect("crast");
            map.insert(key.to_string(), value.to_string());
        }
        Ok(Database {
            // map: HashMap::new(),
            map: map
        })
    }
}

// 1hr 33min 


