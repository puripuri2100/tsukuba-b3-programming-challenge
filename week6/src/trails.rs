use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let p = s_l.next().unwrap().parse::<usize>().unwrap() - 1;
  let t = s_l.next().unwrap().parse::<usize>().unwrap();
  let mut map: Vec<std::collections::HashMap<usize, (usize, usize)>> =
    vec![std::collections::HashMap::new(); p + 1];
  for _ in 0..t {
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let from = s_l.next().unwrap().parse::<usize>().unwrap();
    let to = s_l.next().unwrap().parse::<usize>().unwrap();
    let weight = s_l.next().unwrap().parse::<usize>().unwrap();
    let hashmap = &map[from];
    if from != to {
      if let Some((w, n)) = hashmap.clone().get(&to) {
        #[allow(clippy::comparison_chain)]
        if *w == weight {
          map[from].insert(to, (*w, n + 1));
          map[to].insert(from, (*w, n + 1));
        } else if *w > weight {
          map[from].insert(to, (weight, 1));
          map[to].insert(from, (weight, 1));
        }
      } else {
        map[from].insert(to, (weight, 1));
        map[to].insert(from, (weight, 1));
      }
    }
  }

  // ダイクストラ法の改変
  let mut weight_map = vec![(usize::MAX, Vec::new()); p + 1];
  weight_map[0] = (0, Vec::new());
  let mut bh = BinaryHeap::new();
  bh.push((Reverse(0), 0));

  // Reverseを使っているので最小値から手に入る
  while let Some((Reverse(w), from)) = bh.pop() {
    for (to, (weight, n)) in map[from].iter() {
      #[allow(clippy::comparison_chain)]
      if weight_map[*to].0 > w + weight {
        // より小さなルートが見つかったので経路は上書き
        weight_map[*to].0 = w + weight;
        let mut l = weight_map[from].clone().1;
        l.push((from, *to, weight * n));
        l.sort();
        l.dedup();
        weight_map[*to].1 = l;
        // 更新されたマップなので追加
        bh.push((Reverse(w + weight), *to));
      } else if weight_map[*to].0 == w + weight {
        // 経路は追加
        let mut l = weight_map[from].clone().1;
        l.push((from, *to, weight * n));
        weight_map[*to].1.append(&mut l);
        weight_map[*to].1.sort();
        weight_map[*to].1.dedup();
      }
    }
  }

  let mut ans = 0;
  let mut check: HashSet<(usize, usize)> = HashSet::new();
  for (to, from, w) in weight_map[p].1.iter() {
    let r1 = to.max(from);
    let r2 = to.min(from);
    if !check.contains(&(*r1, *r2)) {
      ans += w;
      check.insert((*r1, *r2));
    }
  }
  println!("{}", ans * 2);
}
