fn calculate_fuel_requirement(mass: &u128) -> u128 {
  let mut fuel_requirement = mass / 3;
  fuel_requirement -= 2;
  fuel_requirement
}

fn calculate_fuel_requirement2(mass: &i128) -> i128 {
  if (mass / 3) - 2 <= 0 {
    let fuel_requirement = mass;
    *fuel_requirement
  } else {
    let mut fuel_requirement = mass / 3;
    fuel_requirement -= 2;
    mass + calculate_fuel_requirement2(&fuel_requirement)
  }
}

fn main() {
  let mut accumulator: i128 = 0;
  let num: [i128; 100] = [
    66690, 86239, 75191, 140364, 95979, 106923, 95229, 123571, 84764, 89444, 98107, 89062, 109369,
    146067, 124760, 76900, 139198, 111441, 74046, 84920, 54397, 143807, 121654, 93863, 73909,
    104121, 58485, 119084, 126227, 142078, 79820, 132617, 108430, 98032, 107434, 127307, 105619,
    57741, 53468, 63301, 137970, 136780, 80897, 133205, 79159, 89124, 94477, 56714, 143704, 122097,
    117335, 108246, 75507, 101459, 101162, 146197, 121884, 66217, 57074, 142903, 140951, 64883,
    124556, 67382, 142407, 121778, 57933, 94599, 87426, 143758, 64043, 65678, 90137, 61090, 77315,
    102383, 146607, 139290, 85394, 149787, 125611, 106405, 91561, 135739, 54845, 68782, 111175,
    61011, 125658, 70751, 85607, 75458, 75419, 124311, 66022, 122784, 129018, 54901, 73788, 108240,
  ];
  for iter in 0 .. num.len() {
    accumulator += calculate_fuel_requirement2(&num[iter]) - &num[iter];
  }

  println!("{}", accumulator);
}
