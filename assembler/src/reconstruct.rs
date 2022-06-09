// Reconstruct from token stream

use crate::definitions::*;
use crate::lexer::*;

pub fn reconstruct(tokens: Vec<Token>) -> Result<String, String> {
    let mut output: String = String::new();
    let mut iter = tokens.iter();

    let mut t = iter.next().expect("No data stream");

    loop {
        let str_form = &t.to_string();
        let mut consumed_extra = false;
        // println!("{:?}", t);
        match &t.contents {
            Tok::Comment(_) => {
                output.push_str(str_form);
                output.push('\n');
            }
            Tok::String(_) => output.push_str(str_form),
            Tok::Label(_) => {
                output.push('\n');
                output.push_str(str_form);
                output.push(' ');
            }
            Tok::Word(w) => match w {
                Wrd::Directive(d) => match d {
                    Dir::Text | Dir::Data => {
                        output.push('\n');
                        output.push_str(str_form);
                        output.push_str(" \n");
                    }
                    _ => {
                        output.push_str(str_form);
                        output.push(' ');
                    }
                },
                Wrd::Instruction(i) => {
                    let args = get_instruction(i);

                    output.push_str(str_form);

                    let mut cur_arg = &args.0;
                    let mut index = 0;
                    let mut token = iter.next().expect("Reach EOF before next token");

                    loop {
                        if matches!(cur_arg, InstructionArgType::Empty) {
                            if !is_token_allowed_after_instruction(token) {
                                return Err(format!(
                                    "Instruction {} did not expect parameter {}\nExpected {:?}, {:?}, {:?}",
                                    t,
                                    token,
                                    &args.0,
                                    &args.1,
                                    &args.2
                                ));
                            }

                            consumed_extra = true;
                            break;
                        }
                        if is_token_arg(token, cur_arg) {
                            if index > 0 {
                                output.push(',');
                            }
                            output.push(' ');
                            // println!("{} {:?}, {}", index, cur_arg, token);
                            output.push_str(&token.to_string());
                        } else {
                            return Err(format!(
                                "Argument {} of {} Expected {:?} ({} Provided)",
                                index, t, cur_arg, token
                            ));
                        }
                        match index {
                            0 => {
                                index = 1;
                                cur_arg = &args.1;
                                match iter.next() {
                                    Some(x) => token = x,
                                    None => return Err("Reach EOF before next token".to_string()),
                                }
                            }
                            1 => {
                                index = 2;
                                cur_arg = &args.2;
                                match iter.next() {
                                    Some(x) => token = x,
                                    None => return Err("Reach EOF before next token".to_string()),
                                }
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    t = token;

                    output.push('\n');
                }
                Wrd::Identifier(_) => output.push_str(str_form),
            },
            _ => (), //Rest should already be handled
        }

        if !consumed_extra {
            match iter.next() {
                Some(x) => t = x,
                None => break,
            }
        }
    }

    Ok(output)
}
