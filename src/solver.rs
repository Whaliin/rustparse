use crate::types::*;
use crate::tokenizer::tokenize;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SolverOptions {
	pub trig_deg: bool
}

impl Default for SolverOptions {
	fn default() -> Self {
		SolverOptions { 
			trig_deg: true 
		}
	}
}

pub struct Solver {
    tokens: Vec<Token>,
    pos: usize,
	variables: HashMap<Rc<str>, f64>,
	pub options: SolverOptions
}

fn factorial(n: f64) -> Result<f64, String> {
    if n < 0.0 || n.fract() != 0.0 {
        return Err("Factorial is only defined for non-negative integers".into());
    }
    let int_n = n as u64;
    // (1..=int_n).product() multiplies 1 * 2 * ... * n:
    // (0! naturally returns 1)
    let result: u64 = (1..=int_n).product();
    Ok(result as f64)
}

impl Default for Solver {
	fn default() -> Self {
		Solver::new(SolverOptions::default())
	}
}

impl Solver {
    pub fn solve(&mut self, input: &str) -> Result<f64, String> {
		self.tokens = tokenize(input)?;
		self.pos = 0;

		// solve the name into an option
		let name_o = if let [Token::Variable(name_t), Token::Operator(OpType::Assignment), ..] = &self.tokens[..] {
			// hop over the first two tokens
			self.pos = 2;
			
			Some(name_t.clone())
		} else {
			None			
		};

		let result = self.parse_expression()?;

		// save the variable if the option can be resolved
		if let Some(name) = name_o {
			self.variables.insert(name, result);
		}

		// check that we consumed all tokens
		if self.pos < self.tokens.len() {
			return Err(format!("Unexpected token: {:?}", self.tokens[self.pos]))
		}
		
        Ok(result)
    }

    pub fn new(options: SolverOptions) -> Self {
        return Solver {
            tokens: Vec::new(),
            pos: 0,
			variables: HashMap::new(),
			options: options
        };
    }

	pub fn variables(&self) -> &HashMap<Rc<str>, f64> {
		&self.variables
	}

	pub fn clear_variables(&mut self) {
		self.variables.clear();
	}

	pub fn set_variable(&mut self, name: &str, val: f64) {
		self.variables.insert(name.into(), val);
	}

    fn peek(&self) -> Option<&Token> {
        return self.tokens.get(self.pos);
    }

    fn next_token(&mut self) -> Option<Token> {
        let val: Option<Token> = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        return val;
    }

	fn solve_trig(&self, func: FuncType, x: f64) -> Result<f64, String> {
		// if true, we want the result in degrees.
		let deg: bool = self.options.trig_deg;

		// radian/angle conversion
		let rad: f64 = if deg { x.to_radians() } else { x };
		
		// closure solving angle calc
		let to_angle = |val: f64| if deg { val.to_degrees() } else { val };

		let res: f64 = match func {
			FuncType::Sin 	=> rad.sin(),
			FuncType::Cos 	=> rad.cos(),
			FuncType::Tan 	=> rad.tan(),
			FuncType::Asin 	=> to_angle(x.asin()),
			FuncType::Acos 	=> to_angle(x.acos()),
			FuncType::Atan 	=> to_angle(x.atan()),
			_ => return Err(format!("Not a trig function: {func:?}"))
		};

		Ok(res)
	}

