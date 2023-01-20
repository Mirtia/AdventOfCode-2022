fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| line
                .split(&['-', ','])
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>())
            .filter(
                |num_range| (num_range[0] <= num_range[2] && num_range[1] >= num_range[3])
                    || (num_range[0] >= num_range[2] && num_range[1] <= num_range[3])
            )
            .count()
    );
}
