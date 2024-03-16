use std::{io, u32};

static mut VAL_DELTA_RAND: u32 = 17;

fn main() {
    start_game();
}

fn start_game() {
    println!("Input rand seed:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read input");
    let rand_seed: Result<u32, _> = input_string.trim().parse();

    match rand_seed {
        Ok(seed) => {
            finder_rand(output_rand(99, seed));
        }
        Err(_err) => {
            println!("Wrong seed Retry to start");
            start_game();
        }
    }
}

fn output_rand(max: u32, seed: u32) -> u32 {
    unsafe { 
        VAL_DELTA_RAND = ((seed * VAL_DELTA_RAND + seed) + 1) % max;
        VAL_DELTA_RAND
    }
}

fn finder_rand(secret_number: u32) {
    let mut guess_try: u32 = 0;
    loop {
        println!("Input answer:");
        guess_try += 1;
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).expect("Failed to read input");
        let guess_number: u32 = match input_string.trim().parse() {
            Ok(guess) => guess,
            Err(_err) => {
                continue;
            }
        };
        if guess_number == secret_number {
            println!("정답입니다. 당신이 입력한 수: {} | 정답 수: {} | 시도 수: {}", guess_number, secret_number, guess_try);
            break;
        } else if guess_number > secret_number {
            println!("정답 수는 당신이 입력한 수: {} 보다 낮습니다.", guess_number);
        } else {
            println!("정답 수는 당신이 입력한 수: {} 보다 높습니다.", guess_number);
        }
    }
}