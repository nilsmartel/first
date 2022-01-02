fn main() {
    let input = {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("to read stdin");
        buffer
    };

    let word = input.split(&[' ', '\n', '\r', '\t'][..]).next().expect("to find next word");

    print!("{}", word);
}
