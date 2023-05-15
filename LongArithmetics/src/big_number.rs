use std::cmp;
use hex;
use std::collections::VecDeque;
use hex::FromHexError;
use std::cmp::Ordering;
use byteorder::{BigEndian, ByteOrder};

pub struct BigNumber {
    pub parts: VecDeque<u64>,
}

impl Clone for BigNumber {
    fn clone(&self) -> Self {
        let mut clone: BigNumber = BigNumber::new();
        for i in 0..self.parts.len() {
            clone.parts.push_back(self.parts[i]);
        }
        clone
    }
}

impl BigNumber {
    pub fn new() -> BigNumber {
        BigNumber {
            parts: VecDeque::new()
        }
    }

    pub fn set_hex(&mut self, number: &str) -> Result<(), FromHexError> {
        self.parts = VecDeque::new();
        let result = match hex::decode(number.chars().collect::<String>()) {
            Ok(data) => {
                let mut i: i32 = i32::try_from(data.len()).unwrap();
                while i > 0 {
                    let lower_bound = usize::try_from(cmp::max(0, i32::try_from(i).unwrap() - 8)).unwrap();
                    let upper_bound = usize::try_from(i).unwrap();
                    let byte_slice: [u8; 8] = self.pad_with_zeros(&data[lower_bound..upper_bound]);
                    let part_of_number: u64 = BigEndian::read_u64(&byte_slice[..]);
                    self.parts.push_front(part_of_number);
                    i -= 8;
                }
                Ok(())
            }
            Err(error) => {
                println!("Error {}", error);
                Err(error)
            }
        };
        result
    }

