#![allow(clippy::needless_range_loop)]

use std::collections::VecDeque;

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let mut h = s_l.next().unwrap().parse::<usize>().unwrap();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();
  let m = s_l.next().unwrap().parse::<usize>().unwrap();
  if h == 0 {
    println!("0");
    return;
  }

  // 空かどうかのマップ
  let mut map: Vec<Vec<bool>> = vec![vec![false; m]; n];
  let mut checked_map: Vec<Vec<bool>> = vec![vec![false; m]; n];

  for i in 0..n {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s_l = s.split_whitespace();
    for (j, s) in s_l.enumerate() {
      if s == "." {
        map[i][j] = true;
      }
    }
  }

  let mut ans_lst = Vec::new();

  for i in 0..n {
    for j in 0..m {
      if map[i][j] && !checked_map[i][j] {
        // まだ訪問されていない
        let mut count = 1;
        let mut queue = VecDeque::new();
        queue.push_back((i, j));
        checked_map[i][j] = true;
        while let Some((i, j)) = queue.pop_front() {
          let lst = check(n, m, i, j, &map, &mut checked_map);
          for (i2, j2) in lst.iter() {
            queue.push_back((*i2, *j2));
            count += 1;
          }
        }
        ans_lst.push(count);
      }
    }
  }

  ans_lst.sort_by(|a, b| b.cmp(a));

  let mut ans = 0_usize;
  for i in ans_lst.iter() {
    if h <= *i {
      ans += 1;
      break;
    } else {
      h -= *i;
      ans += 1;
    }
  }
  println!("{ans}");
}

fn check(
  n: usize,
  m: usize,
  i: usize,
  j: usize,
  map: &[Vec<bool>],
  checked_map: &mut [Vec<bool>],
) -> Vec<(usize, usize)> {
  let n = n as isize;
  let m = m as isize;
  let i = i as isize;
  let j = j as isize;
  let even_diff = [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, -1), (1, 0)];
  let odd_diff = [(-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0), (1, 1)];
  let mut v = Vec::new();
  if i % 2 == 0 {
    // 偶数番目列
    for (plus_i, plus_j) in even_diff.iter() {
      let i2 = i + plus_i;
      let j2 = j + plus_j;
      if i2 >= 0 && i2 < n && j2 >= 0 && j2 < m {
        let i2 = i2 as usize;
        let j2 = j2 as usize;
        if map[i2][j2] && !checked_map[i2][j2] {
          checked_map[i2][j2] = true;
          v.push((i2, j2));
        }
      }
    }
  } else {
    // 奇数番目列
    for (plus_i, plus_j) in odd_diff.iter() {
      let i2 = i + plus_i;
      let j2 = j + plus_j;
      if i2 >= 0 && i2 < n && j2 >= 0 && j2 < m {
        let i2 = i2 as usize;
        let j2 = j2 as usize;
        if map[i2][j2] && !checked_map[i2][j2] {
          checked_map[i2][j2] = true;
          v.push((i2, j2));
        }
      }
    }
  }
  v
}
