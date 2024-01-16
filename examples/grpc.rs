use std::arch::asm;

#[link(name = "add", kind = "static")]
extern "C" {
    fn add_from_c(a: i32, b: i32) -> i32;
}


fn add_asm(a: i32, b: i32) -> i32 {
    let result;
    unsafe {
        asm!("add {0}, {0}, {1}" : "+r"(a) : "r"(b) : "cc");
    }
    result
}


fn main() {
    unsafe {
        println!("Hello, world! {}", add_from_c(10, 32));
        println!("Hello, world! {}", add_asm(10, 32));
    }
}
