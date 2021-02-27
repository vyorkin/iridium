use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    Number { value: i32 },
}
