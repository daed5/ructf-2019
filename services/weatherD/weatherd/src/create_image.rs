extern crate regex;

use regex::bytes::Regex;
use std::path::Path;

pub fn encode(s : &str) -> Vec<u8>{
    let mut vec: Vec<u128> = Vec::new();

    let mut i = 0;
    let mut number : u128 = 0;

    let padding = "00000000000000000000000000000000";
    let mut ss = s.to_string() + padding;

    ss.truncate(32);

    for x in ss.to_lowercase().chars().into_iter() {
        i += 1;
        let n = convert_char(x) as u128;

        number *= 37;
        number += n;

        if i == 16 {
            vec.push(number);
            i = 0 ;
            number = 0;
        }
    }

    vec.push(number);

    let mut result: Vec<u8> = Vec::new();

    for x in vec.into_iter() {
        let mut xx = x;

        for _ in 1..=12 {
            result.push(xx as u8);
            xx = xx >> 8;
        }
    }

    return result;
}

pub fn decode(vec : Vec<u8> ) -> String {
    let mut nums: Vec<u128> = Vec::new();

    let mut i = 0;
    let mut num: u128 = 0;

    for v in vec.into_iter().rev() {
        i += 1;
        num = num << 8;
        num += v as u128;

        if i == 12 {
            i = 0;
            nums.push(num);
            num = 0;
        }
    }

    i = 0;
    let mut result: Vec<char> = Vec::new();
    for x in nums.into_iter() {
        let mut r: u128 = x;

        for _ in 1..=17 {
            let num = (r % 37) as u32;
            result.push(deconvert_char(num));
            r = r / 37;
        }
    }

    let mut result2: Vec<char> = Vec::new();

    for x in 0..32 {
        result2.push(result[x]);
    }

    let result3: String = result2.into_iter().rev().collect();

    return result3;
}

fn calculate_mask(){
}

fn convert_char(c : char) -> u32 {
    if c >= '0' && c <='9' {
        return c as u32 - '0' as u32 ;
    }

    if c >= 'a' && c <='z' {
        return c as u32  - 'a' as u32  + 10;
    }

    if c == '='{
        return 36;
    }

    return 36;
}


fn deconvert_char(n : u32) -> char{
    if n >=0 && n <10{
       return  (n as u8 + '0' as u8) as char;
    }

    if n >=10 && n <36{
        return  ((n - 10) as u8 + 'a' as u8) as char;
    }

    if n==36{
        return '='
    }

    panic!("unexpected number {}", n);
}
