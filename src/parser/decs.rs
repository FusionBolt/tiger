use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple};
use crate::ir::expr::{TDec, TNameType, TType};
use crate::parser::common::identifier;
use nom::multi::{many0, many_m_n, separated_list0};
use crate::parser::ty::parse_type;
use nom::error::context;
use crate::ir::expr::{LSpan, Span};
use crate::parser::fun::parse_fun;
use crate::parser::var::{parse_var};


// tydec -> type type-id = ty
fn parse_type_dec(i: LSpan) -> IResult<LSpan, TDec> {
    let (i, (_, _, type_id, _, _, _, type_info, _)) =
        delimited(multispace0, tuple((tag("type"), multispace0, identifier, multispace0, tag("="), multispace0, parse_type, multispace0)), multispace0)(i)?;
    Ok((i, TDec::TypeDec(vec![TNameType { name: type_id.to_string(), ty: type_info }])))
}

fn parse_var_dec(i: LSpan) -> IResult<LSpan, TDec> {
    parse_var(i)
}

fn parse_fun_dec(i: LSpan) -> IResult<LSpan, TDec> {
    parse_fun(i)
    //let (i, (_, _, _)) = delimited(multispace0, tuple((tag("function"), multispace0, identifier)), multispace0)(i)?;
}

pub fn parse_dec(i: LSpan) -> IResult<LSpan, TDec> {
    context("parse_dec" ,alt((parse_type_dec, parse_var_dec, parse_fun_dec)))(i)
}

pub fn parse_decs(i: LSpan) -> IResult<LSpan, Vec<TDec>> {
    many0(parse_dec)(i)
}

#[cfg(test)]
mod tests {
    use crate::ir::expr::{TDec, TNameType, TType, LSpan};
    use crate::parser::decs::parse_type_dec;

    fn assert_type_dec(i: &str, o: TDec) {
        match parse_type_dec(LSpan::new(i)) {
            Ok((l, res)) => {
                assert_eq!(res, o)
            } Err(_) => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_type_dec() {
        assert_type_dec("type a = int", TDec::TypeDec(vec![TNameType {
            name: "a".to_string(),
            ty: TType::NameType("int".to_string()),
        }]))
    }
}