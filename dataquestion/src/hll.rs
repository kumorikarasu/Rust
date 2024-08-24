// HLL implementation to learn it.
//
// This was really fun to implement, however it is SLOW AF
// Actual algorithms use floating point bit hacks and other optimizations
// The problem with those is that they are hard to understand

use ahash::AHasher;
use std::hash::{Hash, Hasher};

pub struct HyperLogLog {
    // u8 register size should allow up to 1.8e19 unique values
    registers: Vec<u8>,
    bits: u32,
    alpha: f64
}

// Constants
const MAX_REGISTER_SIZE: usize = 1 << 16;
const TWO_32: f64 = (1_u64 << 32) as f64;


impl HyperLogLog {
    pub fn new(register_size: usize) -> HyperLogLog {
        // convert to power of 2
        let mut register_size = 1 << 1 + (register_size.ilog2() as u32);


        if register_size < 16 {
            register_size = 16;
        }else if register_size > MAX_REGISTER_SIZE {
            register_size = MAX_REGISTER_SIZE;
        }
        
        // println!("Error Rate: {:#.2}%", (1.04 / (register_size as f64).sqrt() * 100.0));

        return HyperLogLog {
            registers: vec![0; register_size],
            bits: register_size.ilog2(),
            alpha: HyperLogLog::alpha(register_size as usize)

        }
    }

    pub fn insert(&mut self, value: &u64) {
        let mut hasher = AHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();

        // get the first 'bits' bits of the hash
        let index = hash >> (64 - self.bits);

        // get the number of leading zeroes in the rest of the hash
        let leading_zeroes = (hash << self.bits).leading_zeros() as u8 + 1;

        // get the number of leading zeroes in the rest of the hash
        let current_zeroes = self.registers[index as usize];

        // update the register
        self.registers[index as usize] = std::cmp::max(current_zeroes, leading_zeroes);
    }

    pub fn len(&self) -> f64{
        self.harmonic_mean_with_corrections()
    }

    fn alpha(length: usize) -> f64{
        // Magic numbers, no idea what they really mean
        match length {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            _ => 0.7213 / (1.0 + 1.079 / length as f64)
        }
    }


    fn harmonic_mean_with_corrections(&self) -> f64 {
        let mut sum = 0.0;
        let m = self.registers.len() as f64;

        for register in self.registers.iter(){
            sum += 2.0_f64.powf(-(*register as f64));
        }

        let raw = self.alpha * m.powi(2) / sum;

        //apply correction
        //Also magic
        match raw {
            sr if raw <= 2.5_f64 * m => {
                // get how many empty registers we have
                let empty_registers = self.registers.iter().filter(|&x| *x == 0).count();
                if empty_registers != 0 {
                    m * (m / empty_registers as f64).ln()
                } else {
                    sr
                }
            }
            mr if raw <= TWO_32 / 30.0 => mr,
            _ => -TWO_32 * (1.0 - raw / TWO_32).ln()
        }
    }
}

