use pyo3::prelude::*;
use nom::{
    IResult,
    multi::{many0, many1, separated_list1},
    combinator::{verify, value, recognize, map_res},
    sequence::{preceded, pair, terminated},
    character::complete::{char, digit1, one_of, alphanumeric0, alpha1},
    branch::alt,
    bytes::complete::{tag, tag_no_case}, Parser,
  };

// recognizes all r-type instructions 
pub fn rtype(input: &str) -> PyResult<IResult<&str, &str>> {
    Ok(alt((intrtype, fltrtype))(input))
}
pub fn itype(input: &str) -> PyResult<IResult<&str, &str>> {
    Ok(alt(
      (tag_no_case("addi"),
      tag_no_case("cmpi"),
      tag_no_case("andi"),
      tag_no_case("andis"),
      tag_no_case("eori"),
      tag_no_case("orri"),
      tag_no_case("subi"),
      tag_no_case("subis")))
      (input))
  }
  
// recognizes d-type operators/instructions
pub fn dtype(input: &str) -> PyResult<IResult<&str, &str>> {
    Ok(alt(
      (tag_no_case("ldur"),
      tag_no_case("ldursw"),
      tag_no_case("stur"),
      tag_no_case("lda"),
      tag_no_case("mov")))
      (input))
  }


// recognizes cb-type instructions
pub fn cbtype(input: &str) -> PyResult<IResult<&str, &str>> {
    Ok(alt(
      (tag_no_case("cbnz"),
      tag_no_case("cbz"),
      tag_no_case("b.le"),
      tag_no_case("b.lt"),
      tag_no_case("b.eq"),
      tag_no_case("b.ne"),
      tag_no_case("b.ge"),
      tag_no_case("b.gt")))
      (input))
  }
  
  // recognizes b-type instructions
  pub fn btype(input: &str) -> PyResult<IResult<&str, &str>> {
    Ok(alt(
      (tag_no_case("bl"),
      tag_no_case("b")))
      (input))
  }

// recognizes r-type operators/instructions for integers
pub fn intrtype(input: &str) -> IResult<&str, &str> {
    alt(
    (tag_no_case("add"),
    tag_no_case("adds"),
    tag_no_case("and"),
    tag_no_case("ands"),
    tag_no_case("br"),
    tag_no_case("cmp"),
    tag_no_case("eor"),
    tag_no_case("lsl"),
    tag_no_case("lsr"),
    tag_no_case("orr"),
    tag_no_case("sub"),
    tag_no_case("subs")))
    (input)
  }
  
  // recognizes r-type operators/instructions for floats
  pub fn fltrtype(input: &str) -> IResult<&str, &str> {
    alt(
    (tag_no_case("fadd"),
    tag_no_case("fcmpd"),
    tag_no_case("fdivd"),
    tag_no_case("fmuld"),
    tag_no_case("fsubd"),
    tag_no_case("fmuld"),
    tag_no_case("fsubd"),
    tag_no_case("mul"),
    tag_no_case("sdiv"),
    tag_no_case("smulh"),
    tag_no_case("udiv"),
    tag_no_case("umulh")))
    (input)
  }
  
// recognizes one of the numbered registers
fn numreg(input: &str) -> IResult<&str, &str> {
    recognize(
      pair(
        tag_no_case("x"), 
        verify(
          digit1, 
          |s: &str| (0..31).contains(&(s.parse().unwrap())
      )
    )
  )
    )(input)
}

// recognizes one of the named registers and converts it to the numbered registers
fn altreg(input: &str) -> IResult<&str, &str> {
    alt((
      value("x16", tag_no_case("ip0")),
      value("x17", tag_no_case("ip0")),
      value("x28", tag_no_case("sp")), 
      value("x29", tag_no_case("fp")), 
      value("x30", tag_no_case("lr")), 
      value("x31", tag_no_case("xzr"))
    )
  )(input)
}

// combined parser for registers [both numbered and non numbered]
pub fn reg(input: &str) -> IResult<&str, &str> {
  alt ((
    altreg,
    numreg,
  )
  )(input)
}
// recognizes values we know immediately
pub fn imm(input: &str) -> IResult<&str, u64> {
    map_res(preceded(
      tag("#"),
      recognize(
        many1(
          terminated(one_of("0123456789"), many0(char('_')))
        )
      )
    ), |out: &str|out.to_string().replace("#", "").parse()).parse(input)
}
// recognizes branch names
pub fn branch_name(input: &str)-> IResult<&str, &str>{
    terminated(alpha1, alphanumeric0)(input)
  }
  

