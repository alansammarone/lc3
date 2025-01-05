#![allow(dead_code, unused_mut, unused_imports, unused_variables)]
use std::error::Error;
use std::fs;
use std::process;

const MEMORY_SIZE: usize = 1 << 16; // 65536
const PC_START: usize = 0x3000; // Initial value of the PC register

mod types;

use types::{ConditionFlag, Register};

pub struct Config {
    query: String,
    image_filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: lc3 <image_filepath>");
        }

        let query = args[0].clone();
        let image_filepath = args[1].clone();

        Ok(Config {
            query,
            image_filepath,
        })
    }
}

fn initialize_registers(registers: &mut [u16]) {
    // println!("{:?}", Register::COND)

    // Exactly one condition flag should be set at any given time
    // TODO - do we need "as u16" as opposed to "u8"?
    // Answer: yes - LC3 words are 16bits. "as u16" is still ugly tho
    // TODO is converting enum variants to integers the best way to do this?
    registers[Register::COND as usize] = ConditionFlag::ZRO as u16;

    // Set the PC (program counter) register (0x3000 is the default) */
    // TODO Getting real tired "my_shit as othershit"
    // there must be a cleverer way
    registers[Register::PC as usize] = PC_START as u16
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let image_contents = fs::read_to_string(config.image_filepath)?;

    // println!("Number of registers: {:?}", Register::COUNT);

    // let mut memory: [u16; MEMORY_SIZE] = [0; MEMORY_SIZE];
    // let mut registers: [u16; Register::COUNT] = [0; Register::COUNT];

    // initialize_registers(&mut registers);

    // println!("Register values: {:?}", registers);

    // for register in Register::iter() {
    //     println!("{:?}", register);
    // }

    // load_image_file(&mut memory, )

    Ok(())
}
