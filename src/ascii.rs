static DIGITS: [[&'static str;5]; 10] = [
    ["▉▉▉▉", "▉  ▉", "▉  ▉", "▉  ▉","▉▉▉▉"], //0
    [" ▉ ", "▉▉ ", " ▉ ", " ▉ ", "▉▉▉"], //1
    [" ▉▉ ", "▉  ▉", "  ▉ ", " ▉  ", "▉▉▉▉"], //2
    [" ▉▉ ", "▉  ▉", "  ▉ ", "▉  ▉", " ▉▉ "], //3
    ["   ▉", "  ▉▉", " ▉ ▉", "▉▉▉▉", "   ▉"], //4
    ["▉▉▉▉", "▉   ", " ▉▉ ", "   ▉", "▉▉▉ "], //5
    [" ▉▉▉", "▉   ", "▉▉▉▉", "▉  ▉", "▉▉▉▉"], //6
    ["▉▉▉▉", "   ▉", "  ▉ ", " ▉  ", "▉   "], //7
    [" ▉▉ ", "▉  ▉", " ▉▉ ", "▉  ▉", " ▉▉ "], //8
    ["▉▉▉▉", "▉  ▉", " ▉▉▉", "   ▉", "   ▉"], //9
];

static FADE_CHARACTERS: [&str; 5] = [
    "▉", "▓", "▒", "░", "░"
];

pub fn num_as_ascii(value: i32) -> String {
    let mut result: String = String::new();
    for i in 0..5 {
        let mut row_buf = String::new();
        let mut v = value;
        let mut digit_stack = vec![];
        while v > 0 {
            digit_stack.push(v % 10);
            v = v / 10;
        }
        while !digit_stack.is_empty() {
            let digit = digit_stack.pop().unwrap();
            row_buf.push_str(
                &*DIGITS[digit as usize][i]
                    .replace("▉", FADE_CHARACTERS[i])
                );
            row_buf.push(' ');
        }
        result.push_str(&*row_buf);
        result.push('\n');
    }
    result
}