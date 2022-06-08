use crate::StringList;
use crate::definitions::*;

use logos::{Logos, Lexer};
use std::fmt;
use std::str::FromStr;
use colored::Colorize;

pub type TokenStream = Vec<Token>;

pub type TokenResult = Result<TokenStream, StringList>;


pub struct Token {
    pub contents: Tok,
    pub slice: String
}

#[derive(Logos)]
#[logos(subpattern name = r"[a-zA-Z][_0-9a-zA-Z]*")]
#[logos(subpattern empty = r"[ \t\n\r\f]")]
#[logos(subpattern bin = r"[01][_01]*")]
#[logos(subpattern dec = r"[0-9][_0-9]*")]
#[logos(subpattern hex = r"[0-9a-fA-F][_0-9a-fA-F]*")]
#[logos(subpattern oct = r"[0-7][_0-7]*")]
#[logos(subpattern reg = r"\$(([0-9]|[12][0-9]|3[01])|t[0-9]|zero|at|[vk][01]|a[0-3]|s[0-8]|[gsf]p|ra|f([0-9]|[12][0-9]|3[01]))")]
pub enum Tok {
    #[regex(r"#.*", Tok::string)]
    Comment(String),

    #[regex(r#""(.*)(?x)"#, Tok::string)]
	String(String),

    #[regex(r"[+-]", Op::new)]
    Operand(Op),

    #[regex(r"\((?&reg)\)", Addr::register)]
    #[regex(r"(?&dec)\((?&reg)\)", Addr::imm_register_dec)]
    #[regex(r"0b(?&bin)\((?&reg)\)", Addr::imm_register_bin)]
    #[regex(r"0x(?&hex)\((?&reg)\)", Addr::imm_register_hex)]
    #[regex(r"0o(?&oct)\((?&reg)\)", Addr::imm_register_oct)]
    #[regex(r"(?&name)\((?&reg)\)", Addr::label_register)]
    Address(Addr),
    
    #[regex(r"(?&name):", Tok::string)]
    Label(Label),
    
    #[regex(r"(?&reg)", Tok::string)]
    Register(Register),
    
    #[regex(r#"'(.*)(?x)"#, Imm::character)]
    #[regex(r"(?&dec)", Imm::dec)]
	#[regex(r"0b(?&bin)", Imm::bin)]
	#[regex(r"0x(?&hex)", Imm::hex)]
	#[regex(r"0o(?&oct)", Imm::oct)]
	Immediate(Imm),

    #[regex(r".?(?&name)", Wrd::new)]
    Word(Wrd),

    

    #[error]
	#[regex(r"(?&empty)+", logos::skip)]
	Error,
}




impl Addr {
    pub fn register(l: &mut Lexer<Tok>) -> Addr {
        let s: String = l.slice().to_string();
        let reg_s = s[1..s.len() - 1].to_string();
        return Addr::Register(reg_s);
    }

    pub fn imm_register_dec(l: &mut Lexer<Tok>) -> Option<Addr> {
        let s: String = l.slice().to_string();
        let parts = s.split("(").collect::<Vec<&str>>();
        return Some(
            Addr::ImmRegister(
                Imm::dec_from_str(parts[0]).expect(""), 
                parts[1][..parts[1].len() - 1].to_string()
            )
        );
    }

    pub fn imm_register_hex(l: &mut Lexer<Tok>) -> Option<Addr> {
        let s: String = l.slice()[2..].to_string();
        let parts = s.split("(").collect::<Vec<&str>>();
        return Some(
            Addr::ImmRegister(
                Imm::hex_from_str(parts[0]).expect(""), 
                parts[1][..parts[1].len() - 1].to_string()
            )
        );
    }

    pub fn imm_register_oct(l: &mut Lexer<Tok>) -> Option<Addr> {
        let s: String = l.slice()[2..].to_string();
        let parts = s.split("(").collect::<Vec<&str>>();
        return Some(
            Addr::ImmRegister(
                Imm::oct_from_str(parts[0]).expect(""), 
                parts[1][..parts[1].len() - 1].to_string()
            )
        );
    }

    pub fn imm_register_bin(l: &mut Lexer<Tok>) -> Option<Addr> {
        let s: String = l.slice()[2..].to_string();
        let parts = s.split("(").collect::<Vec<&str>>();
        return Some(
            Addr::ImmRegister(
                Imm::bin_from_str(parts[0]).expect(""), 
                parts[1][..parts[1].len() - 1].to_string()
            )
        );
    }

    pub fn label_register(l: &mut Lexer<Tok>) -> Option<Addr> {
        let s: String = l.slice().to_string();
        let parts = s.split("(").collect::<Vec<&str>>();
        return Some(
            Addr::LabelRegister(
                parts[0].to_string(), 
                parts[1][..parts[1].len() - 1].to_string()
            )
        );
    }
}

impl Op {
    pub fn new(l: &mut Lexer<Tok>) -> Op {
        let s = l.slice();
        match s {
            "+" => Op::Add,
            "-" => Op::Subtract,
            _ => todo!()
        }
    } 
}

