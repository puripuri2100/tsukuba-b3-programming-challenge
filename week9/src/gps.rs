pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();
  let t = s_l.next().unwrap().parse::<usize>().unwrap();
  // GPSでのデータ
  let mut t_now = 0_usize;
  let mut x_now = 0.0;
  let mut y_now = 0.0;
  // 実際の歩いたポイント
  let mut t_tmp = t_now;
  let mut x_tmp = x_now;
  let mut y_tmp = y_now;
  let mut l1 = 0.0;
  let mut l2 = 0.0;
  for i in 0..n {
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let x_i = s_l.next().unwrap().parse::<f64>().unwrap();
    let y_i = s_l.next().unwrap().parse::<f64>().unwrap();
    let t_i = s_l.next().unwrap().parse::<usize>().unwrap();
    let xdiff = (x_i - x_tmp) / (t_i - t_tmp) as f64;
    let ydiff = (y_i - y_tmp) / (t_i - t_tmp) as f64;
    if i == 0 {
      x_now = x_i;
      y_now = y_i;
      x_tmp = x_i;
      y_tmp = y_i;
    } else {
      loop {
        if t_now + t > t_i {
          break;
        } else {
          t_now += t;
          let x = x_tmp + xdiff * (t_now - t_tmp) as f64;
          let y = y_tmp + ydiff * (t_now - t_tmp) as f64;
          let l = f(x_now, y_now, x, y);
          l2 += l;
          x_now = x;
          y_now = y;
        }
      }
      l1 += f(x_i, y_i, x_tmp, y_tmp);
      t_tmp = t_i;
      x_tmp = x_i;
      y_tmp = y_i;
    }

    if i == n - 1 && t_i > t_now {
      // 最期の処理
      let l = f(x_now, y_now, x_i, y_i);
      l2 += l;
    }
  }

  if l1 == 0.0 {
    println!("0.0");
  } else {
    println!("{:.14}", (((l1 - l2) * 100.0) / l1));
  }
}

fn f(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
  let xdiff = x1 - x2;
  let ydiff = y1 - y2;
  (xdiff.powf(2.0) + ydiff.powf(2.0)).sqrt()
}
