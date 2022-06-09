use crate::lexer::*;

pub type Register = String;
pub type Label = String;

pub enum Imm {
    Byte(u8),
    Half(u16),
    Word(u32),
    Float(u64),
    Double(u128),
}

pub enum Wrd {
    Directive(Dir),
    Instruction(Inst),
    Identifier(String),
}

pub enum Inst {
    AddWord,
    AddImmediateWord,
    AddUnsignedWord,
    AddImmediateUnsignedWord,
    SubtractWord,
    SubtractUnsignedWord,
    MultiplyWordtoGPR,
    MultiplyWord,
    MultiplyUnsignedWord,
    MultiplyandAddWordtoHiLo,
    MultiplyandAddUnsignedWordtoHiLo,
    MultiplyandSubtractWordtoHiLo,
    MultiplyandSubtractUnsignedWordtoHiLo,
    DivideWord,
    DivideUnsignedWord,
    RemainderafterWordDivision,
    RemainderafterUnsignedWordDivision,
    CountLeadingOnesinWord,
    CountLeadingZeroesinWord,
    SignExtendByte,
    SignExtendHalfword,
    SetonEqual,
    SetonNotEqual,
    SetonLessThanorEqual,
    SetonLessThanorEqualUnsigned,
    SetonLessThan,
    SetonLessThanUnsigned,
    SetonGreaterThanorEqual,
    SetonGreaterThanorEqualUnsigned,
    SetonLessThanImmediate,
    SetonLessThanImmediateUnsigned,
    AbsoluteValue,
    NegateValue,
    NegateValueUnsigned,
    And,
    AndImmediate,
    Or,
    OrImmediate,
    NotOr,
    ExclusiveOr,
    ExclusiveOrImmediate,
    BitwiseNot,
    RotateLeft,
    RotateRight,
    RotateWordRight,
    RotateWordRightVariable,
    ShiftWordLeftLogical,
    ShiftWordLeftLogicalVariable,
    ShiftWordRightArithmetic,
    ShiftWordRightArithmeticVariable,
    ShiftWordRightLogical,
    ShiftWordRightLogicalVariable,
    LoadImmediate,
    LoadAddress,
    LoadUpperImmediate,
    LoadByte,
    LoadByteUnsigned,
    LoadHalfword,
    LoadHalfwordUnsigned,
    LoadWord,
    LoadDoubleWord,
    StoreByte,
    StoreHalfword,
    StoreWord,
    StoreDoubleWord,
    Pushregistertostack,
    Popregisterfromstack,
    Createstackframe,
    Destroystackframe,
    MoveFromHIRegister,
    MoveFromLORegister,
    MoveToHIRegister,
    MoveToLORegister,
    Move,
    MoveConditionalonZero,
    MoveConditionalonNotZero,
    UnconditionalBranch,
    BranchandLink,
    BranchonEqual,
    BranchonEqualtoZero,
    BranchonNotEqual,
    BranchonNotEqualtoZero,
    BranchonGreaterThanorEqual,
    BranchonUnsignedGreaterThanorEqual,
    BranchonGreaterThanorEqualtoZero,
    BranchonGreaterThan,
    BranchonUnsignedGreaterThan,
    BranchonGreaterThanZero,
    BranchonLessThan,
    BranchonUnsignedLessThan,
    BranchonLessThanZero,
    BranchonLessThanorEqual,
    BranchonUnsignedLessThanorEqual,
    BranchonLessThanorEqualtoZero,
    BranchonGreaterThanorEqualtoZeroandLink,
    BranchonLessThanZeroandLink,
    Jump,
    JumpandLink,
    JumpRegister,
    JumpandLinkRegister,
    Syscall,
}

pub enum Addr {
    Register(Register),
    ImmRegister(Imm, Register),
    LabelRegister(Label, Register),
}

pub enum Op {
    Add,
    Subtract,
}

pub enum Dir {
    Text,
    Data,
    Ascii,
    Asciiz,
    Byte,
    Half,
    Float,
    Double,
    Globl,
    Space,
    Align,
    Word,
}

pub type Instruction = (InstructionArgType, InstructionArgType, InstructionArgType);

pub enum InstructionArgType {
    Register,
    Empty,
    Immediate,
    Address,
    Offset,
    Label,
}

impl std::fmt::Debug for InstructionArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionArgType::Register => write!(f, "Register"),
            InstructionArgType::Empty => write!(f, "Empty"),
            InstructionArgType::Immediate => write!(f, "Immediate"),
            InstructionArgType::Address => write!(f, "Address"),
            InstructionArgType::Offset => write!(f, "Offset"),
            InstructionArgType::Label => write!(f, "Label"),
        }
    }
}

pub fn is_token_arg(t: &Token, arg: &InstructionArgType) -> bool {
    let tok = &t.contents;
    match tok {
        Tok::Comment(_) => false,
        Tok::String(_) => false,
        Tok::Operand(_) => false,
        Tok::Address(a) => match a {
            Addr::Register(_) => matches!(
                arg,
                InstructionArgType::Register | InstructionArgType::Address
            ),
            Addr::ImmRegister(_, _) => matches!(
                arg,
                InstructionArgType::Register
                    | InstructionArgType::Offset
                    | InstructionArgType::Address
            ),
            Addr::LabelRegister(_, _) => matches!(
                arg,
                InstructionArgType::Register
                    | InstructionArgType::Offset
                    | InstructionArgType::Address
            ),
        },
        Tok::Label(_) => matches!(arg, InstructionArgType::Label | InstructionArgType::Address),
        Tok::Register(_) => matches!(arg, InstructionArgType::Register),
        Tok::Immediate(_) => matches!(arg, InstructionArgType::Immediate),
        Tok::Word(w) => match w {
            Wrd::Directive(_) => false,
            Wrd::Instruction(_) => false,
            Wrd::Identifier(_) => matches!(arg, InstructionArgType::Label),
        },
        Tok::Error => false,
    }
}

