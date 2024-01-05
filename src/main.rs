use nom::{
  IResult,
  multi::{many0, many1, separated_list1},
  combinator::{verify, value, recognize, map_res},
  sequence::{preceded, pair, delimited, terminated},
  character::complete::{char, digit1, one_of, multispace1, alphanumeric0, alpha1},
  branch::alt,
  bytes::complete::{tag, is_not, tag_no_case}, Parser,
};
pub fn main(){
  print!("{:#?}", b_inst("b branch"))
  //dtype("ldus[x1, x3]")
}

// recognizes all r-type instructions 
pub fn rtype(input: &str) -> IResult<&str, &str> {
  alt((intrtype, fltrtype))(input)

}

// for parsing r-type instructions into IR.
pub fn r_inst(input: &str) -> IResult<&str, Instruction>{
  let (input, instr) = rtype(input)?; 
  let (input, _) = tag(" ")(input)?;
  let (input, regs)  = separated_list1(alt((tag(", "), tag(","))), reg)(input)?;
  Ok((input, Instruction{typ:Typ::R, instr:instr.to_string(), regs:regs, addr:0, imm:0, bname:"".to_string()}))
}

// for parsing i-type instructions into IR.
pub fn i_inst(input: &str) -> IResult<&str, Instruction>{
  let (input, instr) = itype(input)?; 
  let (input, _) = tag(" ")(input)?;
  let (input, regs)  = separated_list1(alt((tag(", "), tag(","))), reg)(input)?;
  let (input,_) = alt((tag(", "), tag(",")))(input)?;
  let (input, imm) = imm(input)?;
  Ok((input, Instruction{typ:Typ::I, instr:instr.to_string(), regs:regs, addr:0, imm:imm, bname:"".to_string()}))
}

pub fn d_inst(input: &str) -> IResult<&str, Instruction>{
  let (input, instr) = dtype(input)?;
  let (input, _) = tag(" ")(input)?;
  let (input, mut regs)  = separated_list1(alt((tag(", "), tag(","))), reg)(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  if instr.to_lowercase() == "mov" {
    return Ok((input, Instruction{typ:Typ::D, instr:instr.to_string(), regs:regs, addr:0, imm:0, bname:"".to_string()}))

  }
  let (input, _lbrack) = tag("[")(input)?;
  let (input, reg1) = reg(input)?;
  let mut regs2 = vec![reg1];
  regs.append(&mut regs2);
  if instr.to_lowercase() == "lda" {
    let (input, _rbrack) = tag("]")(input)?;
    return Ok((input, Instruction{typ:Typ::D, instr:instr.to_string(), regs:regs, addr:0, imm:0, bname:"".to_string()}))
  }
  let (input, _) = alt((tag(", "), tag(",")))(input)?;
  let (input, dest) = imm(input)?;
  Ok((input, Instruction{typ:Typ::D, instr:instr.to_string(), regs:regs, addr:dest, imm:0, bname:"".to_string()}))
}

pub fn b_inst(input:&str) -> IResult<&str, Instruction>{
  let (input, instr) = btype(input)?;
  let (input, _) = tag(" ")(input)?;
  let(input, branch) = branch_name(input)?;
  Ok((input, Instruction{typ:Typ::B, instr:instr.to_string(), regs:vec![], addr:0, imm:0, bname:branch.to_string()}))
}

pub fn cb_inst(input: &str) -> IResult<&str, Instruction>{
  let (input, instr) = cbtype(input)?;
  let (input, _) = tag(" ")(input)?;
  let(input, branch) = branch_name(input)?;
  Ok((input, Instruction{typ:Typ::C, instr:instr.to_string(), regs:vec![], addr:0, imm:0, bname:branch.to_string()}))
}

// recognizes branch names
pub fn branch_name(input: &str)-> IResult<&str, &str>{
  terminated(alpha1, alphanumeric0)(input)
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

pub fn offset(input: &str) -> IResult<&str, &str>{
  delimited(reg, alt((tag(", "), tag(","))), imm)(input)
}

// recognizes brackets for d-type instructions 
pub fn brack(input: &str) -> IResult<&str, &str> {
    delimited(char('['),offset, char(']'))(input)
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
pub fn parseall(input:&str)-> IResult<&str, &str>{
  alt((reg, comment, brack, rtype, tag(",")))(input)
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


pub struct Branch<'a>{
  pub name: String,
  pub inst: Vec<Instruction<'a>>
}
pub fn parse(code: &str){
  let lines = code.split("\n");
  for line in lines {
    println!("{:?}", line);
    println!("{:#?}", many0(alt((reg, dtype, comment, brack, rtype, multispace1, tag(","))))(line))
  }
  
}