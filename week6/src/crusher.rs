use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Data {
  value: Option<usize>,
  x: usize,
  y: usize,
  route: Vec<(usize, usize)>,
}

fn check(
  data: &Data,
  new_x: usize,
  new_y: usize,
  weight_map: &[Vec<Data>],
  lst: &[Vec<usize>],
) -> Option<Data> {
  let new_data = &weight_map[new_y][new_x];
  if let Some(v) = new_data.value {
    let new_v = data.value.unwrap() + lst[new_y][new_x];
    if new_v < v {
      // 小さい値だったので更新
      let mut route = data.route.clone();
      route.push((new_x, new_y));
      let new_data = Data {
        x: new_x,
        y: new_y,
        route,
        value: Some(data.value.unwrap() + lst[new_y][new_x]),
      };
      Some(new_data)
    } else {
      None
    }
  } else {
    // 新規なので絶対に更新
    let mut route = data.route.clone();
    route.push((new_x, new_y));
    let new_data = Data {
      x: new_x,
      y: new_y,
      route,
      value: Some(data.value.unwrap() + lst[new_y][new_x]),
    };
    Some(new_data)
  }
}

pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let m = s_l.next().unwrap().parse::<usize>().unwrap();
    let n = s_l.next().unwrap().parse::<usize>().unwrap();
    if m == 0 || n == 0 {
      break;
    }

    let mut lst = Vec::new();
    let mut weight_map = vec![
      vec![
        Data {
          x: 0,
          y: 0,
          value: None,
          route: Vec::new()
        };
        n
      ];
      m
    ];
    let mut queue = VecDeque::new();
    for i in 0..m {
      s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      let mut l = Vec::new();
      for c in s.trim().chars() {
        let v = c.to_string().parse::<usize>().unwrap();
        l.push(v);
      }
      if i == 0 {
        for (j, n) in l.iter().enumerate() {
          let data = Data {
            value: Some(*n),
            route: vec![(j, 0)],
            x: j,
            y: 0,
          };
          weight_map[0][j] = data;
          queue.push_back((j, 0));
        }
      }
      lst.push(l);
    }

    while let Some((data_x, data_y)) = queue.pop_front() {
      let data = weight_map[data_y][data_x].clone();
      if data_x != 0 {
        let new_x = data_x - 1;
        // 左
        if let Some(new_data) = check(&data, new_x, data.y, &weight_map, &lst) {
          queue.push_back((new_data.x, new_data.y));
          weight_map[data.y][new_x] = new_data;
        }
        if data_y != m - 1 {
          // 左下
          let new_y = data_y + 1;
          if let Some(new_data) = check(&data, new_x, new_y, &weight_map, &lst) {
            queue.push_back((new_data.x, new_data.y));
            weight_map[new_y][new_x] = new_data;
          }
        }
        if data_y != 0 {
          // 左上
          let new_y = data_y - 1;
          if let Some(new_data) = check(&data, new_x, new_y, &weight_map, &lst) {
            queue.push_back((new_data.x, new_data.y));
            weight_map[new_y][new_x] = new_data;
          }
        }
      }

      if data_x != n - 1 {
        let new_x = data_x + 1;
        if let Some(new_data) = check(&data, new_x, data.y, &weight_map, &lst) {
          // 右
          queue.push_back((new_data.x, new_data.y));
          weight_map[data.y][new_x] = new_data;
        }
        if data_y != m - 1 {
          // 右下
          let new_y = data_y + 1;
          if let Some(new_data) = check(&data, new_x, new_y, &weight_map, &lst) {
            queue.push_back((new_data.x, new_data.y));
            weight_map[new_y][new_x] = new_data;
          }
        }
        if data_y != 0 {
          // 右上
          let new_y = data_y - 1;
          if let Some(new_data) = check(&data, new_x, new_y, &weight_map, &lst) {
            queue.push_back((new_data.x, new_data.y));
            weight_map[new_y][new_x] = new_data;
          }
        }
      }

      // 上
      if data_y != 0 {
        let new_y = data_y - 1;
        if let Some(new_data) = check(&data, data.x, new_y, &weight_map, &lst) {
          queue.push_back((new_data.x, new_data.y));
          weight_map[new_y][data.x] = new_data;
        }
      }

      // 下
      if data_y != m - 1 {
        let new_y = data_y + 1;
        if let Some(new_data) = check(&data, data.x, new_y, &weight_map, &lst) {
          queue.push_back((new_data.x, new_data.y));
          weight_map[new_y][data.x] = new_data;
        }
      }
    }

    let route = &weight_map[m - 1]
      .iter()
      .min_by_key(|d| d.value.unwrap())
      .unwrap()
      .route;
    let mut map = lst
      .iter()
      .map(|l| l.iter().map(|n| (*n, true)).collect())
      .collect::<Vec<Vec<_>>>();
    for (i, j) in route.iter() {
      map[*j][*i].1 = false;
    }

    for l in map.iter() {
      for (n, b) in l.iter() {
        if *b {
          print!("{n}");
        } else {
          print!(" ");
        }
      }
      println!();
    }
  }
}
