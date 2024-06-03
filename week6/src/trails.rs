pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let p = s_l.next().unwrap().parse::<usize>().unwrap() - 1;
  let t = s_l.next().unwrap().parse::<usize>().unwrap();
  let mut map: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; p + 1]; p + 1];
  for _ in 0..t {
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let from = s_l.next().unwrap().parse::<usize>().unwrap();
    let to = s_l.next().unwrap().parse::<usize>().unwrap();
    let weight = s_l.next().unwrap().parse::<usize>().unwrap();
    if let Some((w, n)) = map[from][to] {
      #[allow(clippy::comparison_chain)]
      if w == weight {
        map[from][to] = Some((w, n + 1));
        map[to][from] = Some((w, n + 1));
      } else if w > weight {
        map[from][to] = Some((weight, 1));
        map[to][from] = Some((weight, 1));
      }
    } else {
      map[from][to] = Some((weight, 1));
      map[to][from] = Some((weight, 1));
    }
  }

  // ダイクストラ法の改変
  let mut weight_map = vec![(usize::MAX, 0); p + 1];
  weight_map[0] = (0, 0);
  let mut queue = vec![None; p + 1];
  queue[0] = Some(0_usize);
  while let Some((from, w)) = queue
    .iter()
    .enumerate()
    .filter(|(_, w)| w.is_some())
    .map(|(i, w)| (i, w.unwrap()))
    .min_by_key(|(_, w)| *w)
  {
    queue[from] = None;
    for (to, weight_info) in map[from].iter().enumerate() {
      if let Some((weight, n)) = weight_info {
        #[allow(clippy::comparison_chain)]
        if weight_map[to].0 > w + weight {
          // より小さなルートが見つかったので経路は上書き
          weight_map[to].0 = w + weight;
          weight_map[to].1 = weight_map[from].1 + weight * n;
          // 更新されたマップなので追加
          queue[to] = Some(w + weight);
        } else if weight_map[to].0 == w + weight {
          // 経路は追加
          weight_map[to].1 += weight_map[from].1 + weight * n;
        }
      }
    }
  }

  let ans = &weight_map[p].1;
  println!("{}", ans * 2);
}
