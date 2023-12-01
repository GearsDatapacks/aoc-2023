use std::fs;

pub fn main() {
  let input = fs::read_to_string("day1_inputs.txt").unwrap();
  let mut sum = 0;

  for line in input.lines() {
    let mut digits: Vec<u32> = Vec::new();
    let mut index = 0;
    while index < line.len() {
      if let Some(d) = line.chars().nth(index).and_then(|c| c.to_digit(10)) {
        digits.push(d);
      }

      else if let Some(d) = parse_digit(&line[index..]) {
        digits.push(d);
      }

      index += 1;
    }

    let number = digits[0] * 10 + digits[digits.len() - 1];
    sum += number;
  }

  println!("{}", sum);
}

fn parse_digit(input: &str) -> Option<u32> {
  let digits = ["one","two","three","four","five","six","seven","eight","nine"];
  for i in 0..digits.len() {
    if input.starts_with(digits[i]) {
      return Some((i + 1) as u32);
    }
  }

  return None;
}
