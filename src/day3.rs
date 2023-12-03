pub fn main() {
  let input = std::fs::read_to_string("day3_inputs.txt").unwrap();
  let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut sum = 0;

  for (line_number, line) in matrix.iter().enumerate() {
    let mut i = 0;
    while i < line.len() {
      let ch = line[i];
      if !ch.is_digit(10) {
        i += 1;
        continue;
      }

      let start_index = if i == 0 {0} else {i-1};
      let mut num_str = "".to_string();
      while i < line.len() && line[i].is_digit(10) {
        num_str.push(line[i]);
        i += 1;
      }

      let end_index = if i == matrix.len() {i - 1} else {i};
      let num: i32 = num_str.parse().unwrap();

      let prev_line = if line_number == 0 {0} else {line_number - 1};
      let next_line = if line_number == matrix.len() - 1 {line_number} else {line_number + 1};

      let mut part_number = false;
      for x in start_index..=end_index {
        for y in prev_line..=next_line {
          if is_symbol(matrix[y][x]) {
            part_number = true;
            break;
          }
        }
        if part_number {
          break;
        }
      }

      if part_number {
        sum += num;
      }
    }
  }

  println!("{}", sum);
}

fn is_symbol(ch: char) -> bool {
  return !ch.is_digit(10) && ch != '.';
}

//123...
//..*...
//......
