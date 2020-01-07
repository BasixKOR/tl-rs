use nom::IResult;
use nom::sequence::delimited;
use nom::bytes::complete::{tag, take_until};

pub fn single_line_comment(i: &str) -> IResult<&str, &str> {
  delimited(tag("//"), take_until("\n"), tag("\n"))(i)
}
pub fn multi_line_comment(i: &str) -> IResult<&str, &str> {
  delimited(tag("/*"), take_until("*/"), tag("*/"))(i)
}