impl Wrd {
    pub fn new(l: &mut Lexer<Tok>) -> Wrd {
        let s = l.slice();
        let sl = s.to_lowercase();

        match sl.as_str() {
            //Instructions
            "add" => Wrd::Instruction(Inst::AddWord),
            "addi" => Wrd::Instruction(Inst::AddImmediateWord),
            "addu" => Wrd::Instruction(Inst::AddUnsignedWord),
            "addiu" => Wrd::Instruction(Inst::AddImmediateUnsignedWord),
            "sub" => Wrd::Instruction(Inst::SubtractWord),
            "subu" => Wrd::Instruction(Inst::SubtractUnsignedWord),
            "mul" => Wrd::Instruction(Inst::MultiplyWordtoGPR),
            "mult" => Wrd::Instruction(Inst::MultiplyWord),
            "multu" => Wrd::Instruction(Inst::MultiplyUnsignedWord),
            "madd" => Wrd::Instruction(Inst::MultiplyandAddWordtoHiLo),
            "maddu" => Wrd::Instruction(Inst::MultiplyandAddUnsignedWordtoHiLo),
            "msub" => Wrd::Instruction(Inst::MultiplyandSubtractWordtoHiLo),
            "msubu" => Wrd::Instruction(Inst::MultiplyandSubtractUnsignedWordtoHiLo),
            "div" => Wrd::Instruction(Inst::DivideWord),
            "divu" => Wrd::Instruction(Inst::DivideUnsignedWord),
            "rem" => Wrd::Instruction(Inst::RemainderafterWordDivision),
            "remu" => Wrd::Instruction(Inst::RemainderafterUnsignedWordDivision),
            "clo" => Wrd::Instruction(Inst::CountLeadingOnesinWord),
            "clz" => Wrd::Instruction(Inst::CountLeadingZeroesinWord),
            "seb" => Wrd::Instruction(Inst::SignExtendByte),
            "seh" => Wrd::Instruction(Inst::SignExtendHalfword),
            "seq" => Wrd::Instruction(Inst::SetonEqual),
            "sne" => Wrd::Instruction(Inst::SetonNotEqual),
            "sle" => Wrd::Instruction(Inst::SetonLessThanorEqual),
            "sleu" => Wrd::Instruction(Inst::SetonLessThanorEqualUnsigned),
            "slt" => Wrd::Instruction(Inst::SetonLessThan),
            "sltu" => Wrd::Instruction(Inst::SetonLessThanUnsigned),
            "sgt" => Wrd::Instruction(Inst::SetonGreaterThanorEqual),
            "sgtu" => Wrd::Instruction(Inst::SetonGreaterThanorEqualUnsigned),
            "sge" => Wrd::Instruction(Inst::SetonGreaterThanorEqual),
            "sgeu" => Wrd::Instruction(Inst::SetonGreaterThanorEqualUnsigned),
            "slti" => Wrd::Instruction(Inst::SetonLessThanImmediate),
            "sltiu" => Wrd::Instruction(Inst::SetonLessThanImmediateUnsigned),
            "abs" => Wrd::Instruction(Inst::AbsoluteValue),
            "neg" => Wrd::Instruction(Inst::NegateValue),
            "negu" => Wrd::Instruction(Inst::NegateValueUnsigned),
            "and" => Wrd::Instruction(Inst::And),
            "andi" => Wrd::Instruction(Inst::AndImmediate),
            "or" => Wrd::Instruction(Inst::Or),
            "ori" => Wrd::Instruction(Inst::OrImmediate),
            "nor" => Wrd::Instruction(Inst::NotOr),
            "xor" => Wrd::Instruction(Inst::ExclusiveOr),
            "xori" => Wrd::Instruction(Inst::ExclusiveOrImmediate),
            "not" => Wrd::Instruction(Inst::BitwiseNot),
            "rol" => Wrd::Instruction(Inst::RotateLeft),
            "ror" => Wrd::Instruction(Inst::RotateRight),
            "rotr" => Wrd::Instruction(Inst::RotateWordRight),
            "rotrv" => Wrd::Instruction(Inst::RotateWordRightVariable),
            "sll" => Wrd::Instruction(Inst::ShiftWordLeftLogical),
            "sllv" => Wrd::Instruction(Inst::ShiftWordLeftLogicalVariable),
            "sra" => Wrd::Instruction(Inst::ShiftWordRightArithmetic),
            "srav" => Wrd::Instruction(Inst::ShiftWordRightArithmeticVariable),
            "srl" => Wrd::Instruction(Inst::ShiftWordRightLogical),
            "srlv" => Wrd::Instruction(Inst::ShiftWordRightLogicalVariable),
            "li" => Wrd::Instruction(Inst::LoadImmediate),
            "la" => Wrd::Instruction(Inst::LoadAddress),
            "lui" => Wrd::Instruction(Inst::LoadUpperImmediate),
            "lb" => Wrd::Instruction(Inst::LoadByte),
            "lbu" => Wrd::Instruction(Inst::LoadByteUnsigned),
            "lh" => Wrd::Instruction(Inst::LoadHalfword),
            "lhu" => Wrd::Instruction(Inst::LoadHalfwordUnsigned),
            "lw" => Wrd::Instruction(Inst::LoadWord),
            "ld" => Wrd::Instruction(Inst::LoadDoubleWord),
            "sb" => Wrd::Instruction(Inst::StoreByte),
            "sh" => Wrd::Instruction(Inst::StoreHalfword),
            "sw" => Wrd::Instruction(Inst::StoreWord),
            "sd" => Wrd::Instruction(Inst::StoreDoubleWord),
            "push" => Wrd::Instruction(Inst::Pushregistertostack),
            "pop" => Wrd::Instruction(Inst::Popregisterfromstack),
            "begin" => Wrd::Instruction(Inst::Createstackframe),
            "end" => Wrd::Instruction(Inst::Destroystackframe),
            "mfhi" => Wrd::Instruction(Inst::MoveFromHIRegister),
            "mflo" => Wrd::Instruction(Inst::MoveFromLORegister),
            "mthi" => Wrd::Instruction(Inst::MoveToHIRegister),
            "mtlo" => Wrd::Instruction(Inst::MoveToLORegister),
            "move" => Wrd::Instruction(Inst::Move),
            "movz" => Wrd::Instruction(Inst::MoveConditionalonZero),
            "movn" => Wrd::Instruction(Inst::MoveConditionalonNotZero),
            "b" => Wrd::Instruction(Inst::UnconditionalBranch),
            "bal" => Wrd::Instruction(Inst::BranchandLink),
            "beq" => Wrd::Instruction(Inst::BranchonEqual),
            "beqz" => Wrd::Instruction(Inst::BranchonEqualtoZero),
            "bne" => Wrd::Instruction(Inst::BranchonNotEqual),
            "bnez" => Wrd::Instruction(Inst::BranchonNotEqualtoZero),
            "bge" => Wrd::Instruction(Inst::BranchonGreaterThanorEqual),
            "bgeu" => Wrd::Instruction(Inst::BranchonUnsignedGreaterThanorEqual),
            "bgez" => Wrd::Instruction(Inst::BranchonGreaterThanorEqualtoZero),
            "bgt" => Wrd::Instruction(Inst::BranchonGreaterThan),
            "bgtu" => Wrd::Instruction(Inst::BranchonUnsignedGreaterThan),
            "bgtz" => Wrd::Instruction(Inst::BranchonGreaterThanZero),
            "blt" => Wrd::Instruction(Inst::BranchonLessThan),
            "bltu" => Wrd::Instruction(Inst::BranchonUnsignedLessThan),
            "bltz" => Wrd::Instruction(Inst::BranchonLessThanZero),
            "ble" => Wrd::Instruction(Inst::BranchonLessThanorEqual),
            "bleu" => Wrd::Instruction(Inst::BranchonUnsignedLessThanorEqual),
            "blez" => Wrd::Instruction(Inst::BranchonLessThanorEqualtoZero),
            "bgezal" => Wrd::Instruction(Inst::BranchonGreaterThanorEqualtoZeroandLink),
            "bltzal" => Wrd::Instruction(Inst::BranchonLessThanZeroandLink),
            "j" => Wrd::Instruction(Inst::Jump),
            "jal" => Wrd::Instruction(Inst::JumpandLink),
            "jr" => Wrd::Instruction(Inst::JumpRegister),
            "jalr" => Wrd::Instruction(Inst::JumpandLinkRegister),
            "syscall" => Wrd::Instruction(Inst::Syscall),
            // Directives
            ".text" => Wrd::Directive(Dir::Text),
            ".data" => Wrd::Directive(Dir::Data),
            ".ascii" => Wrd::Directive(Dir::Ascii),
            ".asciiz" => Wrd::Directive(Dir::Asciiz),
            ".byte" => Wrd::Directive(Dir::Byte),
            ".half" => Wrd::Directive(Dir::Half),
            ".float" => Wrd::Directive(Dir::Float),
            ".double" => Wrd::Directive(Dir::Double),
            ".globl" => Wrd::Directive(Dir::Globl),
            ".space" => Wrd::Directive(Dir::Space),
            ".align" => Wrd::Directive(Dir::Align),
            ".word" => Wrd::Directive(Dir::Word),
            _ => Wrd::Identifier(s.trim().to_string())
        }
    }
}

