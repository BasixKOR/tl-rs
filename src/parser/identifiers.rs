use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::error::ErrorKind;
use nom::multi::{many1, count};
use nom::sequence::preceded;

use std::iter::FromIterator;

// Character class
fn lc_letter(i: &str) -> IResult<&str, char> {
  one_of("abcdefghijklmnopqrstuvwxyz")(i)
}
fn uc_letter(i: &str) -> IResult<&str, char> {
  one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}
fn digit(i: &str) -> IResult<&str, char> {
  one_of("1234567890")(i)
}
fn hex_digit(i: &str) -> IResult<&str, char> {
  alt((digit, one_of("abcdef")))(i)
}
fn underscore(i: &str) -> IResult<&str, char> {
  char('_')(i)
}
fn letter(i: &str) -> IResult<&str, char> {
  alt((lc_letter, uc_letter))(i)
}
fn ident_char(i: &str) -> IResult<&str, char> {
  alt((letter, digit, underscore))(i)
}

// Simple identifiers and keywords
use nom::IResult;
pub fn lc_ident(i: &str) -> IResult<&str, String> {
  let mut string = String::new();
  let (input, first) = lc_letter(i)?;
  string.push(first);
  let (input, last) = many1(ident_char)(input)?;
  string.push_str(String::from_iter(last).as_str());
  Ok((input, string))
}
pub fn uc_ident(i: &str) -> IResult<&str, String> {
  let mut string = String::new();
  let (input, first) = uc_letter(i)?;
  string.push(first);
  let (input, last) = many1(ident_char)(input)?;
  string.push_str(String::from_iter(last).as_str());
  Ok((input, string))
}
/// First of the tuple represents ident, and second is namespace if presented.
pub fn lc_ident_ns(i: &str) -> IResult<&str, (String, Option<String>)> {
  let (input, first) = lc_ident(i)?;
  match preceded(char('.'), lc_ident)(input) {
    Ok((input, second)) => Ok((input, (second, Some(first)))),
    Err(nom::Err::Error((input, ErrorKind::Char))) => Ok((input, (first, None))),
    Err(e) => Err(e),
  }
}
/// First of the tuple represents ident, and second is namespace if presented.
pub fn uc_ident_ns(i: &str) -> IResult<&str, (String, Option<String>)> {
  let (input, first) = lc_ident(i)?; // represents ident if 
  match preceded(char('.'), uc_ident)(input) {
    Ok((input, second)) => Ok((input, (second, Some(first)))),
    Err(nom::Err::Error((input, ErrorKind::Char))) => Ok((input, (first, None))),
    Err(e) => Err(e),
  }
}
/// First of the tuple represents ident, and second is namespace if presented, and third is hexdigit if presented.
pub fn lc_ident_full(i: &str) -> IResult<&str, (String, Option<String>, Option<String>)> {
  let (input, (ident, namespace)) = lc_ident_ns(i)?;
  match preceded(char('#'), count(hex_digit, 8))(input) {
    Ok((input, hex_vec)) => {
      let hex = String::from_iter(hex_vec);
      Ok((input, (ident, namespace, Some(hex))))
    },
    Err(nom::Err::Error((input, ErrorKind::Char))) => Ok((input, (ident, namespace, None))),
    Err(e) => Err(e),
  }
}