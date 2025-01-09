use std::fmt;
use rand::Rng;

#[derive(Debug)]

pub struct ProceduralGrid(Box<Expr>);

impl fmt::Display for ProceduralGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ProceduralGrid {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let depth = rng.gen_range(3..=8);
        let expr = Expr::generate(&mut rng, depth);

        ProceduralGrid(expr)
    }

    pub fn calc(&self, x: i32, y: i32) -> i32 {
        self.0.eval(x, y)
    }
}

#[derive(Debug)]
pub enum Expr {
    X,
    Y,
    Num(i32),
    Binary(BinaryOp, Box<Expr>, Box<Expr>)
}

impl Expr {
    fn generate(rng: &mut impl Rng, depth: u32) -> Box<Expr> {
        let expr = if depth == 0 {
            match rng.gen_range(0..3) {
                0 => Expr::X,
                1 => Expr::Y,
                2 => Expr::Num(rng.gen_range(-64..=64)),
                _ => unreachable!(),
            }
        } else {
            match rng.gen_range(0..10) {
                0 => Expr::X,
                1 => Expr::Y,
                2..=5 => Expr::Num(rng.gen_range(-64..=64)),
                // we want binary operators to be more common
                _ => {
                    let left = Expr::generate(rng,
                        depth - 1);
                    let right  = Expr::generate(rng,
                            depth - 1);
                    let op = match rng.gen_range(0..20) {
                        0 => BinaryOp::Add,
                        1 => BinaryOp::Sub,
                        2..=5 =>   BinaryOp::And,
                        6..=9 =>   BinaryOp::Or,
                        10..=14 => BinaryOp::Xor,
                        15..=19 => BinaryOp::Mod,
                        _ => unreachable!(),
                    };
                    Expr::Binary(op, left, right)
                }
            }
        };

        Box::new(expr)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::X => write!(f, "x"),
            Expr::Y => write!(f, "y"),
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Binary(op, left, right) => write!(f, "({left} {op} {right})"),
            // Expr::Unary(op, expr) => {
            //     if matches!(op, UnaryOp::Negate | UnaryOp::BitwiseNot) {
            //         write!(f, "{}{}", op, expr)
            //     } else {
            //         write!(f, "{}({})", op, expr)
            //     }
            // },
        }
    }
}

#[derive(Debug)]
enum BinaryOp {
    Add,
    Sub,
    /// Modulo
    Mod,
    /// Bitwise AND
    And,
    /// Bitwise OR
    Or,
    /// Bitwise XOR
    Xor,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            // BinaryOp::Mul => "*",
            // BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::And => "&",
            BinaryOp::Or  => "|",
            BinaryOp::Xor => "^",
        };

        write!(f, "{repr}")
    }
}

impl Expr {
    pub fn eval(&self, x: i32, y: i32) -> i32 {
        match self {
            Expr::X => x,
            Expr::Y => y,
            Expr::Num(a) => *a,
            Expr::Binary(op, left, right) => {

                let left = left.eval(x, y);
                let right = right.eval(x, y);

                match op {
                    BinaryOp::Add => left + right,
                    BinaryOp::Sub => left - right,
                    BinaryOp::And => left & right,
                    BinaryOp::Or => left | right,
                    BinaryOp::Xor => left ^ right,
                    BinaryOp::Mod => left % right,
                }
            }
        }
    }
}
