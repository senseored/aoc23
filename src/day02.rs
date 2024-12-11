use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test02.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");

    f_p(&contents);
}

fn f_p(contents: &str) {
    let mut score = 0;
    let mut part2 = 0;
    contents.lines().for_each(|line| {
        let (game, data) = line.trim_start_matches("Game ").split_once(":").unwrap();
        // println!("{},{}", game, data);
        let mut max = (0, 0, 0);
        data.trim().split([';', ',']).for_each(|s| {
            let (n, color) = s.trim().split_once(' ').unwrap();
            // println!("{}, {}", n, color);
            match color {
                "red" => {
                    if max.0 < n.parse::<i32>().unwrap() {
                        max.0 = n.parse::<i32>().unwrap();
                    }
                }
                "green" => {
                    if max.1 < n.parse::<i32>().unwrap() {
                        max.1 = n.parse::<i32>().unwrap();
                    }
                }
                "blue" => {
                    if max.2 < n.parse::<i32>().unwrap() {
                        max.2 = n.parse::<i32>().unwrap();
                    }
                }
                _ => panic!(),
            }
        });
        if max.0 <= 12 && max.1 <= 13 && max.2 <= 14 {
            println!("WINNER {}", game);
            score += game.parse::<i32>().unwrap();
        }
        part2 += max.0 * max.1 * max.2 as i32;
    });
    println!("TOTAL: {}\nPART2: {}", score, part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let file_path = "input/test00.txt";

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        println!("{contents}");

        assert_eq!(0, 0);
    }
}
