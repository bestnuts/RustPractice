use std::io;
use rand::Rng;

const NUMBER_MAX: u8 = 31;
static mut PLAYER_CNT: u8 = 0;

fn main() {
    start_game();
}

fn start_game() {
    let mut input_string: String = String::new();
    let mut player_turn: u8 = 0;
    let mut sum_int: u8 = 0;
    let mut curr_turn_cnt: u8 = 0;
    println!("자신을 포함 몇명이서 플레이할지 정하시오.\n자신을 제외한 모든 플레이어는 AI 입니다.");
    io::stdin().read_line(&mut input_string).expect("Failed to read input");
    unsafe { PLAYER_CNT = input_string.trim().parse::<u8>().unwrap() };
    loop {
        if player_turn == 0{
            println!("당신의 차례입니다. 문자를 입력하면 다음 차례로 넘깁니다.");
            input_string.clear();
            io::stdin().read_line(&mut input_string).expect("Failed to read input");
            if input_string.trim().is_empty() && curr_turn_cnt < 3 {
                sum_int += 1;
                curr_turn_cnt += 1;
                println!("You: {}", sum_int);
                continue;
            } else if curr_turn_cnt == 0 {
                println!("무조건 한 번 이상 숫자를 외쳐야 합니다.");
                continue;
            }
        } else {
            let execute = rand::thread_rng().gen_range(1, 3);
            for _i in 0..execute {
                sum_int += 1;
                println!("AI_{}: {}", player_turn, sum_int);
            }
        }
        if sum_int >= NUMBER_MAX {
            break;
        } else {
            player_turn += 1;
            if player_turn >= unsafe { PLAYER_CNT } {
                player_turn = 0;
                curr_turn_cnt = 0;
            }
        }
    }
    if player_turn == 0{
        println!("게임이 종료되었습니다.\n해당 게임에서 패배자는 당신입니다.")
    } else {
        println!("게임이 종료되었습니다.\n해당 게임에서 패배자는 AI_{} 입니다.", player_turn)
    }
}