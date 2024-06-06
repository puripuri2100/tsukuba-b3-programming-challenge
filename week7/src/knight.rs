pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let n = s.trim().parse::<usize>().unwrap();
  if n <= 3 {
    println!("NO");
    return;
  }
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let clst = s.trim().chars();
  let mut map = vec![vec![' '; n]; n];
  let mut queue = std::collections::VecDeque::new();
  for (i, c) in clst.enumerate() {
    let x = i % n;
    let y = i / n;
    map[y][x] = c;
    if c == 'I' {
      queue.push_back((x, y, 0));
    }
  }

  let text = ['I', 'C', 'P', 'C', 'A', 'S', 'I', 'A', 'S', 'G'];
  let mut is_ok = false;
  while let Some((x, y, text_n)) = queue.pop_front() {
    if text_n == 9 {
      break;
    }
    //xCxCx
    //CxxxC
    //xxIxx
    //CxxxC
    //xCxCx
    if x >= 2 {
      if y >= 1 && map[y - 1][x - 2] == text[text_n + 1] {
        if text_n == 8 {
          is_ok = true;
          break;
        } else {
          queue.push_back((x - 2, y - 1, text_n + 1));
        }
      }
      if y <= n - 2 && map[y + 1][x - 2] == text[text_n + 1] {
        if text_n == 8 {
          is_ok = true;
          break;
        } else {
          queue.push_back((x - 2, y + 1, text_n + 1));
        }
      }
    }
    if x >= 1 {
      if y >= 2 && map[y - 2][x - 1] == text[text_n + 1] {
        if text_n == 8 {
          is_ok = true;
          break;
        } else {
          queue.push_back((x - 1, y - 2, text_n + 1));
        }
      }
      if y <= n - 3 && map[y + 2][x - 1] == text[text_n + 1] {
        if text_n == 8 {
          is_ok = true;
          break;
        } else {
          queue.push_back((x - 1, y + 2, text_n + 1));
        }
      }
    }
    if n - 2 >= x {
      if y >= 2 && map[y - 2][x + 1] == text[text_n + 1] {
        if text_n == 8 {
          is_ok = true;
          break;
        } else {
          queue.push_back((x + 1, y - 2, text_n + 1));
        }
      }
      if y <= n - 3 && map[y + 2][x + 1] == text[text_n + 1] {
        if text_n == 8 {
          is_ok = true;
          break;
        } else {
          queue.push_back((x + 1, y + 2, text_n + 1));
        }
      }
    }
    if n - 3 >= x {
      if y >= 1 && map[y - 1][x + 2] == text[text_n + 1] {
        if text_n == 8 {
          is_ok = true;
          break;
        } else {
          queue.push_back((x + 2, y - 1, text_n + 1));
        }
      }
      if y <= n - 2 && map[y + 1][x + 2] == text[text_n + 1] {
        if text_n == 8 {
          is_ok = true;
          break;
        } else {
          queue.push_back((x + 2, y + 1, text_n + 1));
        }
      }
    }
  }
  if is_ok {
    println!("YES")
  } else {
    println!("NO")
  }
}
