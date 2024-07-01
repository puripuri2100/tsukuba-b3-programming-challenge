use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
  x: f64,
  y: f64,
}

impl Point {
  // 人が走るときの経過秒
  fn run(&self, other: &Self) -> f64 {
    let x = self.x - other.x;
    let y = self.y - other.y;
    let len = (x * x + y * y).sqrt();
    len / 5.0
  }

  // 打って走るとき
  // 距離が30mより短いときは走ったほうが早い
  fn launch_and_run(&self, other: &Self) -> f64 {
    let x = self.x - other.x;
    let y = self.y - other.y;
    let len = (x * x + y * y).sqrt();
    if len < 30.0 {
      len / 5.0
    } else if len > 50.0 {
      2.0 + (len - 50.0) / 5.0
    } else {
      2.0 + (50.0 - len) / 5.0
    }
  }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for MinNonNan {
  fn cmp(&self, other: &MinNonNan) -> Ordering {
    self.get().partial_cmp(&other.get()).unwrap()
  }
}

impl MinNonNan {
  fn add(self, other: f64) -> Self {
    MinNonNan(self.0 + other)
  }
  fn get(self) -> f64 {
    self.0
  }
}

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let start_x = s_l.next().unwrap().parse::<f64>().unwrap();
  let start_y = s_l.next().unwrap().parse::<f64>().unwrap();
  s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let end_x = s_l.next().unwrap().parse::<f64>().unwrap();
  let end_y = s_l.next().unwrap().parse::<f64>().unwrap();
  s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let n = s.trim().parse::<usize>().unwrap();
  let mut point = Vec::new();
  point.push(Point {
    x: start_x,
    y: start_y,
  });
  for _ in 0..n {
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let x = s_l.next().unwrap().parse::<f64>().unwrap();
    let y = s_l.next().unwrap().parse::<f64>().unwrap();
    point.push(Point { x, y });
  }
  point.push(Point { x: end_x, y: end_y });

  let mut graph = vec![vec![0.0; n + 2]; n + 2];
  for i in 0..n + 1 {
    for j in i..n + 2 {
      if i == 0 {
        // スタート地点なので走る
        let start = point[i];
        let end = point[j];
        let s = start.run(&end);
        graph[i][j] = s;
        graph[j][i] = s;
      } else {
        let start = point[i];
        let end = point[j];
        let s = start.launch_and_run(&end);
        graph[i][j] = s;
        graph[j][i] = s;
      }
    }
  }

  let ans = dijkstra(n + 2, &graph);
  println!("{ans}");
}

// ダイクストラ法の実装
fn dijkstra(n: usize, graph: &[Vec<f64>]) -> f64 {
  let mut weight_map = vec![f64::MAX; n];
  weight_map[0] = 0.0;
  let mut bh = BinaryHeap::new();
  bh.push((Reverse(MinNonNan(0.0)), 0));

  // Reverseを使っているので最小値から手に入る
  while let Some((Reverse(w), from)) = bh.pop() {
    if from == n - 1 {
      // ゴールへの到達
      return w.get();
    }

    for (to, weight) in graph[from].iter().enumerate() {
      if weight_map[to] > w.get() + (weight) {
        // 小さいものが見つかったので上書き
        weight_map[to] = w.get() + weight;
        bh.push((Reverse(w.add(*weight)), to));
      }
    }
  }
  f64::MAX
}
