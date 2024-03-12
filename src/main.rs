static VAL_FIRST_RAND: i32 = 7;
static mut VAL_DELTA_RAND: i32 = 17;

fn main() {
    for i in 0..10 {
        out_rand(99);
        unsafe {
            println!("result = {0} | loop number = {1}", VAL_DELTA_RAND, i);
        }
    }
}

fn out_rand(max: i32) {
    unsafe { VAL_DELTA_RAND = ((VAL_FIRST_RAND * VAL_DELTA_RAND + VAL_FIRST_RAND) + 1) % max };
}