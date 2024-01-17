//!day_10.rs

use anyhow::Result;
use my_lib::my_map_two_dim::MyMap2D;

// size of CRT
const X: usize = 40;
const Y: usize = 6;

struct SignalRegister<const X: usize, const Y: usize> {
    x: i32,
    current_cycle: i32,
    check_cycle: i32,
    delta_cycle: i32,
    sum_check_cycle_x: i32,
    crt: MyMap2D<char, X, Y>,
}

impl<const X: usize, const Y: usize> SignalRegister<X, Y> {
    fn new(initial_check_cycle: i32, delta_cycle: i32) -> Self {
        SignalRegister {
            x: 1,
            current_cycle: 0,
            check_cycle: initial_check_cycle,
            delta_cycle,
            sum_check_cycle_x: 0,
            crt: MyMap2D::<char, X, Y>::default(),
        }
    }
    fn apply(&mut self, register_command: &RegisterCommand) {
        let next_cycle = self.current_cycle + register_command.cycle_time;
        for cycle in self.current_cycle..next_cycle {
            let pixel_x = cycle % self.delta_cycle;
            let pixel_y = cycle / self.delta_cycle;
            let pixel = if pixel_x >= self.x - 1 && pixel_x <= self.x + 1 {
                '#'
            } else {
                '.'
            };
            self.crt.set((pixel_x as usize, pixel_y as usize).into(), pixel);
        }
        self.current_cycle = next_cycle;
        if self.current_cycle >= self.check_cycle {
            self.sum_check_cycle_x += self.x * self.check_cycle;
            self.check_cycle += self.delta_cycle;
        }
        self.x += register_command.delta_x;
    }
}

#[derive(Debug)]
struct RegisterCommand {
    cycle_time: i32,
    delta_x: i32
}

impl From<&str> for RegisterCommand {
    fn from(value: &str) -> Self {
        let mut split_value = value.split_whitespace();
        if let Some(c) = split_value.next() {
            match c {
                "noop" => return RegisterCommand {
                    cycle_time: 1,
                    delta_x: 0,
                },
                "addx" => if let Some(x) = split_value.next() {
                    let delta_x = x.parse::<i32>().expect("bad input");
                    return RegisterCommand {
                        cycle_time: 2,
                        delta_x,
                    };
                },
                _ => (),
            }
        }
        panic!("bad input");
    }
}

pub fn day_10() -> Result<()> {
    let input = include_str!("../../assets/day_10.txt");
    let register_commands: Vec<RegisterCommand> = input.lines().map(RegisterCommand::from).collect();
    let mut signal_register = SignalRegister::<X, Y>::new(20, 40);
    for rc in register_commands.iter() {
        signal_register.apply(rc);
    }
    
    let result_part1 = signal_register.sum_check_cycle_x;
    println!("result day 10 part 1: {}", result_part1);
    assert_eq!(result_part1, 14_860);

    let result_part2 = format!("{}", signal_register.crt);
    // result in letters: RGZEHURK
    let test_part2 = "###...##..####.####.#..#.#..#.###..#..#.\n\
                            #..#.#..#....#.#....#..#.#..#.#..#.#.#..\n\
                            #..#.#......#..###..####.#..#.#..#.##...\n\
                            ###..#.##..#...#....#..#.#..#.###..#.#..\n\
                            #.#..#..#.#....#....#..#.#..#.#.#..#.#..\n\
                            #..#..###.####.####.#..#..##..#..#.#..#.";
    println!("result day 10 part 2:\n{}", result_part2.trim());
    assert_eq!(result_part2.trim(), test_part2);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_10_example.txt");
        let register_commands: Vec<RegisterCommand> = input.lines().map(RegisterCommand::from).collect();
        //eprintln!("{:?}", &register_commands[..10]);
        let mut signal_register = SignalRegister::<X, Y>::new(20, 40);
        for rc in register_commands.iter() {
            signal_register.apply(rc);
        }
        
        let result_part1 = signal_register.sum_check_cycle_x;
        println!("result example day 10 part 1: {}", result_part1);
        assert_eq!(result_part1, 13_140);


        let result_part2 = format!("{}", signal_register.crt);
        let test_part2 = "##..##..##..##..##..##..##..##..##..##..\n\
                                ###...###...###...###...###...###...###.\n\
                                ####....####....####....####....####....\n\
                                #####.....#####.....#####.....#####.....\n\
                                ######......######......######......####\n\
                                #######.......#######.......#######.....";
        println!("result example day 10 part 2:\n{}", result_part2.trim());
        assert_eq!(result_part2.trim(), test_part2);
        Ok(())
    }
}
