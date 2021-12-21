use std::{fs, num::ParseIntError};

type ParseResult<T> = Result<T, ParseIntError>;

#[derive(Debug)]
enum Operation {
    Add(Vec<Packet>),
}

#[derive(Debug)]
enum Type {
    Literal(u32),
    Operator(Operation),
}

#[derive(Debug)]
struct Packet {
    // The number of bits that make up the packet.
    binary_length: u32,
    version: u8,
    packet_type: Type,
}

fn main() {
    // let input = fs::read_to_string("input/day16")
    //     .expect("Something went wrong reading the file.");
    // let input = String::from("D2FE28");
    let input = String::from("38006F45291200");
    // let input = String::from("8A004A801A8002F478");

    let binary_string = input.chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .map(|int| format!("{:0>4b}", int))
        .collect::<Vec<String>>()
        .join("");
    dbg!(&binary_string);
    let packets = parse_packets(&binary_string).unwrap();
    dbg!(&packets);

    dbg!(part1(&packets));
    dbg!(part2(&packets));
}

fn parse_packet(binary: &str) -> ParseResult<Packet> {
    let version = u8::from_str_radix(&binary[0..3], 2)?;
    let type_id = u8::from_str_radix(&binary[3..6], 2)?;

    let packet_type = match type_id {
        4 => {
            let literal_binary = &binary[6..];
            parse_literal(literal_binary)
        },
        _ => {
            unimplemented!();
        },
    }?;

    unimplemented!();
}

/// Parse the literal part of the binary string.
fn parse_literal(binary: &str) -> ParseResult<Type> {
    let mut literal_binary = String::new();
    for bits in binary.chars().collect::<Vec<_>>().chunks(5) {
        for bit in bits[1..].iter() {
            literal_binary.push(*bit);
        }
        if bits[0] == '0' {
            break;
        }
    }
    Ok(Type::Literal(u32::from_str_radix(&literal_binary, 2)?))
}








fn parse_packets(binary_string: &str) -> Result<Vec<Packet>, ParseIntError> {
    let version = u8::from_str_radix(&binary_string[0..3], 2)?;
    let type_id = match u8::from_str_radix(&binary_string[3..6], 2)? {
        4 => parse_literal(&binary_string[6..])?,
        _ => Type::Operator(parse_operator(&binary_string[6..])),
    };

    Ok(Vec::from([Packet { version: version, packet_type: type_id }]))
}

// fn parse_literal(binary_string: &str) -> Result<Type, ParseIntError> {
//     let mut literal_binary = String::new();
//     for bits in binary_string.chars().collect::<Vec<_>>().chunks(5) {
//         for bit in bits[1..].iter() {
//             literal_binary.push(*bit);
//         }
//         if bits[0] == '0' {
//             break;
//         }
//     }
//     Ok(Type::Literal(u32::from_str_radix(&literal_binary, 2)?))
// }

fn parse_operator(binary_string: &str) -> Operation {
    let length_type_id = binary_string.chars().nth(0).unwrap();
    dbg!(length_type_id);
    let packet_length = match length_type_id {
        '0' => {
            let length_in_bits = usize::from_str_radix(&binary_string[1..16], 2).unwrap();
            // TODO: I need to keep parsing packets as long as I haven't used all the bits indicated
            // by the length. This means I need a way to indicate how many bits were used when parsing
            // a packet.
            dbg!(parse_packets(&binary_string[16..length_in_bits]));
        }
        '1' => {
            let number_of_packets = u32::from_str_radix(&binary_string[1..12], 2).unwrap();
            dbg!(number_of_packets);
        },
        _ => panic!("Invalid length type ID."),
    };
    dbg!(packet_length);

    Operation::Add(Vec::new())
}

fn part1(packets: &Vec<Packet>) -> Option<u32> {
    Some(packets.first()?.version as u32)
}

fn part2(packets: &Vec<Packet>) -> u32 {
    0
}
