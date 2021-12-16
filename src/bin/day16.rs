use aoc::read_lines;
use bitvec::{prelude::*, order::Msb0};


const SUM : u64 = 0;
const PRODUCT : u64 = 1;
const MIN : u64 = 2;
const MAX : u64 = 3;
const LITERAL : u64 = 4;
const GREATER : u64 = 5;
const LESS : u64 = 6;
const EQ : u64 = 7;

fn decode(input: &str) -> BitVec<Msb0, u8> {
    let mut memory = BitVec::<Msb0, u8>::new();
    let chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let s: String = chars[i..i + 2].iter().collect();
        let b = u8::from_str_radix(&s, 16).unwrap();
        memory.extend(BitVec::<Msb0, u8>::from_vec(Vec::from([b])));
        i += 2;
    }
    memory
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum LengthType {
    Bits(u64,usize), //bits len, offset
    Packets(u64,usize) //num packets, offset
}

impl LengthType {
    fn new_bits(bits: u64) -> LengthType {
        LengthType::Bits(bits, 16)
    }
    fn new_packets(packets: u64 )-> LengthType {
        LengthType::Packets(packets, 12)
    }
}

fn length(memory: &BitSlice<Msb0, u8>) -> LengthType {
    if memory[6] == false {
        LengthType::new_bits(to_u64(&memory[7..22]))
    } else {
        LengthType::new_packets(to_u64(&memory[7..18]))
    }
}

fn version(memory: &BitSlice<Msb0, u8>) -> u64 {
    let ver_bits = &memory[0..3];
    to_u64(ver_bits)
}

fn packet_type(memory: &BitSlice<Msb0, u8>) -> u64 {
    let ver_bits = &memory[3..6];
    to_u64(ver_bits)
}

fn read_literal(memory: &BitSlice<Msb0, u8>) -> (usize, u64) {
    let mut result = BitVec::<Msb0, u8>::new();
    let mut i = 6;
    let mut has_another_packet = true;
    while  has_another_packet {
        has_another_packet = memory[i];
        for j in 0..4 {
            result.push(memory[i+1 +j]);
        }
        i += 5;        
    }
    (i, to_u64(&result))
}

fn version_sum_iter(memory: &BitSlice<Msb0, u8>) -> (usize, u64) {
    let pt = packet_type(memory);
    let ver = version(memory);
    if pt == LITERAL {
        let (offset, val) = read_literal(memory);
        println!("Found literal: ver={} val={}", ver, val);
        return (offset, ver);
    } else {
        let mut version_sum = ver;
        match length(memory)  {
            LengthType::Bits(sz, length_offset) => {
                println!("Found bits operator len={}, ver {} ", sz, ver);
                let bits_mem = &memory[6 + length_offset..];        
                let mut var_offset = 0;

                while var_offset < (sz as usize) {
                    let (o2, vsum) = version_sum_iter(&bits_mem[var_offset..]);
                    var_offset += o2;
                    version_sum += vsum; 
                }
                return (6 + length_offset + var_offset, version_sum);
             
            }
            LengthType::Packets(count, length_offset) => {
                println!("Found packets operator count={}, ver {} ", count, ver);
                let packets_mem = &memory[6 + length_offset..];
                let mut packets_offset = 0;
                for _i in 0..count {
                    let (o2, vsum) = version_sum_iter(&packets_mem[packets_offset..]);
                    packets_offset += o2;
                    version_sum += vsum;
                }
                return (6 + length_offset + packets_offset, version_sum);
            }
        }
    }
}

/*

const SUM : u64 = 0;
const PRODUCT : u64 = 1;
const MIN : u64 = 2;
const MAX : u64 = 3;
const LITERAL : u64 = 4;
const GREATER : u64 = 5;
const LESS : u64 = 6;
const EQ : u64 = 7;
 */

fn execute(oper: u64, vals: Vec<u64>) -> u64 {
    match oper {
        SUM => vals.iter().sum(),
        PRODUCT => vals.iter().product(),
        MIN => *vals.iter().min().unwrap(),
        MAX => *vals.iter().max().unwrap(),
        GREATER => if vals[0] > vals[1] { 1 } else { 0 }
        LESS => if vals[0] < vals[1] { 1 } else { 0 }
        EQ => if vals[0] == vals[1] { 1 } else { 0 }
        _ => panic!("Unknown oper")
    }
}


fn eval_iter(memory: &BitSlice<Msb0, u8>) -> (usize, u64) {
    let pt = packet_type(memory);
    if pt == LITERAL {
        let (offset, val) = read_literal(memory);
        return (offset, val);
    } else {
        let mut vals = Vec::new();
        match length(memory)  {
            LengthType::Bits(sz, length_offset) => {
                let bits_mem = &memory[6 + length_offset..];        
                let mut var_offset = 0;

                while var_offset < (sz as usize) {
                    let (o2, vsum) = eval_iter(&bits_mem[var_offset..]);
                    var_offset += o2;
                    vals.push(vsum);
                }
                return (6 + length_offset + var_offset, execute(pt, vals));
             
            }
            LengthType::Packets(count, length_offset) => {
                let packets_mem = &memory[6 + length_offset..];
                let mut packets_offset = 0;
                for _i in 0..count {
                    let (o2, vsum) = eval_iter(&packets_mem[packets_offset..]);
                    packets_offset += o2;
                    vals.push(vsum);
                }
                return (6 + length_offset + packets_offset, execute(pt, vals));
            }
        }
    }
}

