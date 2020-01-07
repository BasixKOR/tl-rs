use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::error::ErrorKind;
use nom::multi::{count, many1};
use nom::sequence::preceded;
use nom::Err::Error;
use nom::IResult;

use std::iter::FromIterator;

// Character class
/// Matches a lowercase letter.
pub fn lc_letter(i: &str) -> IResult<&str, char> {
  one_of("abcdefghijklmnopqrstuvwxyz")(i)
}
/// Matches a uppercase letter.
pub fn uc_letter(i: &str) -> IResult<&str, char> {
  one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}
/// Matches a digit.
pub fn digit(i: &str) -> IResult<&str, char> {
  one_of("1234567890")(i)
}
/// Matches a letter that can be used in a hexdecimal.
pub fn hex_digit(i: &str) -> IResult<&str, char> {
  alt((digit, one_of("abcdef")))(i)
}
/// Matches a alphabet.
pub fn letter(i: &str) -> IResult<&str, char> {
  alt((lc_letter, uc_letter))(i)
}
/// Matches a alphabet, or an underscore.
pub fn ident_char(i: &str) -> IResult<&str, char> {
  alt((letter, digit, char('_')))(i)
}

// Simple identifiers and keywords
/// Matches a TL lowercase identifier. It also represents namespace-ident.
pub fn lc_ident(i: &str) -> IResult<&str, String> {
  let mut string = String::new();
  let (input, first) = lc_letter(i)?;
  string.push(first);
  let (input, last) = many1(ident_char)(input)?;
  string.push_str(String::from_iter(last).as_str());
  Ok((input, string))
}
/// Matches a TL uppercase identifier.
pub fn uc_ident(i: &str) -> IResult<&str, String> {
  let mut string = String::new();
  let (input, first) = uc_letter(i)?;
  string.push(first);
  let (input, last) = many1(ident_char)(input)?;
  string.push_str(String::from_iter(last).as_str());
  Ok((input, string))
}
/// Matches a variable identifier.
pub fn var_ident(i: &str) -> IResult<&str, String> {
  let (input, ident) = many1(ident_char)(i)?; // Official spec says its either lc_ident or uc_ident, but it has same effect as this.
  Ok((input, String::from_iter(ident)))
}
/// Matches a type identifier.
/// First of the tuple represents identifier or #, and second is namespace if presented.
pub fn type_ident(i: &str) -> IResult<&str, (String, Option<String>)> {
  match alt((lc_ident_ns, uc_ident_ns))(i) {
    Ok(result) => Ok(result),
    Err(Error((input, ErrorKind::OneOf))) => {
      let (input, _) = char('#')(input)?;
      Ok((input, (String::from("#"), None)))
    }
    Err(e) => Err(e),
  }
}

/// Matches a TL lowercase identifier with namespace.
/// First of the tuple represents identifier, and second is namespace if presented.
pub fn lc_ident_ns(i: &str) -> IResult<&str, (String, Option<String>)> {
  let (input, first) = lc_ident(i)?;
  match preceded(char('.'), lc_ident)(input) {
    Ok((input, second)) => Ok((input, (second, Some(first)))),
    Err(Error((input, ErrorKind::Char))) => Ok((input, (first, None))),
    Err(e) => Err(e),
  }
}
/// Matches a TL uppercase identifier with namespace.
/// First of the tuple represents ident, and second is namespace if presented.
pub fn uc_ident_ns(i: &str) -> IResult<&str, (String, Option<String>)> {
  let (input, first) = lc_ident(i)?;
  match preceded(char('.'), uc_ident)(input) {
    Ok((input, second)) => Ok((input, (second, Some(first)))),
    Err(Error((input, ErrorKind::Char))) => Ok((input, (first, None))),
    Err(e) => Err(e),
  }
}
/// Matches a TL case-insentensive identifier with namespace.
/// First of the tuple represents ident, and second is namespace if presented.
/// Note that this ident is not defined in TL specs.
pub fn var_ident_ns(i: &str) -> IResult<&str, (String, Option<String>)> {
  let (input, first) = var_ident(i)?;
  match preceded(char('.'), uc_ident)(input) {
    Ok((input, second)) => Ok((input, (second, Some(first)))),
    Err(Error((input, ErrorKind::Char))) => Ok((input, (first, None))),
    Err(e) => Err(e),
  }
}
/// Matches a TL's full identifier, which is used as tokens or parts of combinator identifiers.
/// First of the tuple represents ident, and second is namespace if presented, and third is hexdigit if presented.
pub fn lc_ident_full(i: &str) -> IResult<&str, (String, Option<String>, Option<String>)> {
  let (input, (ident, namespace)) = lc_ident_ns(i)?;
  match preceded(char('#'), count(hex_digit, 8))(input) {
    Ok((input, hex_vec)) => {
      let hex = String::from_iter(hex_vec);
      Ok((input, (ident, namespace, Some(hex))))
    }
    Err(Error((input, ErrorKind::Char))) => Ok((input, (ident, namespace, None))),
    Err(e) => Err(e),
  }
}

#[cfg(test)]
mod tests {
  use super::lc_ident_full;
  #[test]
  fn can_parse_lc_ident_full() {
    assert_eq!(
      lc_ident_full("decryptedMessage#1f814f1f"),
      Ok((
        "",
        (
          String::from("decryptedMessage"),
          None,
          Some(String::from("1f814f1f"))
        )
      ))
    )
  }
}