// Type of instruction being used.
// R: R-type, register based operations
// I: I-type, immediate instructions working with an immediate memory address.
// D: D-type, load/store operations
// B: B-type, unconditional branching
// C: CB-type, conditional branching
// M: IM-type, moving shifted immediate to register
#[derive(Debug)]
pub enum Typ {R, I, D, B, C}

#[derive(Debug)]
pub struct Instruction<'a>{
	pub typ: Typ,
	pub instr: String,
  pub regs: Vec<&'a str>,
  pub addr: u64,   
  pub imm: u64,
  pub bname: String,
}
impl ToString for Instruction<'_>{
    fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}

pub struct Branch<'a>{
  pub name: String,
  pub inst: Vec<Instruction<'a>>
}

#[pyfunction]
// for parsing r-type instructions into IR.
pub fn r_inst(input: &str) -> PyResult<String>{
    let Ok((input, instr)) = rtype(input).unwrap() else{panic!()}; 
    let (_input, regs)  = preceded(tag(" "),separated_list1(alt((tag(", "), tag(","))), reg))(input).unwrap();
    Ok(Instruction{typ:Typ::R, instr:instr.to_string(), regs:regs, addr:0, imm:0, bname:"".to_string()}.to_string())
  }
#[pyfunction]
// for parsing i-type instructions into IR.
pub fn i_inst(input: &str) -> PyResult<String>{
    let Ok((input, instr)) = itype(input).unwrap() else{panic!()}; 
    let (input, regs)  = preceded(tag(" "), separated_list1(alt((tag(", "), tag(","))), reg))(input).unwrap();
    let (_input, imm) = preceded(alt((tag(", "), tag(","))), imm)(input).unwrap();
    Ok(Instruction{typ:Typ::I, instr:instr.to_string(), regs:regs, addr:0, imm:imm, bname:"".to_string()}.to_string())
  }
   
#[pyfunction]
pub fn d_inst(input: &str) -> PyResult<String>{
    let Ok((input, instr)) = dtype(input).unwrap() else{panic!()};
    let (input, mut regs)  = preceded(tag(" "), preceded(separated_list1(alt((tag(", "), tag(","))), reg), many0(tag(" "))))(input).unwrap();
    if instr.to_lowercase() == "mov" {
      return Ok(Instruction{typ:Typ::D, instr:instr.to_string(), regs:regs, addr:0, imm:0, bname:"".to_string()}.to_string())
  
    }
    
    if instr.to_lowercase() == "lda" {
      let (_input, reg1) = preceded(tag("["), preceded(reg, tag("]")))(input).unwrap();
      let mut regs2 = vec![reg1];
      regs.append(&mut regs2);
      return Ok(Instruction{typ:Typ::D, instr:instr.to_string(), regs:regs, addr:0, imm:0, bname:"".to_string()}.to_string())
    }
    let (input, reg1) = preceded(preceded(alt((tag(", "), tag(","))), tag("[")), reg)(input).unwrap();
    let mut regs2 = vec![reg1];
    regs.append(&mut regs2);
    let (_input, dest) = preceded(alt((tag(", "), tag(","))), terminated(imm, tag("]")))(input).unwrap();
    //panic!("{:#?}", dest);
    Ok(Instruction{typ:Typ::D, instr:instr.to_string(), regs:regs, addr:dest, imm:0, bname:"".to_string()}.to_string())
  }

#[pyfunction]
  pub fn b_inst(input:&str) -> PyResult<String>{
    let Ok((input, instr)) = btype(input).unwrap() else{panic!()};
    let(_input, branch) = preceded(tag(" "), branch_name)(input).unwrap();
    Ok(Instruction{typ:Typ::B, instr:instr.to_string(), regs:vec![], addr:0, imm:0, bname:branch.to_string()}.to_string())
  }
#[pyfunction]
  pub fn cb_inst(input: &str) ->PyResult<String>{
    let Ok((input, instr)) = cbtype(input).unwrap() else{panic!()};
    let(_input, branch) = preceded(tag(" "), branch_name)(input).unwrap();
    Ok(Instruction{typ:Typ::C, instr:instr.to_string(), regs:vec![], addr:0, imm:0, bname:branch.to_string()}.to_string())
  }

/// A Python module implemented in Rust.
#[pymodule]
fn legv8sim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(r_inst, m)?)?;
    m.add_function(wrap_pyfunction!(i_inst, m)?)?;
    m.add_function(wrap_pyfunction!(d_inst, m)?)?;
    m.add_function(wrap_pyfunction!(b_inst, m)?)?;
    m.add_function(wrap_pyfunction!(cb_inst, m)?)?;
    Ok(())
}
