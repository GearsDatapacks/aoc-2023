pub fn main() {
  let input =  std::fs::read_to_string("day4_inputs.txt").unwrap();
  let mut card_counts: Vec<u32> = vec![1; input.lines().count()];
  for (line_number, line) in input.lines().into_iter().enumerate() {
    let wins = process_scratchcard(line);
    for i in 1..=wins {
      card_counts[line_number + i as usize] += card_counts[line_number];
    }
  }

  println!("{}", card_counts.iter().sum::<u32>());
}

fn process_scratchcard(card: &str) -> u32 {
  let numbers = card.split(":").nth(1).unwrap().trim();
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

  return num_winning;
}
