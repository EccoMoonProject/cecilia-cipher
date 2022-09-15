use rand::prelude::*;

pub fn chop_string(s: &str) -> String {
    // get some random data:
    let mut data = [0u8; 256];
    rand::thread_rng().fill_bytes(&mut data);

    // change the order of letters randomly
    let mut letters: Vec<char> = s.chars().collect();
    letters.shuffle(&mut rand::thread_rng());
    let shuffled = letters.into_iter().collect::<String>();
    let shuffled_without_spaces = shuffled.replace(" ", "");
    let mut chopped = shuffled_without_spaces.to_string();
    chopped.truncate(8);
    chopped.push_str(
        &data
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>(),
    );
    let mut chopped_as_chars: Vec<char> = chopped.chars().collect();
    chopped_as_chars.shuffle(&mut rand::thread_rng());
    let collected = chopped_as_chars.into_iter().collect::<String>();
    collected
}

pub fn chop_twice(s: String) -> String {
    let mut chopped = chop_string(&s);
    chopped = chop_string(&chopped);
    // get all numbers from string
    let numbers: Vec<char> = chopped.chars().filter(|c| c.is_digit(10)).collect();
    let d = [chopped, numbers.into_iter().collect::<String>()].join("");
    // change the order randomly
    let mut chars: Vec<char> = d.chars().collect();
    chars.shuffle(&mut rand::thread_rng());
    let collected = chars.into_iter().collect::<String>();
    // fix the length with random ordering
    let mut fixed = collected.to_string();
    fixed.truncate(800);
    fixed
}

pub fn cecilia_cipher(s: &str) -> String {
    let mut data = chop_string(s);
    data = chop_twice(data);
    data
}

fn main() {
    let s = "My Dear Cecilia!";
    let encrypted = cecilia_cipher(s);
    println!("{} -> {}", s, encrypted);
    let lengtb = encrypted.len();
    println!("Length: {}", lengtb);
}
