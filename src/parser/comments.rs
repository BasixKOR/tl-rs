use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::IResult;

pub fn single_line_comment(i: &str) -> IResult<&str, &str> {
  use nom::character::complete::{newline, not_line_ending};
  delimited(tag("//"), not_line_ending, newline)(i)
}
pub fn multi_line_comment(i: &str) -> IResult<&str, &str> {
  use nom::bytes::complete::take_until;
  delimited(tag("/*"), take_until("*/"), tag("*/"))(i)
}
