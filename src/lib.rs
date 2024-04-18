use evm_disassembler::{disassemble_str, Opcode, Operation};
use std::fmt::{Debug, Formatter};

pub struct Op {
    pub opcode: Opcode,
    pub input: Vec<u8>,
    pub offset: u32,
}

impl Op {
    pub fn new(operation: &Operation) -> Self {
        Op {
            opcode: operation.opcode,
            offset: operation.offset,
            input: operation.input.clone(),
        }
    }

    pub fn decrement_offset(&mut self) {
        self.offset -= 1;
    }

    pub fn decrement_input(&mut self) {
        let mut be_bytes: [u8; 4] = [0; 4];
        let start = 4 - self.input.len();
        for (i, input_byte) in self.input.iter().enumerate() {
            be_bytes[start + i] = *input_byte;
        }
        let value = u32::from_be_bytes(be_bytes) - 1;
        let new_bytes = value.to_be_bytes();
        let mut new_input: Vec<u8> = vec![];
        for i in start..4 {
            new_input.push(new_bytes[i]);
        }

        self.input = new_input;
    }

    pub fn is_jump(&self) -> bool {
        self.opcode == Opcode::JUMP || self.opcode == Opcode::JUMPI
    }

    pub fn as_string(&self) -> String {
        let mut string = String::new();
        string.push_str(format!("{:02x}", assemble_opcode(self.opcode)).as_str());
        for byte in &self.input {
            string.push_str(format!("{:02x}", byte).as_str());
        }
        string
    }

    pub fn assemble(&self) -> Vec<u8> {
        let mut assembled = vec![assemble_opcode(self.opcode)];
        for byte in &self.input {
            assembled.push(*byte);
        }
        assembled
    }

    pub fn demote_push(&mut self) {
        if self.opcode != Opcode::PUSH2 {
            panic!("Trying to demote an op that is not a PUSH2");
        }
        self.opcode = Opcode::PUSH1;
        self.input.remove(0);
    }

    pub fn get_input_value(&self) -> u32 {
        let mut be_bytes: [u8; 4] = [0; 4];
        let start = 4 - self.input.len();
        for (i, input_byte) in self.input.iter().enumerate() {
            be_bytes[start + i] = *input_byte;
        }
        u32::from_be_bytes(be_bytes)
    }
}

impl Debug for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

