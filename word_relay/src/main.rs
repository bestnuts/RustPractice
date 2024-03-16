use std::io;

static WORD_LIST: [&str; 5] = ["엄준식", "첫말", "복소수", "선형합동법", "내리막길"];
static WRONG_WORD_LIST: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

fn main() {
    start_game();
}

fn start_game() {
    let mut input_string = String::new();
    let mut word_list_index: usize;
    loop {
        println!("0 ~ {} 숫자 사이를 입력해 시작합니다.", WORD_LIST.len() - 1);
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

fn util_word_replace(string: &str) -> bool {
    let mut replace = string.to_string().to_lowercase();
    for p in 0..9 {
        if replace.contains(p.to_string().as_str()) {
            return false;
        }
    }
    for q in WRONG_WORD_LIST {
        if replace.contains(q) {
            return false;
        }
    }
    if replace.contains(" ") {
        return false;
    }
    return true;
}

fn word_relay(mut last_word: String) {
    let last_char: char = last_word.chars().last().unwrap_or(' ');
    println!("'{}'를 첫 단어로 사용해주세요.", last_char);
    let mut input_word: String = String::new();
    loop {
        io::stdin().read_line(&mut input_word).expect("Failed to read input");
        let mut word_vec: Vec<char> = input_word.trim().to_string().chars().collect::<Vec<char>>();
        if util_word_replace(input_word.as_str()) {
            if word_vec.len() > 1{
                let curr_char: char = input_word.chars().next().unwrap_or(' ');
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
        } else {
            println!("문장에 사용되지 못 하는 글자가 있습니다.");
            input_word.clear();
            continue;
        }
    }
    last_word = input_word.trim().to_string();
    word_relay(last_word);
}