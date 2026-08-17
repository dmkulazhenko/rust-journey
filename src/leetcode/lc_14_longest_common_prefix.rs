pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res: String = String::new();
    let mut idx: usize = 0;
    loop {
        for s in strs.iter() {
            if idx >= s.len() || s[idx..idx + 1] != strs[0][idx..idx + 1] { return res; }
        }
        res += &strs[0][idx..idx + 1];
        idx += 1;
    }
}

fn main() {
    println!("{}", longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
}
