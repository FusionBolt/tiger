use crate::ir::expr;
use nom::{IResult, bytes::complete::{tag, take_while_m_n}, combinator::map_res, character::complete::char, sequence::tuple, Or};
use nom::sequence::{delimited, preceded, pair};
use nom::bytes::complete::is_not;
use nom::branch::alt;
use nom::combinator::{opt, recognize};
use nom::multi::many0;
use nom::character::complete::{alpha0, alpha1, alphanumeric1, anychar, multispace0, space0};
use crate::ir::expr::TDec;

// todo:nested
fn parse_comment(i: &str) -> IResult<&str, &str> {
    preceded(multispace0, delimited(tag("/*"), is_not("*/"), tag("*/")))(i)
}


fn parse_source(i: &str) {
    
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::parse_comment;

    #[test]
    fn test_comment() {
        assert_eq!(parse_comment("/*this*/"), Ok(("", "this")));
        assert_eq!(parse_comment("/*this is comment*/"), Ok(("", "this is comment")));
        assert_eq!(parse_comment("/*this is \r escape comment*/"), Ok(("", "this is \r escape comment")));
    }
}