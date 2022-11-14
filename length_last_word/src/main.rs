fn main() {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end()
            .chars()
            .rev()
            .take_while(|c| c.is_alphabetic())
            .count() as i32
    }

    let test_str = String::from(" this is a test ");
    // length_of_last_word(test_str);
    println!("{}", length_of_last_word(test_str));
}