impl Imm {
    fn character(l: &mut Lexer<Tok>) -> Option<Imm> {
        let chars = l.slice().strip_prefix("'")?.strip_suffix("'")?.chars().collect::<Vec<char>>();

        if chars[0] == '\\' {
			// escape sequence
			if chars.len() >= 2 {
				match chars[1] {
					'"' => Some(Imm::Byte(0x22)), // \"	Double Quote Escape
					'\\' => Some(Imm::Byte(0x5C)), // \\	Backslash Escape
					'0' => Some(Imm::Byte(0x00)), // \0	Null Escape
					'a' => Some(Imm::Byte(0x07)), // \a	Bell/'Alert' Escape
					'b' => Some(Imm::Byte(0x08)), // \b	Backspace Escape
					'e' => Some(Imm::Byte(0x1B)), // \e	Escape Escape (lol)
					'f' => Some(Imm::Byte(0x0C)), // \f	Form Feed Escape
					'n' => Some(Imm::Byte(0x0A)), // \n	Line Feed Escape (newline)
					'r' => Some(Imm::Byte(0x0D)), // \r	Carriage Return Escape
					't' => Some(Imm::Byte(0x09)), // \t	Horizontal Tab Escape
					'v' => Some(Imm::Byte(0x0B)), // \v	Vertical Tab Escape
					'x' => {
						// byte escape sequence
						if chars.len() == 4 {
							let mut vs = String::new();
							vs.push(chars[2]);
							vs.push(chars[3]);

							Some(Imm::Byte(u8::from_str_radix(&vs, 16).ok()?))
						} else { None }
					},
					_ => None // unrecognized escape sequence
				}
			} else { None }
		} else {
			// not an escape sequence
			if chars.len() == 1 {
				Some(Imm::Byte(char_to_byte(chars[0])?))
			} else { None } 
		}
    }

    pub fn dec(l: &mut Lexer<Tok>) -> Option<Imm> {
		let s = l.slice();

        return Imm::dec_from_str(s);
	}

    pub fn dec_from_str(s : &str) -> Option<Imm> {
        if let Ok(b) = u8::from_str(s) {
			Some(Imm::Byte(b))
		} else if let Ok(w) = u16::from_str(s) {
			Some(Imm::Half(w))
		} else if let Ok(w) = u32::from_str(s) {
			Some(Imm::Word(w))
		} else if let Ok(w) = u64::from_str(s) {
			Some(Imm::Float(w))
		} else if let Ok(w) = u128::from_str(s) {
			Some(Imm::Double(w))
		} 
        else { None }
    }

