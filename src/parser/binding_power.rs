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
        Op::ChapterOf => (13, 14),
        Op::Select => (15, 16),
        Op::Through => (17, 18),
        _ => (0, 0),
    }
}
