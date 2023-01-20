#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Default)]
struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinates {
    fn find_empty_sides_num(&self, coordinates: &std::collections::HashSet<Coordinates>) -> i32 {
        let mut neighbors: Vec<Coordinates> = Vec::new();
        for distance in [-1, 1] {
            neighbors.push(Coordinates {
                x: (self.x + distance),
                y: (self.y),
                z: (self.z),
            });
            neighbors.push(Coordinates {
                x: (self.x),
                y: (self.y + distance),
                z: (self.z),
            });
            neighbors.push(Coordinates {
                x: (self.x),
                y: (self.y),
                z: (self.z + distance),
            });
        }
        return neighbors
            .iter()
            .filter(|neighbor| !coordinates.contains(neighbor))
            .count()
            .try_into()
            .unwrap();
    }
}

fn parse_coordinates(line: &str) -> Coordinates {
    let mut coords = line.split(",").map(|num| num.parse().unwrap());
    return Coordinates {
        x: (coords.next().unwrap()),
        y: (coords.next().unwrap()),
        z: (coords.next().unwrap()),
    };
}

fn main() {
    let coords: std::collections::HashSet<Coordinates> = include_str!("../input.txt")
        .split("\n")
        .map(|content| content.lines().map(|line| parse_coordinates(line)))
        .flatten()
        .collect();
    let surface: i32 = coords
        .iter()
        .map(|pair| pair.find_empty_sides_num(&coords))
        .sum();
    println!("{}", surface);
}
