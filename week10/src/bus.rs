#![allow(clippy::needless_range_loop)]

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let n = s.trim().parse::<usize>().unwrap();
  // 74^3 = 405,224
  let mut lst = Vec::new();
  for i in 1..74 {
    if i * i * i > n {
      break;
    } else {
      lst.push(i * i * i)
    }
  }
  let len = lst.len();

  let mut count_lst = vec![0; n + 1];
  for i in 0..len {
    for j in i + 1..len {
      let sum = lst[i] + lst[j];
      if sum <= n {
        count_lst[sum] += 1;
      }
    }
  }

  let mut ans = 0_usize;
  for i in 0..=n {
    let v = count_lst[i];
    if v >= 2 {
      ans = i;
    }
  }
  if ans == 0 {
    println!("none");
  } else {
    println!("{ans}");
  }
}
