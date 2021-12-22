use std::{fs, num::ParseIntError};

type ParseResult<T> = Result<T, ParseIntError>;

#[derive(Debug)]
enum Type {
    Literal(u32),
    Add(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: Type,
    // The number of bits that make up the packet.
    size: usize,
}

fn main() {
    // let input = fs::read_to_string("input/day16")
    //     .expect("Something went wrong reading the file.");
    // let input = String::from("D2FE28");
    // let input = String::from("38006F45291200");
    // let input = String::from("8A004A801A8002F478");
    let input = String::from("620080001611562C8802118E34");
    // let input = String::from("C0015000016115A2E0802F182340");
    // let input = String::from("A0016C880162017C3686B18A3D4780");

    let binary_string = input.chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .map(|int| format!("{:0>4b}", int))
        .collect::<Vec<String>>()
        .join("");
    let packet = parse_packet(&binary_string).unwrap();
    dbg!(&packet);

    dbg!(part1(&packet));
    dbg!(part2(&packet));
}

fn part1(packet: &Packet) -> u32 {
    match &packet.packet_type {
        Type::Literal(_) => packet.version as u32,
        Type::Add(packets) => {
            let subpacket_versions: u32 = packets.iter().map(|packet| part1(packet)).sum();
            packet.version as u32 + subpacket_versions
        },
    }
}

fn part2(packets: &Packet) -> u32 {
    0
}

fn parse_packet(packet_binary: &str) -> ParseResult<Packet> {
    let version = u8::from_str_radix(&packet_binary[0..3], 2)?;
    let type_id = u8::from_str_radix(&packet_binary[3..6], 2)?;
    let contents_binary = &packet_binary[6..];
    dbg!(&packet_binary);
    dbg!(&contents_binary);

    let packet_type = match type_id {
        4 => {
            parse_literal(contents_binary)
        },
        _ => {
            parse_operator(contents_binary)
        },
    }?;

    let packet = Packet { version, packet_type, size: packet_binary.len() };
    dbg!(&packet);
    Ok(packet)
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

fn parse_operator(binary: &str) -> ParseResult<Type> {
    let length_type_id = binary.chars().nth(0).unwrap();
    let mut packets: Vec<Packet> = Vec::new(); // TODO: remove this and just get it from the match.
    match length_type_id {
        '0' => {
            let length_in_bits = usize::from_str_radix(&binary[1..16], 2).unwrap();
            let mut start_bit: usize = 16;
            while packets.iter().map(|packet| packet.size as usize).sum::<usize>() < length_in_bits {
                let packet = parse_packet(&binary[start_bit..])?;
                start_bit += packet.size;
                packets.push(packet);
            }
        }
        '1' => {
            let number_of_packets = usize::from_str_radix(&binary[1..12], 2).unwrap();
            let mut start_bit: usize = 12;
            while packets.len() < number_of_packets {
                let packet = parse_packet(&binary[start_bit..])?;
                start_bit += packet.size;
                packets.push(packet);
            }
        },
        _ => panic!("Invalid length type ID."),
    };

    Ok(Type::Add(packets))
}






// fn parse_packets(binary_string: &str) -> Result<Vec<Packet>, ParseIntError> {
//     let version = u8::from_str_radix(&binary_string[0..3], 2)?;
//     let type_id = match u8::from_str_radix(&binary_string[3..6], 2)? {
//         4 => parse_literal(&binary_string[6..])?,
//         _ => Type::Operator(parse_operator(&binary_string[6..])),
//     };

//     Ok(Vec::from([Packet { version: version, packet_type: type_id }]))
// }

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

// fn parse_operator(binary_string: &str) -> Operation {
//     let length_type_id = binary_string.chars().nth(0).unwrap();
//     dbg!(length_type_id);
//     let packet_length = match length_type_id {
//         '0' => {
//             let length_in_bits = usize::from_str_radix(&binary_string[1..16], 2).unwrap();
//             // TODO: I need to keep parsing packets as long as I haven't used all the bits indicated
//             // by the length. This means I need a way to indicate how many bits were used when parsing
//             // a packet.
//             dbg!(parse_packets(&binary_string[16..length_in_bits]));
//         }
//         '1' => {
//             let number_of_packets = u32::from_str_radix(&binary_string[1..12], 2).unwrap();
//             dbg!(number_of_packets);
//         },
//         _ => panic!("Invalid length type ID."),
//     };
//     dbg!(packet_length);

//     Operation::Add(Vec::new())
// }
