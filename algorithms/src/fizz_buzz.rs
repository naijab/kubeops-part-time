
#[derive(Debug)]
pub enum FizzBuzzError {
  NotFizzOrBuzzNumber,
}

pub fn fizz_buzz(x: i32) -> Result<String, FizzBuzzError>  {
  match (x % 3, x % 5) {
    (0, 0) => Ok(String::from("FizzBuzz")),
    (0, _) => Ok(String::from("Fizz")),
    (_, 0) => Ok(String::from("Buzz")),
    _ => Err(FizzBuzzError::NotFizzOrBuzzNumber),
  }     
}

#[cfg(test)]
mod fizz_buzz_test { 
  use super::*;

  #[test]
  fn it_21_should_be_fizz() {
    let result = fizz_buzz(21);
    let _ = match result {
      Ok(actual) => assert_eq!(actual, "Fizz"),
      Err(error) => panic!("Error : {:?}", error),
    };
  }

  #[test]
  fn it_25_should_be_buzz() {
    let result = fizz_buzz(25);
    let _ = match result {
      Ok(actual) => assert_eq!(actual, "Buzz"),
      Err(error) => panic!("Error : {:?}", error),
    };
  }

  #[test]
  fn it_45_should_be_fizz_buzz() {
    let result = fizz_buzz(45);
    let _ = match result {
      Ok(actual) => assert_eq!(actual, "FizzBuzz"),
      Err(error) => panic!("Error : {:?}", error),
    };
  }

  #[test]
  #[should_panic]
  fn it_101_should_be_not_fizz_buzz() {
    let result = fizz_buzz(101);
    let _ = match result {
      Ok(data) => data,
      Err(error) => panic!("Error : {:?}", error),
    };
  }

}
