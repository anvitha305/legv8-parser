use nom::{
  IResult,
  multi::{many0, many1},
  combinator::{verify, value, recognize},
  sequence::{preceded, pair, delimited, terminated},
  character::complete::{char, digit1, one_of},
  branch::alt,
  bytes::complete::{tag, is_not, tag_no_case},
};
pub fn main(){
  print!("{:#?}", parse("add x1, x3, x4"))
  //dtype("ldus[x1, x3]")
}

// recognizes all r-type instructions 
pub fn rtype(input: &str) -> IResult<&str, &str> {
  alt((intrtype, fltrtype))(input)
}

// recognizes r-type operators/instructions for integers
pub fn intrtype(input: &str) -> IResult<&str, &str> {
  alt(
  (tag_no_case("add"),
  tag_no_case("adds"),
  tag_no_case("and"),
  tag_no_case("ands"),
  tag_no_case("br"),
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
  tag_no_case("mul"),
  tag_no_case("sdiv"),
  tag_no_case("smulh"),
  tag_no_case("udiv"),
  tag_no_case("umulh")))
  (input)
}

// recognizes i-type operators/instructions
pub fn itype(input: &str) -> IResult<&str, &str> {
  alt(
    (tag_no_case("addi"),
    tag_no_case("andi"),
    tag_no_case("andis"),
    tag_no_case("eori"),
    tag_no_case("orri"),
    tag_no_case("subi"),
    tag_no_case("subis")))
    (input)
}

// recognizes d-type operators/instructions
pub fn dtype(input: &str) -> IResult<&str, &str> {
  alt(
    (tag_no_case("ldur"),
    tag_no_case("ldursw"),
    tag_no_case("stur"),
    tag_no_case("lda"),
    tag_no_case("mov")))
    (input)
}


// recognizes cb-type instructions
pub fn cbtype(input: &str) -> IResult<&str, &str> {
  alt(
    (tag_no_case("cbnz"),
    tag_no_case("cbz"),
    tag_no_case("b.le"),
    tag_no_case("b.lt"),
    tag_no_case("b.eq"),
    tag_no_case("b.ne"),
    tag_no_case("b.ge"),
    tag_no_case("b.gt")))
    (input)
}

// recognizes b-type instructions
pub fn btype(input: &str) -> IResult<&str, &str> {
  alt(
    (tag_no_case("b"),
    tag_no_case("bl")))
    (input)
}

// flag updater instructions
pub fn flagtype(input: &str) -> IResult<&str, &str> {
  alt(
    (tag_no_case("cmp"),
    tag_no_case("cmpi")))
    (input)
}

// recognizes brackets for d-type instructions 
pub fn brack(input: &str) -> IResult<&str, &str> {
    delimited(char('['), is_not("[]"), char(']'))(input)
}

// recognizes comments 
pub fn comment(input: &str) -> IResult<&str, &str> {
    value(
      "", // Output is thrown away.
      pair(tag("//"),
      is_not("\n\r"
      )
    )
    )(input)
}



// recognizes values we know immediately
pub fn imm(input: &str) -> IResult<&str, &str> {
      preceded(
        tag("#"),
        recognize(
          many1(
            terminated(one_of("0123456789"), many0(char('_')))
          )
        )
      )(input)
}

// recognizes one of the numbered registers
fn numreg(input: &str) -> IResult<&str, &str> {
    recognize(
      pair(
        tag("x"), 
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
      value("x16", tag("ip0")),
      value("x17", tag("ip0")),
      value("x28", tag("sp")), 
      value("x29", tag("fp")), 
      value("x30", tag("lr")), 
      value("x31", tag("xzr"))
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


// Type of instruction being used.
// R: R-type, register based operations
// I: I-type, immediate instructions working with an immediate memory address.
// D: D-type, load/store operations
// B: B-type, unconditional branching
// C: CB-type, conditional branching
// M: IM-type, moving shifted immediate to register

pub enum Typ {R, I, D, B, C, M}

pub struct Instruction{
	pub typ: Typ,
	pub instr: String,
  pub regs: Vec<String>,
  pub addr: u16    
}

pub struct Branch{
  pub name: String,
  pub inst: Vec<Instruction>
}
pub fn parse(code: &str)->IResult<&str,&str>{
  let lines = code.split("\n");
  for line in lines {
    println!(line)
    println!(alt((reg, comment, imm, brack, rtype))(line))
  }
  
}