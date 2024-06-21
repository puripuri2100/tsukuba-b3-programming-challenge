pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();
  let t = s_l.next().unwrap().parse::<usize>().unwrap();
  let mut t_now = 0_usize;
  let mut x_now = 0.0;
  let mut y_now = 0.0;
  let mut l1 = 0.0;
  let mut l2 = 0.0;
  for i in 0..n {
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let x_i = s_l.next().unwrap().parse::<f64>().unwrap();
    let y_i = s_l.next().unwrap().parse::<f64>().unwrap();
    let t_i = s_l.next().unwrap().parse::<usize>().unwrap();
    let xdiff = (x_i - x_now) / (t_i - t_now) as f64;
    let ydiff = (y_i - y_now) / (t_i - t_now) as f64;
    if i == 0 {
      x_now = x_i;
      y_now = y_i;
    } else {
      loop {
        if t_now + t > t_i {break} else {}
      }
    }

    if i == n - 1 {
      // 最期の処理
      let diff = t_i - t_now;
    }
  }
  println!("{l1}");
}

fn f(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
  let xdiff = x1 - x2;
  let ydiff = y1 - y2;
  (xdiff.powf(2.0) as f64 + ydiff.powf(2.0) as f64).sqrt()
}
