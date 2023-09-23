let number = 13;

match number {
  13 => println!("Unlucky!"),
  7 | 21 => println!("Lucky!"),
  14..=19 => println!("Teens."),
  _ => println!("No match default."),
}
