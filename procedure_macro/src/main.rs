use procedure_macro::log_info;

// Define the Log trait with associated functions
trait Log {
    fn info(msg: &str);
    fn warn(msg: &str);
    fn error(msg: &str);   
}

// Implement the Log trait for the Database struct
impl Log for Database {
    fn info(msg: &str) {
        // Your logging implementation here
        println!("INFO: {}", msg);
    }

    fn warn(msg: &str) {
        // Your logging implementation here
        println!("WARN: {}", msg);
    }

    fn error(msg: &str) {
        // Your logging implementation here
        println!("ERROR: {}", msg);
    }
}

#[derive(Debug)]
struct Database {
    url: String,
    connections: u32,
}

impl Database {
    fn new(url: String) -> Database {
        Database { url, connections: 0 }
    }

    fn connect(&mut self) {
        self.connections += 1;
        Database::info(&format!("new connection to {}", self.url));
        if self.connections >= 100 {
            Database::warn("too many connections");
        }
    }
}


fn main() {
    println!("Hello World");
    log_info!("Hello!");

    let mut db = Database::new("localhost:5433".to_string());

    for _ in 0..100 {
        db.connect();

    }

    db.connect();
}