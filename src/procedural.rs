use rand;
use rand::Rng;
pub struct ProceduralGrid(Box<Expr>);

impl ProceduralGrid {
    fn new() -> Self    {
        let mut rng = rand::thread_rng();
        let depth = rng.gen_range(3..=8);
        let expr = Expr::generate(&mut rng, depth);
        ProceduralGrid(expr)
    }
}

enum Expr   {
    X,
    Y,
    Num(i32),
    Binary(BinaryOp, Box<Expr>, Box<Expr>)
}

impl Expr   {

    pub fn generate(rng: &mut impl Rng, depth: u32) -> Box<Expr> {
        let expr = if depth == 0    {
                match rng.gen_range(0..3) {
                    0 => Expr::X,
                    1 => Expr::Y,
                    2 => Expr::Num(rng.gen_range(-64..=64)),
                };
            } else  {
                match rng.gen_range(0..10) {
                    0 => Expr::X,
                    1 => Expr::Y,
                    2..=5 => Expr::Num(rng.gen_range(-64..=64)),
                    _ => {
                        
                        Expr::Binary(op, left, right)
                    }
                };
            };

        Box::new(expr)
    }

    pub fn eval(&self, x: i32, y: i32) -> i32    {
        match self  {
            Expr::X => x,
            Expr::Y => y,
            Expr::Num(a) => *a,
            Expr::Binary(op, left, right) => {
                let left  = left.eval(x, y);
                let right  = right.eval(x, y);
                match op    {
                    BinaryOp::Add => left + right,
                    BinaryOp::Sub => left - right,
                    BinaryOp::And => left & right,
                    BinaryOp::Or => left | right,
                    BinaryOp::Xor => left ^ right,
                }
            }
        }
    }
}

enum BinaryOp    {
    Add,
    Sub,
    /// Bitwise
    And,
    Or,
    Xor
}
