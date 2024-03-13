use std::io;

static WORD_LIST: [&str; 2] = ["엄준식", "첫말"];

fn main() {
    start_game();
}

fn start_game() {
    loop {
        println!("0 ~ {} 숫자 사이를 입력해 시작합니다.", WORD_LIST.len() - 1);
        let mut input_string = String::new();
        let mut word_list_index: usize = 0;
        io::stdin().read_line(&mut input_string).expect("Failed to read input");
        word_list_index = match input_string.trim().parse() {
            Ok(index) => index,
            Err(err) => {
                println!("Wrong index err Retry to start");
                continue;
            }
        };
        if word_list_index >= WORD_LIST.len() {
            println!("IndexOutOfBound Current word list size: {}", WORD_LIST.len());
            continue;
        } else {
            word_relay(WORD_LIST[word_list_index].to_string());
            break;
        }
    }
}

fn word_relay(mut last_word: String) {
    let last_char = last_word.chars().last().unwrap_or(' ');
    println!("'{}'를 첫 단어로 사용해주세요.", last_char);
    let mut input_word = String::new();
    loop {
        io::stdin().read_line(&mut input_word).expect("Failed to read input");
        let char_vector: Vec<char> = input_word.chars().collect();
        if char_vector.len() > 1 {
            let curr_char = input_word.chars().next().unwrap_or(' ');
            if curr_char == last_char {
                break;
            } else {
                println!("'{}'를 첫 단어로 이어주세요.", last_char);
                input_word.clear();
                continue;
            }
        } else {
            println!("문장이 최소 두 글자 이상이여야 합니다.");
            input_word.clear();
            continue;
        }
    }
    last_word = input_word.trim().to_string();
    word_relay(last_word);
}//char_vector 길이 계산 and curr_char == last_char 비교 fix 필요