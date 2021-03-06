fn print_program(arr: &[u32; 145]) {
  for iter in 0..arr.len() {
    if iter == arr.len() - 1 {
      println!("{}", &arr[iter]);
    } else {
      print!("{}, ", &arr[iter]);
    }
  }
}

fn main() {
  for noun in 0..100 {
    for verb in 0..100 {
      let mut num: [u32; 145] = [
        1, noun, verb, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 9, 23, 1, 23, 6, 27,
        1, 9, 27, 31, 1, 31, 10, 35, 2, 13, 35, 39, 1, 39, 10, 43, 1, 43, 9, 47, 1, 47, 13, 51, 1,
        51, 13, 55, 2, 55, 6, 59, 1, 59, 5, 63, 2, 10, 63, 67, 1, 67, 9, 71, 1, 71, 13, 75, 1, 6,
        75, 79, 1, 10, 79, 83, 2, 9, 83, 87, 1, 87, 5, 91, 2, 91, 9, 95, 1, 6, 95, 99, 1, 99, 5,
        103, 2, 103, 10, 107, 1, 107, 6, 111, 2, 9, 111, 115, 2, 9, 115, 119, 2, 13, 119, 123, 1,
        123, 9, 127, 1, 5, 127, 131, 1, 131, 2, 135, 1, 135, 6, 0, 99, 2, 0, 14, 0,
      ];
      let mut x = 0;
      let mut done = false;

      while !done {
        if num[x] == 1 {
          num[num[x + 3] as usize] = num[num[x + 1] as usize] + num[num[x + 2] as usize];
          x += 4;
        } else if num[x] == 2 {
          num[num[x + 3] as usize] = num[num[x + 1] as usize] * num[num[x + 2] as usize];
          x += 4;
        } else if num[x] == 99 {
          done = true;
          x += 1;
        } else {
          x += 4;
        }
      }
      if num[0] == 19690720 {
        print_program(&num);
        println!("{}", (100 * noun) + verb);
      }
    }
  }
}
