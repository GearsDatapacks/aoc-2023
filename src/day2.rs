#[derive(Debug)]
struct Game {
  id: i32,
  revealed: Vec<(i32, i32, i32)>
}

pub fn main() {
  let input = std::fs::read_to_string("day2_inputs.txt").unwrap();
  let mut games: Vec<Game> = Vec::new();
  for line in input.lines() {
    games.push(parse_game(line));
  }

  let mut sum = 0;

  let max_counts = (12,13,14);
  for game in games {
    let mut plausible = true;
    for revealed in game.revealed {
      // println!("{:?} >= {:?}: {}", revealed, max_counts, !less_than(revealed, max_counts));
      if !less_than(revealed, max_counts) {
        plausible = false;
        break;
      }
    }
    if plausible {
      sum += game.id;
    }
  }

  println!("{}", sum);
}

fn less_than(a: (i32, i32, i32), b: (i32, i32, i32)) -> bool {
  return a.0 <= b.0 && a.1 <= b.1 && a.2 <= b.2;
}

fn parse_game(input: &str) -> Game {
  // let re = Regex::new(r"Game (\d+): (((\d+) green, )?((\d+) red, )?((\d+) blue)?(; )?)+").unwrap();
  let game_str = input.split(":").nth(0).unwrap();
  let pulls_str = input.split(":").nth(1).unwrap();
  let pulls = pulls_str.split(";");
  let game_id: i32 = game_str.split(" ").nth(1).unwrap().parse().unwrap();

  let mut game = Game {
    id: game_id,
    revealed: Vec::new(),
  };

  for pull in pulls {
    let cubes = pull.split(", ");
    let mut revealed = (0, 0, 0);
    for cube in cubes.map(|x| x.trim()) {
      let count: i32 = cube.split(" ").nth(0).unwrap().parse().unwrap();
      let colour = cube.split(" ").nth(1).unwrap();
      match colour {
        "red" => revealed.0 = count,
        "green" => revealed.1 = count,
        "blue" => revealed.2 = count,
        _ => {}
      };
    }
    game.revealed.push(revealed);
  }

  return game;
}