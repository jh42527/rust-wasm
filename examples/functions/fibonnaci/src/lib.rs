extern crate image;

#[no_mangle]
pub extern "C" fn fibonacci(n: i64) {
    let mut previous: i64 = 0;
    let mut current: i64 = 1;

    for _count in 1..n {
        current = current + previous;
        previous = current - previous;
    }
    
    println!("{current}");
}