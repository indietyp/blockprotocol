//! Implementation of the algorithm described in:
//! https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
//!
//! This has been adapted from
//! https://github.com/segeljakt/pratt/blob/56e194b52d5defd00ef32f6ada9aad98e0346f95/src/lib.rs
//!
//! Our precedence values are multiplied by 10 to make sure we're able to modify and add new ones
//! easily.

use crate::parser::Parser;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub(crate) struct Precedence(pub u32);

impl Precedence {
    const fn raise(mut self) -> Precedence {
        self.0 += 1;
        self
    }

    const fn lower(mut self) -> Precedence {
        self.0 -= 1;
        self
    }

    const fn normalize(mut self) -> Precedence {
        self.0 *= 10;
        self
    }

    const fn min() -> Precedence {
        Precedence(u32::MIN)
    }

    const fn max() -> Precedence {
        Precedence(u32::MAX)
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Associativity {
    Left,
    Right,
    Neither,
}

#[derive(Copy, Clone)]
pub(crate) enum Affix {
    Nilfix,
    Prefix(Precedence),
    Postfix(Precedence),
    Infix(Precedence, Associativity),
}

//         <lbp>  <rbp>  <nbp> <kind>
// Nilfix:  MIN |  MIN |  MAX | nud
// Prefix:  MIN |   bp |  MAX | nud
// Postfix:  bp |  MIN |  MAX | led
// InfixL:   bp |   bp | bp+1 | led
// InfixR:   bp | bp-1 | bp+1 | led
// InfixN:   bp |   bp |   bp | led
impl Affix {
    fn left_binding_power(self) -> Precedence {
        match self {
            Affix::Nilfix => Precedence::min(),
            Affix::Prefix(_) => Precedence::min(),
            Affix::Postfix(bp) => bp.normalize(),
            Affix::Infix(bp, _) => bp.normalize(),
        }
    }

    fn right_binding_power(self) -> Precedence {
        match self {
            Affix::Nilfix => Precedence::min(),
            Affix::Prefix(bp) => bp.normalize(),
            Affix::Postfix(_) => Precedence::min(),
            Affix::Infix(bp, Associativity::Right) => bp.lower(),
            Affix::Infix(bp, _) => bp,
        }
    }

    fn next_binding_power(self) -> Precedence {
        match self {
            Affix::Nilfix => Precedence::max(),
            Affix::Prefix(_) => Precedence::max(),
            Affix::Postfix(_) => Precedence::max(),
            Affix::Infix(bp, Associativity::Left | Associativity::Right) => bp.raise(),
            Affix::Infix(bp, _) => bp,
        }
    }

    fn get(p: &mut Parser) -> Affix {
        match p.current() {
            T![.] if p.at(T![..=]) => Affix::Infix(Precedence(20), Associativity::Left),
            T![.] if p.at(T![..]) => Affix::Infix(Precedence(20), Associativity::Left),

            T![|] if p.at(T![||]) => Affix::Infix(Precedence(30), Associativity::Left),
            T![|] => Affix::Infix(Precedence(60), Associativity::Left),

            T![^] => Affix::Infix(Precedence(70), Associativity::Left),

            T![&] if p.at(T![&&]) => Affix::Infix(Precedence(40), Associativity::Left),
            T![&] => Affix::Infix(Precedence(80), Associativity::Left),

            T![!] => Affix::Prefix(Precedence(45)),

            T![=] if p.at(T![==]) => Affix::Infix(Precedence(50), Associativity::Left),
            T![=] => Affix::Infix(Precedence(10), Associativity::Neither),

            T![>] if p.at(T![>>]) => Affix::Infix(Precedence(90), Associativity::Left),
            T![>] => Affix::Infix(Precedence(50), Associativity::Left),

            T![<] if p.at(T![<<]) => Affix::Infix(Precedence(9), Associativity::Left),
            T![<] => Affix::Infix(Precedence(50), Associativity::Left),

            T![+] => Affix::Infix(Precedence(100), Associativity::Left),
            T![-] => Affix::Infix(Precedence(100), Associativity::Left),

            T![%] => Affix::Infix(Precedence(110), Associativity::Left),

            T![*] if p.at(T![**]) => Affix::Infix(Precedence(120), Associativity::Right),

            T![*] => Affix::Infix(Precedence(110), Associativity::Left),
            T![/] => Affix::Infix(Precedence(110), Associativity::Left),

            _ => Affix::Nilfix,
        }
    }
}
