fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|file_content| file_content
                .lines()
                .map(|symbol| symbol.parse::<u32>().unwrap())
                .sum::<u32>())
            .max()
            .unwrap()
    );
}
