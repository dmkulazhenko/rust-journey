struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}

fn main() {
    println!("{}", Solution::length_of_last_word(String::from("   fly me   to   the moon  ")));
}
