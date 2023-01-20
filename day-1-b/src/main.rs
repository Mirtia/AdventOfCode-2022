fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|str_content| str_content
                .lines()
                .map(|symbol| symbol.parse::<u32>().unwrap())
                .sum::<u32>())
            .collect::<std::collections::BinaryHeap<u32>>()
            .into_sorted_vec()
            .iter()
            .rev()
            .take(3)
            .sum::<u32>()
    );
}
