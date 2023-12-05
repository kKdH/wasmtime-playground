#[no_mangle]
pub extern "C" fn start(lhs: i32, rhs: i32) -> i32 {
    println!("Hello from Plugin");
    lhs.wrapping_add( rhs)
}