fn assemble_opcode(op: Opcode) -> u8 {
    match op {
        Opcode::STOP => 0x00,
        Opcode::ADD => 0x01,
        Opcode::MUL => 0x02,
        Opcode::SUB => 0x03,
        Opcode::DIV => 0x04,
        Opcode::SDIV => 0x05,
        Opcode::MOD => 0x06,
        Opcode::SMOD => 0x07,
        Opcode::ADDMOD => 0x08,
        Opcode::MULMOD => 0x09,
        Opcode::EXP => 0x0a,
        Opcode::SIGNEXTEND => 0x0b,
        Opcode::LT => 0x10,
        Opcode::GT => 0x11,
        Opcode::SLT => 0x12,
        Opcode::SGT => 0x13,
        Opcode::EQ => 0x14,
        Opcode::ISZERO => 0x15,
        Opcode::AND => 0x16,
        Opcode::OR => 0x17,
        Opcode::XOR => 0x18,
        Opcode::NOT => 0x19,
        Opcode::BYTE => 0x1a,
        Opcode::SHL => 0x1b,
        Opcode::SHR => 0x1c,
        Opcode::SAR => 0x1d,
        Opcode::SHA3 => 0x20,
        Opcode::ADDRESS => 0x30,
        Opcode::BALANCE => 0x31,
        Opcode::ORIGIN => 0x32,
        Opcode::CALLER => 0x33,
        Opcode::CALLVALUE => 0x34,
        Opcode::CALLDATALOAD => 0x35,
        Opcode::CALLDATASIZE => 0x36,
        Opcode::CALLDATACOPY => 0x37,
        Opcode::CODESIZE => 0x38,
        Opcode::CODECOPY => 0x39,
        Opcode::GASPRICE => 0x3a,
        Opcode::EXTCODESIZE => 0x3b,
        Opcode::EXTCODECOPY => 0x3c,
        Opcode::RETURNDATASIZE => 0x3d,
        Opcode::RETURNDATACOPY => 0x3e,
        Opcode::EXTCODEHASH => 0x3f,
        Opcode::BLOCKHASH => 0x40,
        Opcode::COINBASE => 0x41,
        Opcode::TIMESTAMP => 0x42,
        Opcode::NUMBER => 0x43,
        Opcode::DIFFICULTY => 0x44,
        Opcode::GASLIMIT => 0x45,
        Opcode::CHAINID => 0x46,
        Opcode::SELFBALANCE => 0x47,
        Opcode::BASEFEE => 0x48,
        Opcode::BLOBHASH => 0x49,
        Opcode::BLOBBASEFEE => 0x4a,
        Opcode::POP => 0x50,
        Opcode::MLOAD => 0x51,
        Opcode::MSTORE => 0x52,
        Opcode::MSTORE8 => 0x53,
        Opcode::SLOAD => 0x54,
        Opcode::SSTORE => 0x55,
        Opcode::JUMP => 0x56,
        Opcode::JUMPI => 0x57,
        Opcode::PC => 0x58,
        Opcode::MSIZE => 0x59,
        Opcode::GAS => 0x5a,
        Opcode::JUMPDEST => 0x5b,
        Opcode::TLOAD => 0x5c,
        Opcode::TSTORE => 0x5d,
        Opcode::MCOPY => 0x5e,
        Opcode::PUSH0 => 0x5f,
        Opcode::PUSH1 => 0x60,
        Opcode::PUSH2 => 0x61,
        Opcode::PUSH3 => 0x62,
        Opcode::PUSH4 => 0x63,
        Opcode::PUSH5 => 0x64,
        Opcode::PUSH6 => 0x65,
        Opcode::PUSH7 => 0x66,
        Opcode::PUSH8 => 0x67,
        Opcode::PUSH9 => 0x68,
        Opcode::PUSH10 => 0x69,
        Opcode::PUSH11 => 0x6a,
        Opcode::PUSH12 => 0x6b,
        Opcode::PUSH13 => 0x6c,
        Opcode::PUSH14 => 0x6d,
        Opcode::PUSH15 => 0x6e,
        Opcode::PUSH16 => 0x6f,
        Opcode::PUSH17 => 0x70,
        Opcode::PUSH18 => 0x71,
        Opcode::PUSH19 => 0x72,
        Opcode::PUSH20 => 0x73,
        Opcode::PUSH21 => 0x74,
        Opcode::PUSH22 => 0x75,
        Opcode::PUSH23 => 0x76,
        Opcode::PUSH24 => 0x77,
        Opcode::PUSH25 => 0x78,
        Opcode::PUSH26 => 0x79,
        Opcode::PUSH27 => 0x7a,
        Opcode::PUSH28 => 0x7b,
        Opcode::PUSH29 => 0x7c,
        Opcode::PUSH30 => 0x7d,
        Opcode::PUSH31 => 0x7e,
        Opcode::PUSH32 => 0x7f,
        Opcode::DUP1 => 0x80,
        Opcode::DUP2 => 0x81,
        Opcode::DUP3 => 0x82,
        Opcode::DUP4 => 0x83,
        Opcode::DUP5 => 0x84,
        Opcode::DUP6 => 0x85,
        Opcode::DUP7 => 0x86,
        Opcode::DUP8 => 0x87,
        Opcode::DUP9 => 0x88,
        Opcode::DUP10 => 0x89,
        Opcode::DUP11 => 0x8a,
        Opcode::DUP12 => 0x8b,
        Opcode::DUP13 => 0x8c,
        Opcode::DUP14 => 0x8d,
        Opcode::DUP15 => 0x8e,
        Opcode::DUP16 => 0x8f,
        Opcode::SWAP1 => 0x90,
        Opcode::SWAP2 => 0x91,
        Opcode::SWAP3 => 0x92,
        Opcode::SWAP4 => 0x93,
        Opcode::SWAP5 => 0x94,
        Opcode::SWAP6 => 0x95,
        Opcode::SWAP7 => 0x96,
        Opcode::SWAP8 => 0x97,
        Opcode::SWAP9 => 0x98,
        Opcode::SWAP10 => 0x99,
        Opcode::SWAP11 => 0x9a,
        Opcode::SWAP12 => 0x9b,
        Opcode::SWAP13 => 0x9c,
        Opcode::SWAP14 => 0x9d,
        Opcode::SWAP15 => 0x9e,
        Opcode::SWAP16 => 0x9f,
        Opcode::LOG0 => 0xa0,
        Opcode::LOG1 => 0xa1,
        Opcode::LOG2 => 0xa2,
        Opcode::LOG3 => 0xa3,
        Opcode::LOG4 => 0xa4,
        Opcode::CREATE => 0xf0,
        Opcode::CALL => 0xf1,
        Opcode::CALLCODE => 0xf2,
        Opcode::RETURN => 0xf3,
        Opcode::DELEGATECALL => 0xf4,
        Opcode::CREATE2 => 0xf5,
        Opcode::STATICCALL => 0xfa,
        Opcode::REVERT => 0xfd,
        Opcode::INVALID => 0xfe,
        Opcode::SELFDESTRUCT => 0xff,
    }
}

