use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test01.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");
    println!("{:?}", populate(&contents));
}
fn populate(contents: &str) -> (i32, i32) {
    let mut nums: Vec<i32> = Vec::new();
    contents.lines().for_each(|line| {
        let mut num: Vec<i32> = Vec::new();
        line.chars().for_each(|c| {
            if c.is_digit(10) {
                num.push(c.to_digit(10).unwrap() as i32);
            }
        });
        nums.push(num.first().unwrap() * 10 + num.last().unwrap());
    });
    let mut nums_letters: Vec<i32> = Vec::new();
    contents.lines().for_each(|line| {
        let line = &line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "thr3e")
            .replace("four", "f4ur")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "n9ne");
        let mut num: Vec<i32> = Vec::new();
        line.chars().for_each(|c| {
            if c.is_digit(10) {
                num.push(c.to_digit(10).unwrap() as i32);
            }
        });
        nums_letters.push(num.first().unwrap() * 10 + num.last().unwrap());
    });
    (nums.iter().sum(), nums_letters.iter().sum())
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
