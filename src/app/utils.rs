
pub fn fill_string(len: isize, ch: char) -> String {
    if len == 0 {
        "".into()
    } else {
        (0..len).map(|_| ch).collect::<String>()
    }
}