    pub fn hex(l: &mut Lexer<Tok>) -> Option<Imm> {
		let s = l.slice().strip_prefix("0x")?;

		return Imm::hex_from_str(s);
	}

    pub fn hex_from_str(s: &str) -> Option<Imm> {
        if let Ok(b) = u8::from_str_radix(s, 16) {
			Some(Imm::Byte(b))
		} else if let Ok(w) = u16::from_str_radix(s, 16) {
			Some(Imm::Half(w))
		} else if let Ok(w) = u32::from_str_radix(s, 16) {
			Some(Imm::Word(w))
		} else if let Ok(w) = u64::from_str_radix(s, 16) {
			Some(Imm::Float(w))
		} else if let Ok(w) = u128::from_str_radix(s, 16) {
			Some(Imm::Double(w))
		} 
        else { None }
    }

    pub fn oct(l: &mut Lexer<Tok>) -> Option<Imm> {
		let s = l.slice().strip_prefix("0o")?;

		return Imm::oct_from_str(s);
	}

    pub fn oct_from_str(s: &str) -> Option<Imm> {
        if let Ok(b) = u8::from_str_radix(s, 8) {
			Some(Imm::Byte(b))
		} else if let Ok(w) = u16::from_str_radix(s, 8) {
			Some(Imm::Half(w))
		} else if let Ok(w) = u32::from_str_radix(s, 8) {
			Some(Imm::Word(w))
		} else if let Ok(w) = u64::from_str_radix(s, 8) {
			Some(Imm::Float(w))
		} else if let Ok(w) = u128::from_str_radix(s, 8) {
			Some(Imm::Double(w))
		} 
        else { None }
    }

    pub fn bin(l: &mut Lexer<Tok>) -> Option<Imm> {
		let s = l.slice().strip_prefix("0b")?;

		return Imm::bin_from_str(s);
	}

    pub fn bin_from_str(s: &str) -> Option<Imm> {
        if let Ok(b) = u8::from_str_radix(s, 2) {
			Some(Imm::Byte(b))
		} else if let Ok(w) = u16::from_str_radix(s, 2) {
			Some(Imm::Half(w))
		} else if let Ok(w) = u32::from_str_radix(s, 2) {
			Some(Imm::Word(w))
		} else if let Ok(w) = u64::from_str_radix(s, 2) {
			Some(Imm::Float(w))
		} else if let Ok(w) = u128::from_str_radix(s, 2) {
			Some(Imm::Double(w))
		} 
        else { None }
    }
}

impl Token {
    pub fn lex(content: &String) -> TokenResult {
        let mut toks = Vec::new();

        for(token, span) in Tok::lexer(&content).spanned() {
            match token {
                Tok::Error => {
                    // Dont include empty tokens
                },
                _ => {
                    toks.push(
                        Token {
                            contents: token,
                            slice: String::from(&content[span])
                    })
                }
            }
        }
            
        
        return Ok(toks); 
    }
}

