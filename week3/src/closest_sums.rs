fn gen_sum_lst(lst: &[isize]) -> Vec<isize> {
  let mut v = Vec::new();
  let len = lst.len();
  for i in 0..len {
    if i == len - 1 {
      break;
    }
    for j in i + 1..len {
      v.push(lst[i] + lst[j]);
    }
  }
  v.sort();
  v.dedup();
  v
}

fn search_lst(n: isize, lst: &[isize]) -> isize {
  let mut tmp1 = isize::MIN;
  for i in lst.iter() {
    if *i < n {
      tmp1 = *i;
    } else if *i == n {
      tmp1 = *i;
      break;
    } else {
      let diff1 = tmp1.abs_diff(n);
      let diff2 = i.abs_diff(n);
      if diff1 > diff2 {
        tmp1 = *i;
      }
      break;
    }
  }
  tmp1
}

#[derive(Debug, Clone)]
pub struct T {
  pub n: usize,
  pub n_lst: Vec<isize>,
  pub m: usize,
  pub m_lst: Vec<isize>,
}

pub fn main() {
  let mut v = Vec::new();
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    if let Ok(n) = s.trim().parse::<usize>() {
      let mut n_lst = Vec::new();
      for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let i = s.trim().parse::<isize>().unwrap();
        n_lst.push(i);
      }
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).ok();
      let m = s.trim().parse::<usize>().unwrap();
      let mut m_lst = Vec::new();
      for _ in 0..m {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let i = s.trim().parse::<isize>().unwrap();
        m_lst.push(i);
      }
      v.push(T { n, n_lst, m, m_lst });
    } else {
      break;
    }
  }

  for (i, t) in v.iter().enumerate() {
    println!("Case {}:", i + 1);
    let l = gen_sum_lst(&t.n_lst);
    for n in t.m_lst.iter() {
      let ans = search_lst(*n, &l);
      println!("Closest sum to {} is {}.", n, ans);
    }
  }
}
