use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
  to: usize,
  value: usize,
}

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();
  let m = s_l.next().unwrap().parse::<usize>().unwrap();
  let x = s_l.next().unwrap().parse::<usize>().unwrap();
  let mut graph = vec![Vec::new(); n];
  for _ in 0..m {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let start = s_l.next().unwrap().parse::<usize>().unwrap();
    let end = s_l.next().unwrap().parse::<usize>().unwrap();
    let value = s_l.next().unwrap().parse::<usize>().unwrap();
    graph[start - 1].push(Edge { to: end - 1, value });
    graph[end - 1].push(Edge {
      to: start - 1,
      value,
    });
  }

  let target = dijkstra(n, &graph, usize::MAX);
  let target_sum = (target.0 * (100 + x)) as f64 / 100.0;
  // 2分探索をするときの範囲
  let mut left = 0;
  let mut right = target.1;

  while left + 1 < right {
    let mid = (left + right) / 2;
    let v = dijkstra(n, &graph, mid);
    if v.0 as f64 > target_sum {
      left = mid;
    } else {
      right = mid;
    }
  }

  // 最後の確認
  let v = dijkstra(n, &graph, left);
  if v.0 as f64 > target_sum {
    println!("{right}")
  } else {
    println!("{left}")
  }
}

// ダイクストラ法の実装
// 引数：グラフのサイズ・グラフ・許されるコストの最大値
fn dijkstra(n: usize, graph: &[Vec<Edge>], max_cost: usize) -> (usize, usize) {
  let mut weight_map = vec![usize::MAX; n];
  weight_map[0] = 0;
  let mut bh = BinaryHeap::new();
  bh.push((Reverse(0), (0, 0)));

  // Reverseを使っているので最小値から手に入る
  while let Some((Reverse(w), (from, max))) = bh.pop() {
    if from == n - 1 {
      // ゴールへの到達
      return (w, max);
    }

    for edge in graph[from].iter() {
      let to = edge.to;
      let weight = edge.value;
      if edge.value <= max_cost && weight_map[to] > w + weight {
        // 許容されるルートでより小さいものが見つかったので上書き
        weight_map[to] = w + weight;
        bh.push((Reverse(w + weight), (to, edge.value.max(max))));
      }
    }
  }
  (usize::MAX, usize::MAX)
}