    fn parse_expression(&mut self) -> Result<f64, String> {
        // add / sub
        // read the first token
        let mut left: f64 = self.parse_term()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Operator(OpType::Add) => {
                    // Calculate the output
                    self.next_token();
                    let right: f64 = self.parse_term()?;
                    left = left + right;
                }
                Token::Operator(OpType::Sub) => {
                    self.next_token();
                    let right: f64 = self.parse_term()?;
                    left = left - right;
                }
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        // mult / div
        let mut left: f64 = self.parse_exponential()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Operator(OpType::Mul) => {
                    // Calculate the output
                    self.next_token();
                    let right: f64 = self.parse_exponential()?;
                    left = left * right;
                }
                Token::Operator(OpType::Div) => {
                    self.next_token();
                    let right: f64 = self.parse_exponential()?;
                    left = left / right;
                }
				Token::Operator(OpType::Mod) => {
					self.next_token();
					let right: f64 = self.parse_exponential()?;
					left = left % right;
				}
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    fn parse_exponential(&mut self) -> Result<f64, String> {
        let left = self.parse_factor()?;

        // check if the next token is an exponent
        if let Some(Token::Operator(OpType::Exp)) = self.peek() {
            self.next_token();

            // recurse for nested ^
            let right = self.parse_exponential()?;

            return Ok(left.powf(right));
        }

        // if not, just return the result of pfactor
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        let mut val = self.parse_primary()?;

		while let Some(token) = self.peek() {
			match token {
				Token::Operator(OpType::Fac) => {
					self.next_token();
					val = factorial(val)?;
				},
				_ => {break;}
			}
		}

		Ok(val)
    }

	fn parse_primary(&mut self) -> Result<f64, String> {
		// number + parentheses
        if let Some(token) = self.next_token() {
            match token {
                Token::Number(num) => Ok(num),
				Token::Variable(name) => match self.variables.get(&name) {
					Some(val) => Ok(*val),
					None => Err(format!("Undefined variable '{name}'"))
				}
                Token::LPar => {
                    // resolve the container
                    let val = self.parse_expression()?;

                    // consume the closing parentheses
                    let next = self.next_token();
                    if next != Some(Token::RPar) {
                        return Err("Token after is not closing parentheses".into());
                    }

                    Ok(val)
                }
                // Handle leading neg sign on numbers
                Token::Operator(OpType::Sub) => {
					// since the operator is consumed, just parse the incoming value
                    let val: f64 = self.parse_factor()?;

                    return Ok(-val);
                }
				// Handle leading add sign on numbers
				Token::Operator(OpType::Add) => self.parse_factor(),
                Token::Func(f) => {
					// resolve the arguments passed to the function
					let args = self.parse_args()?;

					// Early match on min and max for vararg support (neat!)
					match f {
						FuncType::Min => {
							if args.len() < 2 { return Err("min requires at least 2 arguments".into()); }
							return Ok(args.iter().copied().reduce(f64::min).unwrap())
						}
						FuncType::Max => {
							if args.len() < 2 { return Err("max requires at least 2 arguments".into()); }
							return Ok(args.iter().copied().reduce(f64::max).unwrap())
						}
						_ => {}
					}

					// match on arg count (WTF?!)
					match &args[..] {
						[x] => match f {
							FuncType::Sin | FuncType::Cos | FuncType::Tan | 
							FuncType::Asin | FuncType::Acos | FuncType::Atan 
											=> self.solve_trig(f, *x),
							FuncType::Sqrt 	=> Ok(x.sqrt()),
							FuncType::Abs 	=> Ok(x.abs()),
							FuncType::Log 	=> Ok(x.log10()),
							FuncType::Floor => Ok(x.floor()),
							FuncType::Ceil 	=> Ok(x.ceil()),
							FuncType::Round => Ok(x.round()),
							_ => Err(format!("{f:?} does not take 1 argument")),
						},
						[a, b] => match f {
							FuncType::Pow => Ok(a.powf(*b)),
                        	_ => Err(format!("{f:?} does not take 2 arguments")),
						},
						[a, b, c] => match f {
							FuncType::Clamp => Ok(a.clamp(*b, *c)),
							_ => Err(format!("{f:?} does not take 3 arguments")),
						}
						_ => Err(format!("No function takes {} arguments", args.len())),
					}
                }
                Token::Const(c) => match c {
					ConstType::Pi => Ok(std::f64::consts::PI),
					ConstType::E => Ok(std::f64::consts::E),
					ConstType::GoldR => Ok(std::f64::consts::GOLDEN_RATIO),
					ConstType::Tau => Ok(std::f64::consts::TAU)
                }
                _ => {
                    return Err(format!("Unknown token in pfactor {token:?}"));
                }
            }
        } else {
            // Invalid or unknown value. Return 0.
            return Err("No tokens".into());
        }
	}

	fn parse_args(&mut self) -> Result<Vec<f64>, String> {
		let mut out: Vec<f64> = Vec::new();

		// Consume the next token, and ensure it is an '('
		let start = self.next_token();
		if start != Some(Token::LPar) {
			return Err("Function does not begin with opening parentheses".into());
		}
		
		while let Some(token) = self.peek() {
			match token {
				Token::Comma => { self.next_token(); },
				Token::RPar => {
					self.next_token();
					return Ok(out);
				},
				_ => {
					let val = self.parse_expression()?;
					out.push(val);
				}
			}
		}

		// if we reach here, we did not reach a ')'
		return Err("Expected ')' at end of function arguments".into());
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    // --- 1. Basic Arithmetic & PEMDAS ---
    #[test]
    fn test_pemdas_order() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(solver.solve("2 * 3 + 4").unwrap(), 10.0);
        assert_eq!(solver.solve("20 - 4 / 2").unwrap(), 18.0);
    }

    #[test]
    fn test_left_associativity() {
        let mut solver = Solver::default();
        // 100 - 20 - 10 - 5 must be ((100 - 20) - 10) - 5 = 65
        assert_eq!(solver.solve("100 - 20 - 10 - 5").unwrap(), 65.0);
        // 48 / 4 / 2 must be (48 / 4) / 2 = 6
        assert_eq!(solver.solve("48 / 4 / 2").unwrap(), 6.0);
    }

    #[test]
    fn test_modulo() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("10 % 3").unwrap(), 1.0);
        assert_eq!(solver.solve("2 + 10 % 3").unwrap(), 3.0);
        assert_eq!(solver.solve("20 % 6 % 4").unwrap(), 2.0);
    }

    // --- 2. Exponents (Right-Associative) ---
    #[test]
    fn test_exponent_precedence_and_associativity() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("2 ^ 3").unwrap(), 8.0);
        assert_eq!(solver.solve("2 * 3 ^ 2").unwrap(), 18.0);
        // 2 ^ 3 ^ 2 must be 2 ^ (3 ^ 2) = 2 ^ 9 = 512, NOT (2 ^ 3) ^ 2 = 64
        assert_eq!(solver.solve("2 ^ 3 ^ 2").unwrap(), 512.0);
    }

    // --- 3. Unary Operators ---
    #[test]
    fn test_unary_minus_and_plus() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("-5 + 10").unwrap(), 5.0);
        assert_eq!(solver.solve("10 * -2").unwrap(), -20.0);
        assert_eq!(solver.solve("-(3 + 2)").unwrap(), -5.0);
        assert_eq!(solver.solve("- - -5").unwrap(), -5.0);
        assert_eq!(solver.solve("+5 + +3").unwrap(), 8.0);
    }

    // --- 4. Parentheses & Grouping ---
    #[test]
    fn test_nested_parentheses() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("(2 + 3) * (4 + 5)").unwrap(), 45.0);
        assert_eq!(solver.solve("((1.5 + 2.5) * 2) / (5 - 3)").unwrap(), 4.0);
    }

    // --- 5. Implicit Multiplication ---
    #[test]
    fn test_implicit_multiplication() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("2(3)").unwrap(), 6.0);
        assert_eq!(solver.solve("(1 + 2)(3 + 4)").unwrap(), 21.0);
        assert_eq!(solver.solve("(2 + 3)4").unwrap(), 20.0);
        assert!(approx_eq(solver.solve("2pi").unwrap(), 2.0 * std::f64::consts::PI));
        assert_eq!(solver.solve("2sin(90)").unwrap(), 2.0);
    }

    // --- 6. Postfix Factorial ---
    #[test]
    fn test_factorial() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("0!").unwrap(), 1.0);
        assert_eq!(solver.solve("4!").unwrap(), 24.0);
        assert_eq!(solver.solve("5!").unwrap(), 120.0);
        // Chained factorial: (3!)! = 6! = 720
        assert_eq!(solver.solve("3!!").unwrap(), 720.0);
        assert_eq!(solver.solve("(2 + 1)!").unwrap(), 6.0);
        assert_eq!(solver.solve("4! / 2").unwrap(), 12.0);
    }

    // --- 7. Single-Argument Functions & Constants ---
    #[test]
    fn test_single_arg_functions_and_constants() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("sqrt(16)").unwrap(), 4.0);
        assert_eq!(solver.solve("abs(-42)").unwrap(), 42.0);
        assert_eq!(solver.solve("log(100)").unwrap(), 2.0);
        assert_eq!(solver.solve("floor(3.9)").unwrap(), 3.0);
        assert_eq!(solver.solve("ceil(3.1)").unwrap(), 4.0);
        assert_eq!(solver.solve("round(3.5)").unwrap(), 4.0);
        assert!(approx_eq(solver.solve("pi").unwrap(), std::f64::consts::PI));
        assert!(approx_eq(solver.solve("e").unwrap(), std::f64::consts::E));
        assert!(approx_eq(solver.solve("tau").unwrap(), std::f64::consts::TAU));
        assert!(approx_eq(solver.solve("goldr").unwrap(), std::f64::consts::GOLDEN_RATIO));
    }

    // --- 8. Trigonometry Modes ---
    #[test]
    fn test_trig_degrees_mode() {
        let mut solver = Solver::default();
        solver.options.trig_deg = true;
        assert!(approx_eq(solver.solve("sin(90)").unwrap(), 1.0));
        assert!(approx_eq(solver.solve("cos(0)").unwrap(), 1.0));
        assert!(approx_eq(solver.solve("tan(45)").unwrap(), 1.0));
        assert!(approx_eq(solver.solve("asin(1)").unwrap(), 90.0));
        assert!(approx_eq(solver.solve("acos(1)").unwrap(), 0.0));
        assert!(approx_eq(solver.solve("atan(1)").unwrap(), 45.0));
    }

    #[test]
    fn test_trig_radians_mode() {
        let mut solver = Solver::default();
        solver.options.trig_deg = false;
        assert!(approx_eq(solver.solve("sin(pi / 2)").unwrap(), 1.0));
        assert!(approx_eq(solver.solve("cos(0)").unwrap(), 1.0));
        assert!(approx_eq(solver.solve("tan(pi / 4)").unwrap(), 1.0));
        assert!(approx_eq(solver.solve("asin(1)").unwrap(), std::f64::consts::FRAC_PI_2));
    }

    // --- 9. Multi-Argument & Variadic Functions ---
    #[test]
    fn test_multi_arg_and_variadic() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("pow(2, 8)").unwrap(), 256.0);
        assert_eq!(solver.solve("clamp(15, 0, 10)").unwrap(), 10.0);
        assert_eq!(solver.solve("clamp(-5, 0, 10)").unwrap(), 0.0);
        assert_eq!(solver.solve("clamp(5, 0, 10)").unwrap(), 5.0);
        assert_eq!(solver.solve("min(5, 2, 8, 1, 9)").unwrap(), 1.0);
        assert_eq!(solver.solve("max(5, 2, 8, 1, 9)").unwrap(), 9.0);
    }

    // --- 10. Variable Assignment & Persistence ---
    #[test]
    fn test_variables() {
        let mut solver = Solver::default();
        assert_eq!(solver.solve("x = 42").unwrap(), 42.0);
        assert_eq!(solver.solve("x + 8").unwrap(), 50.0);
        // Self-reassignment
        assert_eq!(solver.solve("x = x + 1").unwrap(), 43.0);
        // Multi-variable formula
        solver.solve("a = 3").unwrap();
        solver.solve("b = 4").unwrap();
        assert_eq!(solver.solve("sqrt(a^2 + b^2)").unwrap(), 5.0);
    }

    // --- 11. Error Handling & Edge Cases ---
    #[test]
    fn test_syntax_errors() {
        let mut solver = Solver::default();
        // Unclosed parentheses
        assert!(solver.solve("(2 + 3").is_err());
        // Unexpected closing parenthesis
        assert!(solver.solve("2 + 3)").is_err());
        // Undefined variable
        assert!(solver.solve("unknown_var + 1").is_err());
        // Incorrect argument count
        assert!(solver.solve("pow(2)").is_err());
        assert!(solver.solve("clamp(1, 2)").is_err());
        // Invalid factorials
        assert!(solver.solve("(-3)!").is_err());
        assert!(solver.solve("(3.5)!").is_err());
        // Trailing unrecognized tokens
        assert!(solver.solve("1 + 2 3").is_err());
    }
}

