#![allow(unused)]

use super::operator::Op;

#[repr(u8)]
pub enum BindingPower {
    Minimum,
    Book,
    And,
    Following,
}

pub fn postfix_binding_power(op: Op) -> u8 {
    match op {
        Op::Following => 20,
        Op::PartOf => 21,
        _ => panic!("bad postfix operator {op}"),
    }
}

pub fn prefix_binding_power(op: Op) -> u8 {
    match op {
        // Op::BookOf => 1,
        _ => panic!("bad prefix operator {op}"),
    }
}

pub fn infix_binding_power(op: Op) -> (u8, u8) {
    match op {
        Op::BookOf => (9, 10),
        Op::And => (11, 12),
        Op::Select => (13, 14),
        Op::Through => (15, 16),
        Op::ChapterOf => (17, 18),
        _ => (0, 0),
    }
}
