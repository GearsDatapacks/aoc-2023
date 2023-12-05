struct Map {
  source_range_start: u64,
  destination_range_start: u64,
  range_length: u64,
}

pub fn main() {
  let input = std::fs::read_to_string("day5_inputs.txt").unwrap();
  let mut seeds: Vec<u64> = Vec::new();
  let mut current_maps: Vec<Map> = Vec::new();

  for (line_number, line) in input.lines().enumerate() {
    if line_number == 0 {
      seeds = line.split(": ").nth(1).unwrap().split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    }
    if line.is_empty() {
      seeds = seeds.iter().map(|&seed| process(seed, &current_maps)).collect();
      current_maps.clear();
      continue;
    }
    if !line.chars().nth(0).unwrap().is_digit(10) {
      continue;
    }
    let mut nums: Vec<u64> = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let map = Map {
      destination_range_start: nums[0],
      source_range_start: nums[1],
      range_length: nums[2],
    };
    current_maps.push(map);
  }

  seeds = seeds.iter().map(|&seed| process(seed, &current_maps)).collect();
    current_maps.clear();

  println!("{}", seeds.iter().min().unwrap());
}

fn process(seed: u64, maps: &[Map]) -> u64 {
  for map in maps {
    if (map.source_range_start..(map.source_range_start+map.range_length)).contains(&seed) {
      return map.destination_range_start + seed - map.source_range_start;
    }
  }
  return seed;
}
