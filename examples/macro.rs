fn main() {
    macro_rules! hello {
        () => {
            println!("H");
        }
    }

    hello!()
}