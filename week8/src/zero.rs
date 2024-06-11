pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();
    if n == 0 {
      break;
    }

    let mut count_2 = 0_usize;
    let mut count_5 = 0_usize;
    let mut v = 1_usize;

    for i in 2..=n {
      let mut i = i;
      loop {
        if i % 2 == 0 {
          i /= 2;
          count_2 += 1;
        } else {
          break;
        }
      }
      loop {
        if i % 5 == 0 {
          i /= 5;
          count_5 += 1;
        } else {
          break;
        }
      }
      v *= i;
      v %= 10;
    }

    #[allow(clippy::comparison_chain)]
    if count_2 > count_5 {
      let mut t = 1_usize;
      for _ in 0..(count_2 - count_5) {
        t *= 2;
        t %= 10;
      }
      println!("{}", (v * t) % 10);
    } else if count_2 < count_5 {
      let mut t = 1_usize;
      for _ in 0..(count_5 - count_2) {
        t *= 5;
        t %= 10;
      }
      println!("{}", (v * t) % 10);
    } else {
      println!("{}", v % 10);
    }
  }
}
