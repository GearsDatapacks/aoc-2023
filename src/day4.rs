pub fn main() {
  let input =  std::fs::read_to_string("day4_inputs.txt").unwrap();
  let mut sum = 0;
  for line in input.lines() {
    let numbers = line.split(":").nth(1).unwrap().trim();
    let winning_numbers: Vec<u32> = numbers.split(" | ")
                                    .nth(0).unwrap().split_ascii_whitespace()
                                    .map(str::parse::<u32>).filter_map(Result::ok).collect();
    let our_numbers: Vec<u32> = numbers.split(" | ")
                                .nth(1).unwrap().split_ascii_whitespace()
                                .map(str::parse::<u32>).filter_map(Result::ok).collect();

    let mut num_winning: u32 = 0;
    for num in winning_numbers {
      if our_numbers.contains(&num) {
        num_winning += 1;
      }
    }

    if num_winning > 0 {
      sum += 2_u32.pow(num_winning-1);
    }
  }

  println!("{}", sum);
}