impl fmt::Debug for Token {
	fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
		match &self.contents {
            Tok::Word(w) => match w {
                Wrd::Directive(d) => match d {
                    Dir::Text => write!(fmtr,   "[Directive] Text"),
                    Dir::Data => write!(fmtr,   "[Directive] Data"),
                    Dir::Ascii => write!(fmtr,  "[Directive] Ascii"),
                    Dir::Asciiz => write!(fmtr, "[Directive] Asciiz"),
                    Dir::Byte => write!(fmtr,   "[Directive] Byte"),
                    Dir::Half => write!(fmtr,   "[Directive] Half"),
                    Dir::Float => write!(fmtr,  "[Directive] Float"),
                    Dir::Double => write!(fmtr, "[Directive] Double"),
                    Dir::Globl => write!(fmtr,  "[Directive] Globl"),
                    Dir::Space => write!(fmtr,  "[Directive] Space"),
                    Dir::Align => write!(fmtr,  "[Directive] Align"),
                    Dir::Word => write!(fmtr,   "[Directive] Word"),
                },
                Wrd::Instruction(i) => {
                    match i {
                        Inst::AddWord => write!(fmtr, "[Instruction] Add Word"),
                        Inst::AddImmediateWord => write!(fmtr, "[Instruction] Add Immediate Word"),
                        Inst::AddUnsignedWord => write!(fmtr, "[Instruction] Add Unsigned Word"),
                        Inst::AddImmediateUnsignedWord => write!(fmtr, "[Instruction] Add Immediate Unsigned Word"),
                        Inst::SubtractWord => write!(fmtr, "[Instruction] Subtract Word"),
                        Inst::SubtractUnsignedWord => write!(fmtr, "[Instruction] Subtract Unsigned Word"),
                        Inst::MultiplyWordtoGPR => write!(fmtr, "[Instruction] Multiply Word to GPR"),
                        Inst::MultiplyWord => write!(fmtr, "[Instruction] Multiply Word"),
                        Inst::MultiplyUnsignedWord => write!(fmtr, "[Instruction] Multiply Unsigned Word"),
                        Inst::MultiplyandAddWordtoHiLo => write!(fmtr, "[Instruction] Multiply and Add Word to Hi, Lo"),
                        Inst::MultiplyandAddUnsignedWordtoHiLo => write!(fmtr, "[Instruction] Multiply and Add Unsigned Word to Hi, Lo"),
                        Inst::MultiplyandSubtractWordtoHiLo => write!(fmtr, "[Instruction] Multiply and Subtract Word to Hi, Lo"),
                        Inst::MultiplyandSubtractUnsignedWordtoHiLo => write!(fmtr, "[Instruction] Multiply and Subtract Unsigned Word to Hi, Lo"),
                        Inst::DivideWord => write!(fmtr, "[Instruction] Divide Word"),
                        Inst::DivideUnsignedWord => write!(fmtr, "[Instruction] Divide Unsigned Word"),
                        Inst::RemainderafterWordDivision => write!(fmtr, "[Instruction] Remainder after Word Division"),
                        Inst::RemainderafterUnsignedWordDivision => write!(fmtr, "[Instruction] Remainder after Unsigned Word Division"),
                        Inst::CountLeadingOnesinWord => write!(fmtr, "[Instruction] Count Leading Ones in Word"),
                        Inst::CountLeadingZeroesinWord => write!(fmtr, "[Instruction] Count Leading Zeroes in Word"),
                        Inst::SignExtendByte => write!(fmtr, "[Instruction] Sign-Extend Byte"),
                        Inst::SignExtendHalfword => write!(fmtr, "[Instruction] Sign-Extend Halfword"),
                        Inst::SetonEqual => write!(fmtr, "[Instruction] Set on Equal"),
                        Inst::SetonNotEqual => write!(fmtr, "[Instruction] Set on Not Equal"),
                        Inst::SetonLessThanorEqual => write!(fmtr, "[Instruction] Set on Less Than or Equal"),
                        Inst::SetonLessThanorEqualUnsigned => write!(fmtr, "[Instruction] Set on Less Than or Equal Unsigned"),
                        Inst::SetonLessThan => write!(fmtr, "[Instruction] Set on Less Than"),
                        Inst::SetonLessThanUnsigned => write!(fmtr, "[Instruction] Set on Less Than Unsigned"),
                        Inst::SetonGreaterThanorEqual => write!(fmtr, "[Instruction] Set on Greater Than or Equal"),
                        Inst::SetonGreaterThanorEqualUnsigned => write!(fmtr, "[Instruction] Set on Greater Than or Equal Unsigned"),
                        Inst::SetonLessThanImmediate => write!(fmtr, "[Instruction] Set on Less Than Immediate"),
                        Inst::SetonLessThanImmediateUnsigned => write!(fmtr, "[Instruction] Set on Less Than Immediate Unsigned"),
                        Inst::AbsoluteValue => write!(fmtr, "[Instruction] Absolute Value"),
                        Inst::NegateValue => write!(fmtr, "[Instruction] Negate Value"),
                        Inst::NegateValueUnsigned => write!(fmtr, "[Instruction] Negate Value Unsigned"),
                        Inst::And => write!(fmtr, "[Instruction] And"),
                        Inst::AndImmediate => write!(fmtr, "[Instruction] And Immediate"),
                        Inst::Or => write!(fmtr, "[Instruction] Or"),
                        Inst::OrImmediate => write!(fmtr, "[Instruction] Or Immediate"),
                        Inst::NotOr => write!(fmtr, "[Instruction] Not Or"),
                        Inst::ExclusiveOr => write!(fmtr, "[Instruction] Exclusive Or"),
                        Inst::ExclusiveOrImmediate => write!(fmtr, "[Instruction] Exclusive Or Immediate"),
                        Inst::BitwiseNot => write!(fmtr, "[Instruction] Bitwise Not"),
                        Inst::RotateLeft => write!(fmtr, "[Instruction] Rotate Left"),
                        Inst::RotateRight => write!(fmtr, "[Instruction] Rotate Right"),
                        Inst::RotateWordRight => write!(fmtr, "[Instruction] Rotate Word Right"),
                        Inst::RotateWordRightVariable => write!(fmtr, "[Instruction] Rotate Word Right Variable"),
                        Inst::ShiftWordLeftLogical => write!(fmtr, "[Instruction] Shift Word Left Logical"),
                        Inst::ShiftWordLeftLogicalVariable => write!(fmtr, "[Instruction] Shift Word Left Logical Variable"),
                        Inst::ShiftWordRightArithmetic => write!(fmtr, "[Instruction] Shift Word Right Arithmetic"),
                        Inst::ShiftWordRightArithmeticVariable => write!(fmtr, "[Instruction] Shift Word Right Arithmetic Variable"),
                        Inst::ShiftWordRightLogical => write!(fmtr, "[Instruction] Shift Word Right Logical"),
                        Inst::ShiftWordRightLogicalVariable => write!(fmtr, "[Instruction] Shift Word Right Logical Variable"),
                        Inst::LoadImmediate => write!(fmtr, "[Instruction] Load Immediate"),
                        Inst::LoadAddress => write!(fmtr, "[Instruction] Load Address"),
                        Inst::LoadUpperImmediate => write!(fmtr, "[Instruction] Load Upper Immediate"),
                        Inst::LoadByte => write!(fmtr, "[Instruction] Load Byte"),
                        Inst::LoadByteUnsigned => write!(fmtr, "[Instruction] Load Byte Unsigned"),
                        Inst::LoadHalfword => write!(fmtr, "[Instruction] Load Halfword"),
                        Inst::LoadHalfwordUnsigned => write!(fmtr, "[Instruction] Load Halfword Unsigned"),
                        Inst::LoadWord => write!(fmtr, "[Instruction] Load Word"),
                        Inst::LoadDoubleWord => write!(fmtr, "[Instruction] Load Double Word"),
                        Inst::StoreByte => write!(fmtr, "[Instruction] Store Byte"),
                        Inst::StoreHalfword => write!(fmtr, "[Instruction] Store Halfword"),
                        Inst::StoreWord => write!(fmtr, "[Instruction] Store Word"),
                        Inst::StoreDoubleWord => write!(fmtr, "[Instruction] Store Double Word"),
                        Inst::Pushregistertostack => write!(fmtr, "[Instruction] Push register to stack"),
                        Inst::Popregisterfromstack => write!(fmtr, "[Instruction] Pop register from stack"),
                        Inst::Createstackframe => write!(fmtr, "[Instruction] Create stack frame"),
                        Inst::Destroystackframe => write!(fmtr, "[Instruction] Destroy stack frame"),
                        Inst::MoveFromHIRegister => write!(fmtr, "[Instruction] Move From HI Register"),
                        Inst::MoveFromLORegister => write!(fmtr, "[Instruction] Move From LO Register"),
                        Inst::MoveToHIRegister => write!(fmtr, "[Instruction] Move To HI Register"),
                        Inst::MoveToLORegister => write!(fmtr, "[Instruction] Move To LO Register"),
                        Inst::Move => write!(fmtr, "[Instruction] Move"),
                        Inst::MoveConditionalonZero => write!(fmtr, "[Instruction] Move Conditional on Zero"),
                        Inst::MoveConditionalonNotZero => write!(fmtr, "[Instruction] Move Conditional on Not Zero"),
                        Inst::UnconditionalBranch => write!(fmtr, "[Instruction] Unconditional Branch"),
                        Inst::BranchandLink => write!(fmtr, "[Instruction] Branch and Link"),
                        Inst::BranchonEqual => write!(fmtr, "[Instruction] Branch on Equal"),
                        Inst::BranchonEqualtoZero => write!(fmtr, "[Instruction] Branch on Equal to Zero"),
                        Inst::BranchonNotEqual => write!(fmtr, "[Instruction] Branch on Not Equal"),
                        Inst::BranchonNotEqualtoZero => write!(fmtr, "[Instruction] Branch on Not Equal to Zero"),
                        Inst::BranchonGreaterThanorEqual => write!(fmtr, "[Instruction] Branch on Greater Than or Equal"),
                        Inst::BranchonUnsignedGreaterThanorEqual => write!(fmtr, "[Instruction] Branch on Unsigned Greater Than or Equal"),
                        Inst::BranchonGreaterThanorEqualtoZero => write!(fmtr, "[Instruction] Branch on Greater Than or Equal to Zero"),
                        Inst::BranchonGreaterThan => write!(fmtr, "[Instruction] Branch on Greater Than"),
                        Inst::BranchonUnsignedGreaterThan => write!(fmtr, "[Instruction] Branch on Unsigned Greater Than"),
                        Inst::BranchonGreaterThanZero => write!(fmtr, "[Instruction] Branch on Greater Than Zero"),
                        Inst::BranchonLessThan => write!(fmtr, "[Instruction] Branch on Less Than"),
                        Inst::BranchonUnsignedLessThan => write!(fmtr, "[Instruction] Branch on Unsigned Less Than"),
                        Inst::BranchonLessThanZero => write!(fmtr, "[Instruction] Branch on Less Than Zero"),
                        Inst::BranchonLessThanorEqual => write!(fmtr, "[Instruction] Branch on Less Than or Equal"),
                        Inst::BranchonUnsignedLessThanorEqual => write!(fmtr, "[Instruction] Branch on Unsigned Less Than or Equal"),
                        Inst::BranchonLessThanorEqualtoZero => write!(fmtr, "[Instruction] Branch on Less Than or Equal to Zero"),
                        Inst::BranchonGreaterThanorEqualtoZeroandLink => write!(fmtr, "[Instruction] Branch on Greater Than or Equal to Zero and Link"),
                        Inst::BranchonLessThanZeroandLink => write!(fmtr, "[Instruction] Branch on Less Than Zero and Link"),
                        Inst::Jump => write!(fmtr, "[Instruction] Jump"),
                        Inst::JumpandLink => write!(fmtr, "[Instruction] Jump and Link"),
                        Inst::JumpRegister => write!(fmtr, "[Instruction] Jump Register"),
                        Inst::JumpandLinkRegister => write!(fmtr, "[Instruction] Jump and Link Register"),
                        Inst::Syscall => write!(fmtr, "[Instruction] Syscall"),
                    }
                },
                Wrd::Identifier(s) => write!(fmtr, "{} {}", "[Identifier]".red(), s),
            },

            Tok::String(s) => write!(fmtr, "[String] {}", s),

            Tok::Error => Ok(()),
            Tok::Label(s) => write!(fmtr, "[Label] {}", s),
            Tok::Immediate(i) => match i {
				Imm::Byte(b) => write!(fmtr, "[Byte] {} ({})", b, self.slice),
				Imm::Word(b) => write!(fmtr, "[Word] {} ({})", b, self.slice),
                Imm::Half(b) => write!(fmtr, "[Half] {} ({})", b, self.slice),
                Imm::Float(b) => write!(fmtr, "[Float] {} ({})", b, self.slice),
                Imm::Double(b) =>write!(fmtr, "[Double] {} ({})", b, self.slice),
			},

            Tok::Register(s) => write!(fmtr, "[Register] {}", s),
            Tok::Comment(s) => write!(fmtr, "[Comment] {}", s),
            Tok::Operand(o) => match o {
                Op::Add => write!(fmtr, "[Operand] Add"),
                Op::Subtract => write!(fmtr, "[Operand] Subtract"),
            },
            Tok::Address(a) => match a {
                Addr::Register(r) => write!(fmtr, "[Address Register] {}", r),
                Addr::ImmRegister(i, r) => write!(fmtr, "[Address Imm Register] {} + {}", i, r),
                Addr::LabelRegister(l, r) => write!(fmtr, "[Address Label Register] {} + {}", l, r),
            },
        }
    }
}

