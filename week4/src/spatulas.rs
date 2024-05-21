#[derive(Debug, Clone, Copy)]
struct AnsInfo {
  start: usize,
  end: usize,
  sum: isize,
}

impl AnsInfo {
  fn max(&self, other: &Self) -> Self {
    if self.sum > other.sum {
      *self
    } else if self.sum < other.sum {
      *other
    } else {
      if self.end - self.start > other.end - other.start {
        *other
      } else if self.end - self.start < other.end - other.start {
        *self
      } else {
        if self.start < other.start {
          *self
        } else {
          *other
        }
      }
    }
  }

  fn max_cost(&self, other: &Cost) -> Self {
    if self.sum > other.p {
      *self
    } else {
      AnsInfo {
        start: other.t,
        end: other.t,
        sum: other.p,
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Cost {
  t: usize,
  p: isize,
  // 間のコスト
  d: isize,
}

pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();
    if n == 0 {
      break;
    }

    let mut cost_lst = Vec::new();
    let mut time_lst = Vec::new();
    let mut time = 0;
    for i in 0..n {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      let mut s_l = s.trim().split_whitespace();
      let t = s_l.next().unwrap().parse::<usize>().unwrap();
      let p = (s_l.next().unwrap().parse::<f64>().unwrap() * 100.0) as isize;
      time_lst.push(t);
      if i == 0 {
        cost_lst.push(Cost { t, p: p - 8, d: 0 });
        time = t;
      } else {
        cost_lst.push(Cost {
          t,
          p: p - 8,
          d: (t - time - 1) as isize * 8,
        });
        time = t;
      }
    }

    let mut ans = AnsInfo {
      start: cost_lst[0].t,
      end: cost_lst[0].t,
      sum: cost_lst[0].p,
    };
    let mut tmp = AnsInfo {
      start: cost_lst[0].t,
      end: cost_lst[0].t,
      sum: cost_lst[0].p,
    };
    for (i, _) in time_lst.iter().enumerate() {
      if i != 0 {
        // 先頭の処理はもう終了しているのでそれ以降を取り扱う
        let next_cost = cost_lst[i];
        // 次の期間も開けた場合の利益
        let next_tmp = AnsInfo {
          start: tmp.start,
          end: next_cost.t,
          sum: tmp.sum + next_cost.p - next_cost.d,
        };
        // 利益が多い方でtmpを更新
        tmp = next_tmp.max_cost(&next_cost);
        // 既存の利益と比較して大きい方で更新
        ans = ans.max(&tmp);
      }
      //println!("{tmp:?}");
    }

    if ans.sum <= 0 {
      println!("no profit");
    } else {
      println!("{:.2} {} {}", ans.sum as f64 / 100.0, ans.start, ans.end);
    }
  }
}
