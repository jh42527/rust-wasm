extern crate image;

use std::path::Path;

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

#[no_mangle]
pub extern fn webp_convert(input: &String, output: &String) {
    println!("Converting Image {input} {output}");

    let im = image::open(&Path::new(&input)).unwrap();

    im.save(&Path::new(&output)).unwrap();
}

#[no_mangle]
pub extern fn read_img(ptr: *mut u8, len: usize) {
    let img = unsafe { Vec::from_raw_parts(ptr, len, len) };

    if let Err(e) = image::load_from_memory(&img) {
        panic!("Invalid Input Buffer: {}", e)
    }
}