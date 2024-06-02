use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Data {
  x: usize,
  y: usize,
  value: usize,
  n: usize,
}

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let m = s_l.next().unwrap().parse::<usize>().unwrap();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();

  let mut lst = Vec::new();
  for _ in 0..m {
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut l = Vec::new();
    for c in s.trim().chars() {
      let v = c.to_string().parse::<usize>().unwrap();
      l.push(v);
    }
    lst.push(l);
  }

  let mut cheked = vec![vec![false; n]; m];
  cheked[0][0] = true;
  let mut queue = VecDeque::new();
  queue.push_back(Data {
    x: 0,
    y: 0,
    value: lst[0][0],
    n: 0,
  });

  let mut ans = None;

  while let Some(data) = queue.pop_front() {
    if data.x + data.value < n && !cheked[data.y][data.x + data.value] {
      if data.x + data.value == n - 1 && data.y == m - 1 {
        ans = Some(data.n + 1);
        break;
      }
      let new_value = lst[data.y][data.x + data.value];
      cheked[data.y][data.x + data.value] = true;
      queue.push_back(Data {
        x: data.x + data.value,
        y: data.y,
        value: new_value,
        n: data.n + 1,
      });
    }

    if data.y + data.value < m && !cheked[data.y + data.value][data.x] {
      if data.x == n - 1 && data.y + data.value == m - 1 {
        ans = Some(data.n + 1);
        break;
      }
      let new_value = lst[data.y + data.value][data.x];
      cheked[data.y + data.value][data.x] = true;
      queue.push_back(Data {
        x: data.x,
        y: data.y + data.value,
        value: new_value,
        n: data.n + 1,
      });
    }

    if data.x >= data.value && !cheked[data.y][data.x - data.value] {
      let new_value = lst[data.y][data.x - data.value];
      cheked[data.y][data.x - data.value] = true;
      queue.push_back(Data {
        x: data.x - data.value,
        y: data.y,
        value: new_value,
        n: data.n + 1,
      });
    }

    if data.y >= data.value && !cheked[data.y - data.value][data.x] {
      let new_value = lst[data.y - data.value][data.x];
      cheked[data.y - data.value][data.x] = true;
      queue.push_back(Data {
        x: data.x,
        y: data.y - data.value,
        value: new_value,
        n: data.n + 1,
      });
    }
  }

  if let Some(ans) = ans {
    println!("{ans}");
  } else {
    println!("-1");
  }
}
