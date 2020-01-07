pub mod identifiers;
pub mod comments;

use identifiers::*;
use nom::multi::many1;
use nom::IResult;

/// Matches a sequence of digits. The name is copied from TL specs.
pub fn nat_const(i: &str) -> IResult<&str, usize> {
  use std::iter::FromIterator;
  let (input, number) = many1(digit)(i)?;
  let number = String::from_iter(number).parse::<usize>().unwrap(); // The usage of unwrap is safe since many1(digit) will only matches to digits.
  Ok((input, number))
}
