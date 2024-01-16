fn main() {
    let raw_pointer: *const i32 = 0x1234 as *const i32;  // Example raw pointer

    unsafe {
        // Dereferencing the raw pointer
        // let value: i32 = *raw_pointer;

        // Output the value
        // println!("Value: {}", value);
    }

    let mut s = "Let's Get Rusty".to_owned();

    let raw1 = &mut s as *const String;
    let raw2 = &mut s as *mut String;


    let address = 0x12345usize;

    let raw3 = address as *const String;

    unsafe {
        (*raw2).push_str("!!!");
        println!("raw1 is: {}", *raw1);
    }
}