    fn pad_with_zeros<'a>(&'a self, arr: &'a[u8]) -> [u8; 8] {
        if arr.len() == 8 {
            let mut result: [u8; 8] = [0; 8];
            result.copy_from_slice(arr);
            result
        } else {
            let mut result: [u8; 8] = [0; 8];
            let pad_end: usize = 8 - arr.len();
            let mut data_index = 0;
            while pad_end + data_index < 8 {
                result[pad_end + data_index] = arr[data_index];
                data_index += 1;
            }
            result
        }
    }

    pub fn get_hex(&self) -> String {
        if self.parts.len() > 0 {
            let mut result: String = String::new();
            for part in self.parts.iter().rev() {
                // println!("{part}");
                let mut eight_bit_chunks: [u8; 8] = bytemuck::cast(*part);
                for chunk in eight_bit_chunks {
                    result = format!("{}{}", hex::encode(&[chunk]), result);
                }
            }
            result
        } else {
            String::from("0")
        }
    }

    pub fn bit_invert(&mut self) -> () {
        for i in 0..self.parts.len() {
            self.parts[i] = !(self.parts[i]);
        }
    }

    pub fn and(&self, other: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new();
        let max: &BigNumber;
        let min: &BigNumber;

        if self.parts.len() >= other.parts.len() {
            max = self;
            min = other;
        } else {
            max = other;
            min = self;
        }
        let mut max_part: u64;
        let mut min_part: u64;
        let mut i: i32 = 0;
        let min_length: i32 = min.parts.len().try_into().unwrap();
        let max_length: i32 = max.parts.len().try_into().unwrap();
        while i < max_length  {
            if (min_length - i - 1) < 0 {
                min_part = 0;
            } else {
                min_part = min.parts[(min_length - i - 1).try_into().unwrap()];
            }
            max_part = max.parts[(max_length - i - 1).try_into().unwrap()];
            result.parts.push_front(min_part & max_part);
            i += 1;
        }
        result
    }

    pub fn or(&self, other: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new();
        let max: &BigNumber;
        let min: &BigNumber;

        if self.parts.len() >= other.parts.len() {
            max = self;
            min = other;
        } else {
            max = other;
            min = self;
        }
        let mut max_part: u64;
        let mut min_part: u64;
        let mut i: i32 = 0;
        let min_length: i32 = min.parts.len().try_into().unwrap();
        let max_length: i32 = max.parts.len().try_into().unwrap();
        while i < max_length  {
            if (min_length - i - 1) < 0 {
                min_part = 0;
            } else {
                min_part = min.parts[(min_length - i - 1).try_into().unwrap()];
            }
            max_part = max.parts[(max_length - i - 1).try_into().unwrap()];
            result.parts.push_front(min_part | max_part);
            i += 1;
        }
        result
    }

    pub fn xor(&self, other: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new();
        let max: &BigNumber;
        let min: &BigNumber;

        if self.parts.len() >= other.parts.len() {
            max = self;
            min = other;
        } else {
            max = other;
            min = self;
        }
        let mut max_part: u64;
        let mut min_part: u64;
        let mut i: i32 = 0;
        let min_length: i32 = min.parts.len().try_into().unwrap();
        let max_length: i32 = max.parts.len().try_into().unwrap();
        while i < max_length {
            if (min_length - i - 1) < 0 {
                min_part = 0;
            } else {
                min_part = min.parts[(min_length - i - 1).try_into().unwrap()];
            }
            max_part = max.parts[(max_length - i - 1).try_into().unwrap()];
            result.parts.push_front(min_part ^ max_part);
            i += 1;
        }
        result
    }

    pub fn bitshift_right(&mut self, shift: u32) -> () {
        let chunk_shift = shift / 64;
        for _ in 0..chunk_shift {
            self.parts.pop_back();
        }
        let bit_shift_per_chunk = shift % 64;
        let mut carry: u64 = 0;
        let mut next_carry: u64 = 0;
        for i in 0..self.parts.len() {
            next_carry = self.parts[i] & (u64::pow(2, bit_shift_per_chunk) - 1);
            self.parts[i] >>= bit_shift_per_chunk;
            if carry != 0 {
                self.parts[i] |= carry << (64 - bit_shift_per_chunk);
            }
            carry = next_carry;
        }
        if self.parts[0] == 0 {
            self.parts.pop_front();
        }
    }

    pub fn bitshift_left(&mut self, shift: u32) -> () {
        let chunk_shift = shift / 64;
        for _ in 0..chunk_shift {
            self.parts.push_back(0);
        }
        let bit_shift_per_chunk = shift % 64;
        let mut leading_zeros_in_first_chunk = 0;
        let mut tmp: u64 = u64::pow(2, 63);
        while self.parts[0] & tmp == 0 {
            leading_zeros_in_first_chunk += 1;
            tmp >>= 1;
        }
        if leading_zeros_in_first_chunk < bit_shift_per_chunk {
            self.parts.push_front(0);
        }
        let mut carry: u64 = 0;
        let mut next_carry: u64 = 0;
        for i in (0..self.parts.len()).rev() {
            next_carry = self.parts[i] & (u64::MAX - (u64::pow(2, bit_shift_per_chunk + 1) - 1));
            self.parts[i] <<= bit_shift_per_chunk;
            if carry != 0 {
                self.parts[i] |= carry >> (64 - bit_shift_per_chunk);
            }
            carry = next_carry;
        }
    }

    pub fn add(&self, other: &BigNumber) -> BigNumber {
        let mut result: BigNumber = BigNumber::new();
        let max: &BigNumber;
        let min: &BigNumber;

        if self.parts.len() >= other.parts.len() {
            max = self;
            min = other;
        } else {
            max = other;
            min = self;
        }

        for _ in 0..max.parts.len() {
            result.parts.push_front(0);
        }

        let mut max_part: u64;
        let mut min_part: u64;

        let mut i: usize = 0;
        let mut total: u128;
        let mut carry: u128 = 0;


        while i < max.parts.len() {
            if (
                    <usize as TryInto<i32>>::try_into(min.parts.len()).unwrap() -
                    <usize as TryInto<i32>>::try_into(i).unwrap() - 1
            ) < 0 {
                min_part = 0;
            } else {
                min_part = min.parts[min.parts.len() - i - 1];
            }
            max_part = max.parts[max.parts.len() - i - 1];
            total = (
                    <u64 as TryInto<u128>>::try_into(min_part).unwrap() +
                    <u64 as TryInto<u128>>::try_into(max_part).unwrap() +
                    carry
            );
            if total > u64::MAX.try_into().unwrap() {
                carry = total / u128::pow(2, 64);
                total = total % u128::pow(2, 64);
            } else {
                carry = 0;
            }
            result.parts[max.parts.len() - i - 1] = total.try_into().unwrap();
            i += 1;
        }
        if carry > 0 {
            result.parts.push_front(carry.try_into().unwrap());
        }
        result
    }

    pub fn sub(&self, other: &BigNumber) -> BigNumber {
        // println!("This: {}; Other: {};", self.get_hex(), other.get_hex());
        assert!(other.less_or_eq(self));

        let mut result: BigNumber = BigNumber::new();

        let mut max_part: u64;
        let mut min_part: u64;

        let mut i: usize = 0;

        // suppose BigNumber chunk holds 0 as its value. Subtracting 1 from it will result in underflow :(
        // need an array that will store "debt" of each chunk
        let mut debt: Vec<u128> = Vec::with_capacity(self.parts.len() + 1); // probably would be more efficient with a simple u128, but don't have time to try this
        debt.resize(self.parts.len() + 1, 0);
        while i < self.parts.len() {
            if
                <usize as TryInto<i32>>::try_into(other.parts.len()).unwrap() -
                <usize as TryInto<i32>>::try_into(i).unwrap() - 1 < 0
            {
                min_part = 0;
            } else {
                min_part = other.parts[other.parts.len() - i - 1];
            }
            max_part = self.parts[self.parts.len() - i - 1];

            if max_part < min_part {
                debt[i + 1] += 1;
                result.parts.push_front(
                    <u128 as TryInto<u64>>::try_into(
                    u128::pow(2, 64) -
                        <u64 as TryInto<u128>>::try_into(min_part).unwrap() +
                        <u64 as TryInto<u128>>::try_into(max_part).unwrap() -
                        debt[i]
                    ).unwrap()
                ); // guaranteed not to overflow
            } else {
                result.parts.push_front(max_part - min_part - <u128 as TryInto<u64>>::try_into(debt[i]).unwrap());
            }

            i += 1;
        }
        let mut result_clone: BigNumber = result.clone();
        let mut i = 0;
        while  result.parts[i] == 0 && i < result.parts.len() {
            result_clone.parts.pop_front();
            i += 1;
        }
        result_clone
    }

    pub fn less(&self, other: &BigNumber) -> bool {
        let mut i: usize = 0;
        match self.parts.len().cmp(&other.parts.len()) {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => {
                while
                    self.parts[i] == other.parts[i]
                    &&
                    i < self.parts.len()
                {
                    i += 1;
                }
                if i == self.parts.len() {
                    false
                } else {
                    match self.parts[i].cmp(&other.parts[i]) {
                        Ordering::Less => true,
                        Ordering::Greater => false,
                        Ordering::Equal => false,
                    }
                }
            }
        }
    }

    pub fn less_or_eq(&self, other: &BigNumber) -> bool {
        let mut i: usize = 0;
        match self.parts.len().cmp(&other.parts.len()) {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => {
                while
                self.parts[i] == other.parts[i]
                    &&
                    i < self.parts.len()
                {
                    i += 1;
                }
                if i == self.parts.len() {
                    true
                } else {
                    match self.parts[i].cmp(&other.parts[i]) {
                        Ordering::Less => true,
                        Ordering::Greater => false,
                        Ordering::Equal => true,
                    }
                }
            }
        }
    }

    // https://leetcode.com/problems/divide-two-integers/solutions/13407/c-bit-manipulations
    pub fn modulo(&self, other: &BigNumber) -> BigNumber {
        let mut quotient: BigNumber = BigNumber::new();
        quotient.set_hex("0000000000000000").unwrap();
        let mut other_copy: BigNumber = (*other).clone();
        let mut self_copy: BigNumber = (*self).clone();

        while !self_copy.less(&other_copy) {
            let mut temp: BigNumber = other_copy.clone();
            let mut m = BigNumber::new();
            m.set_hex("01").unwrap();
            // I know this is bad code -> don't have time to fix this, sorry, I've got a job to do as well :(
            while temp.less_or_eq(&self_copy) {
                // println!("Before bitshift {}", temp.get_hex());
                temp.bitshift_left(1);
                // println!("After bitshift {}", temp.get_hex());
                m.bitshift_left(1);
            }
            // println!("F*** go back {}", temp.get_hex());
            temp.bitshift_right(1);
            m.bitshift_right(1);


            self_copy = self_copy.sub(&temp);
            // quotient = quotient.add(&m);
            // println!("Quotient {}", quotient.get_hex()); //  -> division implemented as well
        }
        self_copy
    }
}