fn version_sum(input: &str) -> u64 {
    let memory = decode(input);
    println!("bits: {}", memory);
    version_sum_iter(&memory).1
}

fn eval(input: &str) -> u64 {
    let memory = decode(input);
    eval_iter(&memory).1
}

fn part1(lines: &Vec<String>) -> u64 {
    version_sum(&lines[0])
}

fn part2(lines: &Vec<String>) -> u64 {
    eval(&lines[0])
}


fn to_u64(memory: &BitSlice<Msb0, u8>) -> u64 {
    let mut pow = 1;
    let mut acc = 0;
    for i in 0..memory.len() {
        if memory[memory.len() - (1 + i)] {
            acc += pow
        }
        pow *= 2;
    }
    acc
}

fn main() {
    if let Ok(lines) = read_lines("./input/day16.txt") {
        println!("Part 1 (951): {}", part1(&lines));
        println!("Part 2: {}", part2(&lines)); //3227 too low, 937610646452 too high
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_test_literal() {
        let input = "D2FE28";
        let memory = decode(input);
        println!("{}", &memory);
        assert_eq!(6, version(&memory));
        assert_eq!(4, packet_type(&memory));
        assert_eq!((21, 2021), read_literal(&memory));
    }

    #[test]
    fn day16_test_sub_packets_bits() {
        let input = "38006F45291200";
        let memory = decode(input);
        println!("{}", &memory);
        assert_eq!(1, version(&memory));
        assert_eq!(6, packet_type(&memory));
        
        match length(&memory) {
            LengthType::Bits(bits, offset) => {
                assert_eq!(bits, 27);
                let packet_1 = &memory[(6 + offset)..];
                assert_eq!(6, version(packet_1));
                assert_eq!(4, packet_type(packet_1));

                assert_eq!((11, 10), read_literal(&packet_1));

                let packet_2 = &packet_1[11..];
                assert_eq!(2, version(packet_2));
                assert_eq!(4, packet_type(packet_2));

                assert_eq!((16, 20), read_literal(&packet_2));

            }
            _ => panic!("Wrong type")
        }
    }


    #[test]
    fn day16_test_sub_packets_packets() {
        let input = "EE00D40C823060";
        let memory = decode(input);
        println!("{}", &memory);
        assert_eq!(7, version(&memory));
        assert_eq!(3, packet_type(&memory));
        
        match length(&memory) {
            LengthType::Packets(packets, offset) => {
                assert_eq!(packets, 3);
                let packet_1 = &memory[(6 + offset)..];
                assert_eq!(2, version(packet_1));
                assert_eq!(4, packet_type(packet_1));

                assert_eq!((11, 1), read_literal(&packet_1));

                let packet_2 = &packet_1[11..];
                println!("packet 2: {}", packet_2);
                assert_eq!(4, version(packet_2));
                assert_eq!(4, packet_type(packet_2));

                assert_eq!((11, 2), read_literal(&packet_2));

                let packet_3 = &packet_2[11..];
                assert_eq!(1, version(packet_3));
                assert_eq!(4, packet_type(packet_3));

                assert_eq!((11, 3), read_literal(&packet_3));

            }
            _ => panic!("Wrong type")
        }
    }


    #[test]
    fn day16_version_sum() {
        assert_eq!(16, version_sum("8A004A801A8002F478"));
        assert_eq!(12, version_sum("620080001611562C8802118E34"));
        assert_eq!(23, version_sum("C0015000016115A2E0802F182340"));
        assert_eq!(31, version_sum("A0016C880162017C3686B18A3D4780"));
    }


    #[test]
    fn day16_text_example_1() {
        let input = "8A004A801A8002F478";
        let input_bytes = input.as_bytes();
        let memory = BitVec::<Msb0, _>::from_slice(input_bytes).unwrap();

        let version = &memory[0..3];
        println!("{}", version);
    }


    #[test]
    fn day16_part2_examples() {
        assert_eq!(3, eval("C200B40A82"));
        assert_eq!(54, eval("04005AC33890"));
        assert_eq!(7, eval("880086C3E88112"));
        assert_eq!(9, eval("CE00C43D881120"));
        assert_eq!(1, eval("D8005AC2A8F0"));
        assert_eq!(0, eval("F600BC2D8F"));
        assert_eq!(0, eval("9C005AC2F8F0"));
        assert_eq!(1, eval("9C0141080250320F1802104A08"));
    }
}
