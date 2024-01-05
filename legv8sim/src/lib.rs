use pyo3::prelude::*;
use nom::{
    IResult,
    multi::{many0, many1, separated_list1},
    combinator::{verify, value, recognize, map_res},
    sequence::{preceded, pair, delimited, terminated},
    character::complete::{char, digit1, one_of, multispace1, alphanumeric0, alpha1},
    branch::alt,
    bytes::complete::{tag, is_not, tag_no_case}, Parser,
  };
#[pyfunction]
// recognizes all r-type instructions 
pub fn rtype(input: &str) -> PyResult<IResult<&str, &str>> {
    Ok(alt((intrtype, fltrtype))(input))
}

  
/// A Python module implemented in Rust.
#[pymodule]
fn legv8sim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
