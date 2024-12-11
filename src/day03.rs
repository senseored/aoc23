use std::fs;
pub fn main(file_path: &str) {
    let file_path = "input/test03.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");
    println!("{:?}", populate(&contents));
}
fn populate(contents: &str) {
    let mut symbols: Vec<(i32, i32)> = Vec::new();

    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("{:?}", map);

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c != '.' && !c.is_numeric() {
                symbols.push((x as i32, y as i32));
            }
        });
    });
    // println!("{:?}", symbols);

    // contents.split('.').for_each(|line| {
    //     if !line.is_empty() {
    //         line.split_whitespace().for_each(|s| println!("{s}"));
    //     }
    // });

    let mut result = 0;
    symbols.iter().for_each(|(x, y)| {
        for (dx, dy) in &[
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ] {
            let nx = x + dx;
            let ny = y + dy;
            let n = map[nx as usize][ny as usize];
            if n.is_numeric() {
                let (mut left, mut right) = (nx - 1, ny + 1);
                let mut m = n;
                let mut number: String = n.to_string();
                m = map[nx as usize][left as usize];
                while m.is_numeric() && left >= 0 {
                    m = map[nx as usize][left as usize];
                    number = format!("{}{}", m, number);
                    left -= 1;
                }
                let mut m = n;
                m = map[nx as usize][right as usize];
                while m.is_numeric() && right < map.len() as i32 {
                    m = map[nx as usize][right as usize];
                    number = format!("{}{}", number, m);
                    right += 1;
                }
                println!("number: {}", number);
                // if n.is_numeric() {
                //
                //     // result += n.to_digit(10).unwrap() as i32;
                // }
            }
        }
    });
    println!("{result}");
    // println!("{:?}", numbers);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let file_path = "input/test01.txt";

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let test = populate(&contents);

        assert_eq!(test, (142, 0));
    }
}
