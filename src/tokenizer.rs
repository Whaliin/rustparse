use crate::types::*;

#[derive(Debug, PartialEq)]
enum LexState {
    None,
    Number,
    Word,
}

fn push_state(tokens: &mut Vec<Token>, slice: &str, state: LexState) -> Result<(), String> {
	match state {
		LexState::Number => {
			if let Ok(n) = slice.parse::<f64>() {
				tokens.push(Token::Number(n));
				return Ok(());
			}
		}
		LexState::Word => match slice {
			"e" => tokens.push(Token::Const(ConstType::E)),
			"pi" => tokens.push(Token::Const(ConstType::Pi)),
			"tau" => tokens.push(Token::Const(ConstType::Tau)),
			"sin" => tokens.push(Token::Func(FuncType::Sin)),
			"cos" => tokens.push(Token::Func(FuncType::Cos)),
			"tan" => tokens.push(Token::Func(FuncType::Tan)),
			"abs" => tokens.push(Token::Func(FuncType::Abs)),
			"log" => tokens.push(Token::Func(FuncType::Log)),
			"max" => tokens.push(Token::Func(FuncType::Max)),
			"min" => tokens.push(Token::Func(FuncType::Min)),
			"pow" => tokens.push(Token::Func(FuncType::Pow)),
			"sqrt" => tokens.push(Token::Func(FuncType::Sqrt)),
			"asin" => tokens.push(Token::Func(FuncType::Asin)),
			"acos" => tokens.push(Token::Func(FuncType::Acos)),
			"atan" => tokens.push(Token::Func(FuncType::Atan)),
			"ceil" => tokens.push(Token::Func(FuncType::Ceil)),
			"goldr" => tokens.push(Token::Const(ConstType::GoldR)),
			"floor" => tokens.push(Token::Func(FuncType::Floor)),
			"round" => tokens.push(Token::Func(FuncType::Round)),
			"clamp" => tokens.push(Token::Func(FuncType::Clamp)),
			_ => tokens.push(Token::Variable(slice.into())),
		},
		LexState::None => { /* Do nothing when the state is None. */ } // _ => return Err(format!("Invalid token state on slice: {state:?}"))
	}

	Ok(())
}

// Add implicit mul
// can_multiply is type matched to be a function that takes borrowed token and returns bool
// shorthand instead of adding type generics/matching
fn impl_mul(tokens: &mut Vec<Token>, can_multiply: impl Fn(&Token) -> bool) {
	// get the last token (if any)
	if let Some(last) = tokens.last() {
		// if it matches any of the types passed, push mul op
		if can_multiply(last) {
			tokens.push(Token::Operator(OpType::Mul))
		}
	}
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
	let mut tokens: Vec<Token> = Vec::new();
	let bytes = input.as_bytes();
	let mut i = 0;
	let mut state = LexState::None;
	let mut slice_start: usize = 0;

	while i < bytes.len() {
		let b = bytes[i];

		// If we entered a word already, and it's still alphanumeric (a..z1..9) or _, 
		// consider it as a variable name still
		if state == LexState::Word && ( b.is_ascii_alphanumeric() || b == b'_') {
			// we don't need to do anything
		} else if b.is_ascii_alphabetic() {
			// flush state if we are not already lexing a word
			if state != LexState::Word {
				push_state(&mut tokens, &input[slice_start..i], state)?;
				// add implicit mul if last is a number or closing paren
				impl_mul(&mut tokens, |t| matches!(t, 
					Token::Number(_) |
					Token::RPar
				));
				
				state = LexState::Word;
				slice_start = i;

			}
		} else if b.is_ascii_digit() || b == b'.' {
			if state != LexState::Number {
				push_state(&mut tokens, &input[slice_start..i], state)?;

				// add implicit mul if the last operator is closing paren
				impl_mul(&mut tokens, |t| matches!(t, Token::RPar));

				state = LexState::Number;
				slice_start = i;
			}
		} else {
			if state != LexState::None {
				push_state(&mut tokens, &input[slice_start..i], state)?;
				state = LexState::None;
				slice_start = i;
			}
			// single char tokens can be pushed directly, but must still flush the state.
			match b {
				b'^' => tokens.push(Token::Operator(OpType::Exp)),
				b'*' => tokens.push(Token::Operator(OpType::Mul)),
				b'/' => tokens.push(Token::Operator(OpType::Div)),
				b'+' => tokens.push(Token::Operator(OpType::Add)),
				b'-' => tokens.push(Token::Operator(OpType::Sub)),
				b'%' => tokens.push(Token::Operator(OpType::Mod)),
				b'!' => tokens.push(Token::Operator(OpType::Fac)),
				b'(' => {
					// add implicit mul on starting parentheses for all below types
					impl_mul(&mut tokens, |t|  
						matches!(t, 
						Token::Number(_) |
						Token::RPar |
						Token::Variable(_) |
						Token::Const(_)
					));

					tokens.push(Token::LPar)
				}
				b')' => tokens.push(Token::RPar),
				b',' => tokens.push(Token::Comma),
				b'=' => tokens.push(Token::Operator(OpType::Assignment)),
				_ => { /* Skip token */ }
			}
		}
		i += 1;
	}

	// run the final push
	if state != LexState::None {
		push_state(&mut tokens, &input[slice_start..i], state)?;
	}

	return Ok(tokens);
}