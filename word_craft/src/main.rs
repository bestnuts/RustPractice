use std::{io, vec};
use rand::Rng;

fn main() {
    start_game();
}

fn show_map(map: Vec<&str>, grid: u8) {
    let mut row: u8 = 0;
    let mut map_text: String = String::new();
    for _i in 0..map.len() {
        row += 1;
        if row % grid == 1 {
            map_text += "\n";
        }
        map_text = map_text + map[_i] + " ";
    }
    println!("{}", map_text);
}

fn util_map_clear(map: Vec<String>, word: String) -> Vec<String> {
    let mut tmp_map: Vec<String> = map.clone();
    let mut val_return: bool = true;
    let mut vec_char: Vec<char> = vec![];
    for char in word.trim().chars() {
        if let Some(index) = tmp_map.iter().position(|str| str == &char.to_string())  {
            tmp_map[index] = "\u{E000}".to_string();
        } else {
            vec_char.push(char);
            val_return = false;
        }
    }
    if val_return {
        return tmp_map;
    } else {
        println!("{:?} 부적합한 알파벳들 입니다.", vec_char);
        return map;
    }
}

fn util_map_done(map: Vec<String>, grid: u8) -> bool {
    map.iter().filter(|str| str.to_string() == "\u{E000}").count() == grid.pow(2) as usize
}

fn int_to_alphabet(num: u8) -> Option<String> {
    let alphabet = (b'a'..=b'z').map(char::from);
    let alphabet_vec: Vec<char> = alphabet.collect();
    Some(alphabet_vec[(num - 1) as usize].to_string())
}

fn is_only_alphabet(string: String) -> bool {
    let mut val_return: bool = true;
    for char in string.trim().chars() {
        if !char.is_alphabetic() {
            val_return = false;
            break;
        }
    }
    val_return
}

fn start_game() {
    let mut map: Vec<String> = Vec::new();
    let mut input_string: String = String::new();
    println!("맵 크기를 입력하시오.");
    io::stdin().read_line(&mut input_string).expect("Failed to read input");
    let grid: u8 = input_string.trim().parse::<u8>().unwrap();
    for _ in 0..grid.pow(2) {
        map.push(int_to_alphabet(rand::thread_rng().gen_range(1, 26)).unwrap());
    }
    println!("맵에 있는 알파벳을 사용해 단어를 조합하시오.");
    loop {
        show_map(map.iter().map(|s| s.as_str()).collect(), grid);
        input_string.clear();
        io::stdin().read_line(&mut input_string).expect("Failed to read input");
        if is_only_alphabet(input_string.clone()) {
            map = util_map_clear(map.clone(), input_string.clone());
        } else {
            println!("단어 조합에 쓰인 알파벳이 일치하지 않습니다.");
            continue;
        }
        if util_map_done(map.clone(), grid) {
            break;
        } else {
            continue;
        }
    }
    println!("게임을 완료하셨습니다!");
}