impl fmt::Display for Token {
	fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
		match &self.contents {
            Tok::Word(w) => match w {
                Wrd::Directive(d) => match d {
                    Dir::Text => write!(fmtr,   ".text"),
                    Dir::Data => write!(fmtr,   ".data"),
                    Dir::Ascii => write!(fmtr,  ".ascii"),
                    Dir::Asciiz => write!(fmtr, ".asciiz"),
                    Dir::Byte => write!(fmtr,   ".byte"),
                    Dir::Half => write!(fmtr,   ".half"),
                    Dir::Float => write!(fmtr,  ".float"),
                    Dir::Double => write!(fmtr, ".double"),
                    Dir::Globl => write!(fmtr,  ".globl"),
                    Dir::Space => write!(fmtr,  ".space"),
                    Dir::Align => write!(fmtr,  ".align"),
                    Dir::Word => write!(fmtr,   ".word"),
                },
                Wrd::Instruction(i) => {
                    match i {
                        Inst::AddWord => write!(fmtr, "add"),
                        Inst::AddImmediateWord => write!(fmtr, "addi"),
                        Inst::AddUnsignedWord => write!(fmtr, "addu"),
                        Inst::AddImmediateUnsignedWord => write!(fmtr, "addiu"),
                        Inst::SubtractWord => write!(fmtr, "sub"),
                        Inst::SubtractUnsignedWord => write!(fmtr, "subu"),
                        Inst::MultiplyWordtoGPR => write!(fmtr, "mul"),
                        Inst::MultiplyWord => write!(fmtr, "mult"),
                        Inst::MultiplyUnsignedWord => write!(fmtr, "multu"),
                        Inst::MultiplyandAddWordtoHiLo => write!(fmtr, "madd"),
                        Inst::MultiplyandAddUnsignedWordtoHiLo => write!(fmtr, "maddu"),
                        Inst::MultiplyandSubtractWordtoHiLo => write!(fmtr, "msub"),
                        Inst::MultiplyandSubtractUnsignedWordtoHiLo => write!(fmtr, "msubu"),
                        Inst::DivideWord => write!(fmtr, "div"),
                        Inst::DivideUnsignedWord => write!(fmtr, "divu"),
                        Inst::RemainderafterWordDivision => write!(fmtr, "rem"),
                        Inst::RemainderafterUnsignedWordDivision => write!(fmtr, "remu"),
                        Inst::CountLeadingOnesinWord => write!(fmtr, "clo"),
                        Inst::CountLeadingZeroesinWord => write!(fmtr, "clz"),
                        Inst::SignExtendByte => write!(fmtr, "seb"),
                        Inst::SignExtendHalfword => write!(fmtr, "seh"),
                        Inst::SetonEqual => write!(fmtr, "seq"),
                        Inst::SetonNotEqual => write!(fmtr, "sne"),
                        Inst::SetonLessThanorEqual => write!(fmtr, "sle"),
                        Inst::SetonLessThanorEqualUnsigned => write!(fmtr, "sleu"),
                        Inst::SetonLessThan => write!(fmtr, "slt"),
                        Inst::SetonLessThanUnsigned => write!(fmtr, "sltu"),
                        Inst::SetonGreaterThanorEqual => write!(fmtr, "sgt"),
                        Inst::SetonGreaterThanorEqualUnsigned => write!(fmtr, "sgtu"),
                        Inst::SetonLessThanImmediate => write!(fmtr, "slti"),
                        Inst::SetonLessThanImmediateUnsigned => write!(fmtr, "sltiu"),
                        Inst::AbsoluteValue => write!(fmtr, "abs"),
                        Inst::NegateValue => write!(fmtr, "neg"),
                        Inst::NegateValueUnsigned => write!(fmtr, "negu"),
                        Inst::And => write!(fmtr, "and"),
                        Inst::AndImmediate => write!(fmtr, "andi"),
                        Inst::Or => write!(fmtr, "or"),
                        Inst::OrImmediate => write!(fmtr, "ori"),
                        Inst::NotOr => write!(fmtr, "nor"),
                        Inst::ExclusiveOr => write!(fmtr, "xor"),
                        Inst::ExclusiveOrImmediate => write!(fmtr, "xori"),
                        Inst::BitwiseNot => write!(fmtr, "not"),
                        Inst::RotateLeft => write!(fmtr, "rol"),
                        Inst::RotateRight => write!(fmtr, "ror"),
                        Inst::RotateWordRight => write!(fmtr, "rotr"),
                        Inst::RotateWordRightVariable => write!(fmtr, "rotrv"),
                        Inst::ShiftWordLeftLogical => write!(fmtr, "sll"),
                        Inst::ShiftWordLeftLogicalVariable => write!(fmtr, "sllv"),
                        Inst::ShiftWordRightArithmetic => write!(fmtr, "sra"),
                        Inst::ShiftWordRightArithmeticVariable => write!(fmtr, "srav"),
                        Inst::ShiftWordRightLogical => write!(fmtr, "srl"),
                        Inst::ShiftWordRightLogicalVariable => write!(fmtr, "srlv"),
                        Inst::LoadImmediate => write!(fmtr, "li"),
                        Inst::LoadAddress => write!(fmtr, "la"),
                        Inst::LoadUpperImmediate => write!(fmtr, "lui"),
                        Inst::LoadByte => write!(fmtr, "lb"),
                        Inst::LoadByteUnsigned => write!(fmtr, "lbu"),
                        Inst::LoadHalfword => write!(fmtr, "lh"),
                        Inst::LoadHalfwordUnsigned => write!(fmtr, "lhu"),
                        Inst::LoadWord => write!(fmtr, "lw"),
                        Inst::LoadDoubleWord => write!(fmtr, "ld"),
                        Inst::StoreByte => write!(fmtr, "sb"),
                        Inst::StoreHalfword => write!(fmtr, "sh"),
                        Inst::StoreWord => write!(fmtr, "sw"),
                        Inst::StoreDoubleWord => write!(fmtr, "sd"),
                        Inst::Pushregistertostack => write!(fmtr, "push"),
                        Inst::Popregisterfromstack => write!(fmtr, "pop"),
                        Inst::Createstackframe => write!(fmtr, "begin"),
                        Inst::Destroystackframe => write!(fmtr, "end"),
                        Inst::MoveFromHIRegister => write!(fmtr, "mfhi"),
                        Inst::MoveFromLORegister => write!(fmtr, "mflo"),
                        Inst::MoveToHIRegister => write!(fmtr, "mthi"),
                        Inst::MoveToLORegister => write!(fmtr, "mtlo"),
                        Inst::Move => write!(fmtr, "move"),
                        Inst::MoveConditionalonZero => write!(fmtr, "movz"),
                        Inst::MoveConditionalonNotZero => write!(fmtr, "movn"),
                        Inst::UnconditionalBranch => write!(fmtr, "b"),
                        Inst::BranchandLink => write!(fmtr, "bal"),
                        Inst::BranchonEqual => write!(fmtr, "beq"),
                        Inst::BranchonEqualtoZero => write!(fmtr, "beqz"),
                        Inst::BranchonNotEqual => write!(fmtr, "bne"),
                        Inst::BranchonNotEqualtoZero => write!(fmtr, "bnez"),
                        Inst::BranchonGreaterThanorEqual => write!(fmtr, "bge"),
                        Inst::BranchonUnsignedGreaterThanorEqual => write!(fmtr, "bgeu"),
                        Inst::BranchonGreaterThanorEqualtoZero => write!(fmtr, "bgez"),
                        Inst::BranchonGreaterThan => write!(fmtr, "bgt"),
                        Inst::BranchonUnsignedGreaterThan => write!(fmtr, "bgtu"),
                        Inst::BranchonGreaterThanZero => write!(fmtr, "bgtz"),
                        Inst::BranchonLessThan => write!(fmtr, "blt"),
                        Inst::BranchonUnsignedLessThan => write!(fmtr, "bltu"),
                        Inst::BranchonLessThanZero => write!(fmtr, "bltz"),
                        Inst::BranchonLessThanorEqual => write!(fmtr, "ble"),
                        Inst::BranchonUnsignedLessThanorEqual => write!(fmtr, "bleu"),
                        Inst::BranchonLessThanorEqualtoZero => write!(fmtr, "blez"),
                        Inst::BranchonGreaterThanorEqualtoZeroandLink => write!(fmtr, "bgezal"),
                        Inst::BranchonLessThanZeroandLink => write!(fmtr, "bltzal"),
                        Inst::Jump => write!(fmtr, "j"),
                        Inst::JumpandLink => write!(fmtr, "jal"),
                        Inst::JumpRegister => write!(fmtr, "jr"),
                        Inst::JumpandLinkRegister => write!(fmtr, "jalr"),
                        Inst::Syscall => write!(fmtr, "syscall"),
                    }
                },
                Wrd::Identifier(s) => write!(fmtr, "{}", s),
            },

            Tok::String(s) => write!(fmtr, "{}", s),

            Tok::Error => Ok(()),
            Tok::Label(s) => write!(fmtr, "{}", s),
            Tok::Immediate(i) => match i {
				Imm::Byte(_) => write!(fmtr, "{}", self.slice),
				Imm::Word(_) => write!(fmtr, "{}", self.slice),
                Imm::Half(_) => write!(fmtr, "{}", self.slice),
                Imm::Float(_) => write!(fmtr, "{}", self.slice),
                Imm::Double(_) => write!(fmtr, "{}", self.slice),
			},

            Tok::Register(s) => write!(fmtr, "{}", s),
            Tok::Comment(s) => write!(fmtr, "{}", s),
            Tok::Operand(o) => match o {
                Op::Add => write!(fmtr, "+"),
                Op::Subtract => write!(fmtr, "-"),
            },
            Tok::Address(a) => match a {
                Addr::Register(r) => write!(fmtr, "({})", r),
                Addr::ImmRegister(i, r) => write!(fmtr, "{}({})", i, r),
                Addr::LabelRegister(l, r) => write!(fmtr, "{}({})", l, r),
            },
        }
    }
}

impl fmt::Display for Imm {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Imm::Byte(b) => write!(fmtr, "{}", b),
            Imm::Word(b) => write!(fmtr, "{}", b),
            Imm::Half(b) => write!(fmtr, "{}", b),
            Imm::Float(b) => write!(fmtr, "{}", b),
            Imm::Double(b) => write!(fmtr, "{}", b),
        }
        
    }
}

impl Tok {
    fn string(l: &mut Lexer<Tok>) -> Option<String> {
        let chars : String = l.slice().trim().to_string();

        if chars.is_empty() {
            return None;
        }

        return Some(chars);
    }
}

fn char_to_byte(c: char) -> Option<u8> {
	if c.is_ascii() {
		let mut buf: [u8; 1] = [0; 1];
		c.encode_utf8(&mut buf);
		Some(buf[0])
	} else { None }
}