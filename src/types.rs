use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Exp,
    Mul,
    Div,
	Mod,
    Add,
    Sub,
	Fac,
	Assignment
}

#[derive(Debug, PartialEq, Clone)]
pub enum FuncType {
    // Simple functions
	Sin,
    Cos,
    Tan,
	Sqrt,
	Abs,
	Log,
	Asin,
	Acos,
	Atan,
	Floor,
	Ceil,
	Round,
	// Multi argument functions
	Max,
	Min,
	Pow,
	Clamp
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConstType {
    Pi,
    E,
    GoldR,
	Tau
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Operator(OpType),
    Func(FuncType),
    Const(ConstType),
	Variable(Rc<str>),
    LPar,
    RPar,
	Comma
}