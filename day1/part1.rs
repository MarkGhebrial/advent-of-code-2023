use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;

    for line in input.lines() {
        //print!("{}", line);

        let mut s = String::new();

        for c in line.chars() {
            if c.is_numeric() {
                s.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                s.push(c);
                break;
            }
        }

        sum += s.parse::<usize>().unwrap();

        println!("{}", s.parse::<usize>().unwrap());
    }

    println!("Sum: {}", sum);
}