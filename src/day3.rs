pub fn main() {
  let input = std::fs::read_to_string("day3_inputs.txt").unwrap();
  let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut sum = 0;

  for (line_number, line) in matrix.iter().enumerate() {
    for (line_index, &ch) in line.iter().enumerate() {
      if ch != '*' {
        continue;
      }

      let mut adjacent_nums: Vec<i32> = Vec::new();
      let start_line = if line_number == 0 {0} else {line_number - 1};
      let end_line = (matrix.len()-1).min(line_number+1);

      let start_index = if line_index == 0 {0} else {line_index - 1};
      let end_index = (line.len()-1).min(line_index + 1);

      let mut y = start_line;
      while y <= end_line {
        let mut x = start_index;
        while x <= end_index {
          if let Some(x) = parse_int(&matrix, &mut x, y) {
            adjacent_nums.push(x)
          }
          x += 1;
        }
        y += 1;
      }

      if adjacent_nums.len() == 2 {
        sum += adjacent_nums[0] * adjacent_nums[1];
      }
    }
  }

  println!("{}", sum);
}

fn parse_int(matrix: &[Vec<char>], x: &mut usize, y: usize) -> Option<i32> {
    if !matrix[y][*x].is_digit(10) {
      return None;
    }

    let mut num_str = "".to_string();
    while matrix[y][*x - 1].is_digit(10) {
      *x -= 1;
      if *x == 0 {
        break;
      }
    }

    while *x < matrix.len() && matrix[y][*x].is_digit(10) {
      num_str.push(matrix[y][*x]);
      if *x < matrix.len() - 1 {
        *x += 1;
      } else {
        break;
      }
    }

    return num_str.parse().ok();
}

//123...
//..*...
//......