pub fn is_token_allowed_after_instruction(t: &Token) -> bool {
    let tok = &t.contents;
    match tok {
        Tok::Comment(_) => true,
        Tok::String(_) => false,
        Tok::Operand(_) => false,
        Tok::Address(_a) => false,
        Tok::Label(_) => true,
        Tok::Register(_) => false,
        Tok::Immediate(_) => false,
        Tok::Word(w) => match w {
            Wrd::Directive(_) => true,
            Wrd::Instruction(_) => true,
            Wrd::Identifier(_) => false,
        },
        Tok::Error => true,
    }
}

pub fn get_instruction(i: &Inst) -> Instruction {
    match i {
        Inst::AddWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::AddImmediateWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::AddUnsignedWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::AddImmediateUnsignedWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::SubtractWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SubtractUnsignedWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::MultiplyWordtoGPR => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::MultiplyWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::MultiplyUnsignedWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::MultiplyandAddWordtoHiLo => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::MultiplyandAddUnsignedWordtoHiLo => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::MultiplyandSubtractWordtoHiLo => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::MultiplyandSubtractUnsignedWordtoHiLo => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::DivideWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::DivideUnsignedWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::RemainderafterWordDivision => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::RemainderafterUnsignedWordDivision => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::CountLeadingOnesinWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::CountLeadingZeroesinWord => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::SignExtendByte => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::SignExtendHalfword => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::SetonEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SetonNotEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SetonLessThanorEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SetonLessThanorEqualUnsigned => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SetonLessThan => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SetonLessThanUnsigned => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SetonGreaterThanorEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SetonGreaterThanorEqualUnsigned => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::SetonLessThanImmediate => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::SetonLessThanImmediateUnsigned => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::AbsoluteValue => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::NegateValue => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::NegateValueUnsigned => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::And => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::AndImmediate => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::Or => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::OrImmediate => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::NotOr => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::ExclusiveOr => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::ExclusiveOrImmediate => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::BitwiseNot => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::RotateLeft => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::RotateRight => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::RotateWordRight => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::RotateWordRightVariable => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::ShiftWordLeftLogical => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::ShiftWordLeftLogicalVariable => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::ShiftWordRightArithmetic => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::ShiftWordRightArithmeticVariable => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::ShiftWordRightLogical => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Immediate,
        ),
        Inst::ShiftWordRightLogicalVariable => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::LoadImmediate => (
            InstructionArgType::Register,
            InstructionArgType::Immediate,
            InstructionArgType::Empty,
        ),
        Inst::LoadAddress => (
            InstructionArgType::Register,
            InstructionArgType::Label,
            InstructionArgType::Empty,
        ),
        Inst::LoadUpperImmediate => (
            InstructionArgType::Register,
            InstructionArgType::Immediate,
            InstructionArgType::Empty,
        ),
        Inst::LoadByte => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::LoadByteUnsigned => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::LoadHalfword => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::LoadHalfwordUnsigned => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::LoadWord => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::LoadDoubleWord => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::StoreByte => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::StoreHalfword => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::StoreWord => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::StoreDoubleWord => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::Pushregistertostack => (
            InstructionArgType::Register,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::Popregisterfromstack => (
            InstructionArgType::Register,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::Createstackframe => (
            InstructionArgType::Empty,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::Destroystackframe => (
            InstructionArgType::Empty,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::MoveFromHIRegister => (
            InstructionArgType::Register,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::MoveFromLORegister => (
            InstructionArgType::Register,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::MoveToHIRegister => (
            InstructionArgType::Register,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::MoveToLORegister => (
            InstructionArgType::Register,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::Move => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Empty,
        ),
        Inst::MoveConditionalonZero => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::MoveConditionalonNotZero => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Register,
        ),
        Inst::UnconditionalBranch => (
            InstructionArgType::Offset,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::BranchandLink => (
            InstructionArgType::Offset,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::BranchonEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonEqualtoZero => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::BranchonNotEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonNotEqualtoZero => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::BranchonGreaterThanorEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonUnsignedGreaterThanorEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonGreaterThanorEqualtoZero => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::BranchonGreaterThan => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonUnsignedGreaterThan => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonGreaterThanZero => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::BranchonLessThan => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonUnsignedLessThan => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonLessThanZero => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::BranchonLessThanorEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonUnsignedLessThanorEqual => (
            InstructionArgType::Register,
            InstructionArgType::Register,
            InstructionArgType::Offset,
        ),
        Inst::BranchonLessThanorEqualtoZero => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::BranchonGreaterThanorEqualtoZeroandLink => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::BranchonLessThanZeroandLink => (
            InstructionArgType::Register,
            InstructionArgType::Offset,
            InstructionArgType::Empty,
        ),
        Inst::Jump => (
            InstructionArgType::Address,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::JumpandLink => (
            InstructionArgType::Address,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::JumpRegister => (
            InstructionArgType::Register,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::JumpandLinkRegister => (
            InstructionArgType::Register,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
        Inst::Syscall => (
            InstructionArgType::Empty,
            InstructionArgType::Empty,
            InstructionArgType::Empty,
        ),
    }
}
