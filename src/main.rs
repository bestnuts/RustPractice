use std::io;
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

fn int_to_alphabet(num: u8) -> Option<String> {
    let alphabet = (b'a'..=b'z').map(char::from);
    let alphabet_vec: Vec<char> = alphabet.collect();
    Some(alphabet_vec[(num - 1) as usize].to_string())
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
    show_map(map.iter().map(|s| s.as_str()).collect(), grid);
} //맵에 있는 알파벳 사용 시 공백으로 대체하기 and 알파벳 조합 예외처리 해주기