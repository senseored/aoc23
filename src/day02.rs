use std::fs;
pub fn main(file_path: &str) {
    let file_path = "input/test02.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");

    f_p(&contents);
}

fn f_p(contents: &str) {
    contents.split(": ").for_each(|line| {
        line.split("; ")
            .for_each(|l| l.split(", ").for_each(|n| println!("{n}")))
    });
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
