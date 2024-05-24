#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapType {
  Gold,
  Start,
  Safe,
  Trap,
  Wall,
}

pub fn main() {
  use MapType::*;
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let w = s_l.next().unwrap().parse::<usize>().unwrap();
  let h = s_l.next().unwrap().parse::<usize>().unwrap();

  let mut start_pos: (usize, usize) = (0, 0);
  // マスの種類と、トラップに隣接しているかどうか
  let mut map = vec![vec![Safe; w]; h];
  let mut near_trap_map = vec![vec![true; w]; h];
  for i in 0..h {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    for (j, c) in s.trim().chars().enumerate() {
      if c == 'P' {
        start_pos = (j, i);
        map[i][j] = Start;
      } else if c == 'G' {
        map[i][j] = Gold;
      } else if c == 'T' {
        near_trap_map[i][j] = false;
        near_trap_map[i - 1][j] = false;
        near_trap_map[i + 1][j] = false;
        near_trap_map[i][j - 1] = false;
        near_trap_map[i][j + 1] = false;
        map[i][j] = Trap;
      } else if c == '#' {
        map[i][j] = Wall;
      } else {
        map[i][j] = Safe;
      }
    }
  }

  let mut visits = vec![vec![false; w]; h];
  visits[start_pos.1][start_pos.0] = true;
  let mut golds = vec![];
  let mut queue = std::collections::VecDeque::new();
  if near_trap_map[start_pos.1][start_pos.0] {
    queue.push_back(start_pos);
  }
  while let Some((head_x, head_y)) = queue.pop_front() {
    let x_1 = head_x - 1;
    let y_1 = head_y;
    let next_ok = f(x_1, y_1, &mut visits, &mut golds, &map);
    let not_near_trap = near_trap_map[y_1][x_1];
    if next_ok && not_near_trap {
      queue.push_back((x_1, y_1));
    }

    let x_1 = head_x + 1;
    let y_1 = head_y;
    let next_ok = f(x_1, y_1, &mut visits, &mut golds, &map);
    let not_near_trap = near_trap_map[y_1][x_1];
    if not_near_trap && next_ok {
      queue.push_back((x_1, y_1));
    }

    let x_1 = head_x;
    let y_1 = head_y - 1;
    let next_ok = f(x_1, y_1, &mut visits, &mut golds, &map);
    let not_near_trap = near_trap_map[y_1][x_1];
    if not_near_trap && next_ok {
      queue.push_back((x_1, y_1));
    }

    let x_1 = head_x;
    let y_1 = head_y + 1;
    let next_ok = f(x_1, y_1, &mut visits, &mut golds, &map);
    let not_near_trap = near_trap_map[y_1][x_1];
    if not_near_trap && next_ok {
      queue.push_back((x_1, y_1));
    }
  }
  println!("{}", golds.len());
}

// 次も探索するかどうかと隣接がトラップかどうかを判定する
fn f(
  x: usize,
  y: usize,
  visits: &mut [Vec<bool>],
  golds: &mut Vec<(usize, usize)>,
  map: &[Vec<MapType>],
) -> bool {
  use MapType::*;
  if visits[y][x] {
    // 探索済み
    false
  } else {
    // 未探索
    let map_type = map[y][x];
    if map_type == Gold {
      golds.push((x, y));
    }
    visits[y][x] = true;

    !(map_type == Wall || map_type == Trap)
  }
}