fn assemble_operation(op: Op) -> Vec<u8> {
    let mut assembled = vec![assemble_opcode(op.opcode)];
    for byte in op.input {
        assembled.push(byte);
    }
    assembled
}

pub fn bunny_hop(bytecode_string: &str) -> String {
        disassemble_str(bytecode_string).expect("Failed to disassemble input string");
    let mut ops: Vec<Op> = disassemble_str(bytecode_string)
        .expect("Failed to disassemble input string")
        .iter()
        .map(Op::new)
        .collect();
    let mut target_index: Option<usize> = None;
    loop {
        for (index, op) in ops.iter().enumerate() {
            if index == 0 {
                continue;
            }

            if op.is_jump() {
                let prev_op = ops.get(index - 1).unwrap();
                let is_prev_bunny_hoppable =
                    prev_op.opcode == Opcode::PUSH2 && prev_op.input[0] == 0;
                if is_prev_bunny_hoppable {
                    target_index = Some(index - 1);
                    break;
                }
            }
        }
        if target_index.is_none() {
            break;
        }
        let index = target_index.unwrap();
        let jump2_operation = ops.get_mut(index).unwrap();
        jump2_operation.demote_push();

        let mut decremented_jumpdests: Vec<u32> = vec![];
        for i in (index + 2)..ops.len() {
            {
                let op = ops.get(i).unwrap();
                if op.opcode == Opcode::JUMPDEST {
                    decremented_jumpdests.push(op.offset);
                }
            }
            ops.get_mut(i).unwrap().decrement_offset();
        }

        if decremented_jumpdests.len() > 0 {
            for i in 0..(ops.len() - 1) {
                {
                    let next_op = ops.get(i + 1).unwrap();
                    if !next_op.is_jump() {
                        continue;
                    }
                }
                let op = ops.get_mut(i).unwrap();
                if op.opcode == Opcode::PUSH1 || op.opcode == Opcode::PUSH2 {
                    if decremented_jumpdests.contains(&op.get_input_value()) {
                        op.decrement_input();
                    }
                }
            }
        }

        target_index = None;
    }
    let mut output = String::new();
    for op in ops {
        let assembled = assemble_operation(op);
        for byte in assembled {
            output.push_str(format!("{:02x}", byte).as_str());
        }
    }
    output
}
