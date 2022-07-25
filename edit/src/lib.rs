static mut total_time: f32 = 0f32;

#[no_mangle]
pub extern "C" fn tick(input: u64, delta_s: f32) -> u32 {
    unsafe { total_time += delta_s; }
    println!("hi from edit! total_time: {}", unsafe { total_time });
    1
}
