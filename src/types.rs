use strum_macros::{EnumCount, EnumIter};

#[derive(EnumIter, EnumCount, Debug)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
}

pub enum Instruction {
    BR = 0, // Branch
    ADD,    // Add
    LD,     // Load
    ST,     // Store
    JSR,    // Jump register
    AND,    // Bitwise and
    LDR,    // Load register
    STR,    // Store register
    RTI,    // Unused
    NOT,    // Bitwise not
    LDI,    // Load indirect
    STI,    // Store indirect
    JMP,    // Jump
    RES,    // Reserved (unused)
    LEA,    // Load effective address
    TRAP,   // Execute trap
}

pub enum ConditionFlag {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}
