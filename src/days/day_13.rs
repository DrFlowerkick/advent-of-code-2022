//!day_13.rs

use anyhow::Result;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(value: &str) -> Self {
        if value.starts_with('[') {
            let mut list: Vec<Packet> = Vec::new();
            let mut brackets_count = 1;
            let mut index: usize = 0;
            let mut seperator_indices: Vec<usize> = Vec::new();

            while brackets_count > 0 {
                index += 1;
                match &value[index..index + 1] {
                    "[" => brackets_count += 1,
                    "]" => brackets_count -= 1,
                    "," => {
                        if brackets_count == 1 {
                            seperator_indices.push(index);
                        }
                    }
                    _ => (),
                }
            }
            assert_eq!(index + 1, value.len());
            if index > 1 {
                let mut prev_sep_i = 1;
                // append index to include last respectivly single item
                seperator_indices.push(index);
                for sep_i in seperator_indices.into_iter() {
                    list.push(Packet::from(&value[prev_sep_i..sep_i]));
                    prev_sep_i = sep_i + 1;
                }
            }
            return Packet::List(list);
        }
        Packet::Integer(value.parse::<u8>().expect("bad input"))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(s), Packet::Integer(o)) => s.cmp(o),
            (Packet::List(s), Packet::List(o)) => {
                for (left, right) in s.iter().zip(o.iter()) {
                    match left.cmp(right) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                    }
                }
                s.len().cmp(&o.len())
            }
            (Packet::Integer(s), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*s)]).cmp(other)
            }
            (Packet::List(_), Packet::Integer(o)) => {
                self.cmp(&Packet::List(vec![Packet::Integer(*o)]))
            }
        }
    }
}

fn compare_packet_pairs(packet_pairs: &[(Packet, Packet)]) -> usize {
    let mut sum_index = 0;
    for (i, (left, right)) in packet_pairs.iter().enumerate() {
        match left.cmp(right) {
            Ordering::Less => sum_index += 1 + i,
            Ordering::Greater => (),
            Ordering::Equal => panic!("what to do with equal packets?"),
        }
    }
    sum_index
}

fn sort_packets(packets: &mut Vec<Packet>) -> usize {
    let seperator_1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let seperator_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    packets.push(seperator_1.clone());
    packets.push(seperator_2.clone());
    packets.sort();
    let seperator_1_index = packets.iter().position(|p| *p == seperator_1).unwrap() + 1;
    let seperator_2_index = packets.iter().position(|p| *p == seperator_2).unwrap() + 1;
    seperator_1_index * seperator_2_index
}

pub fn day_13() -> Result<()> {
    let input = include_str!("../../assets/day_13.txt");
    let packet_pairs: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|p| {
            p.split_once('\n')
                .map(|(l, r)| (Packet::from(l), Packet::from(r)))
                .unwrap()
        })
        .collect();
    let result_part1 = compare_packet_pairs(&packet_pairs);
    println!("result day 13 part 1: {}", result_part1);
    assert_eq!(result_part1, 4_734);

    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::from)
        .collect();
    let result_part2 = sort_packets(&mut packets);
    println!("result day 13 part 2: {}", result_part2);
    assert_eq!(result_part2, 21_836);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = include_str!("../../assets/day_13.txt");
        eprintln!("{:?}", Packet::from(input.lines().next().unwrap()));

        let input = include_str!("../../assets/day_13_example.txt");
        let packet_pairs: Vec<(Packet, Packet)> = input
            .split("\n\n")
            .map(|p| {
                p.split_once('\n')
                    .map(|(l, r)| (Packet::from(l), Packet::from(r)))
                    .unwrap()
            })
            .collect();
        let result_part1 = compare_packet_pairs(&packet_pairs);
        println!("result example day 13 part 1: {}", result_part1);
        assert_eq!(result_part1, 13);

        let mut packets: Vec<Packet> = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(Packet::from)
            .collect();
        let result_part2 = sort_packets(&mut packets);
        println!("result example day 13 part 2: {}", result_part2);
        assert_eq!(result_part2, 140);
        Ok(())
    }
}
