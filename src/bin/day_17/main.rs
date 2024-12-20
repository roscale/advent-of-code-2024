use std::ops::BitXor;
use itertools::Itertools;

fn combo(n: usize, registers: &[usize]) -> usize {
    let a = match n {
        0..=3 => n,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!(),
    };
    dbg!(n);
    dbg!(a);
    a
}

fn run(registers: &mut [usize], program: &[usize]) -> Vec<usize> {
    let mut ip = 0;
    let mut output = Vec::new();

    loop {
        if ip >= program.len() {
            break;
        }
        match program[ip] {
            0 => registers[0] = registers[0] >> combo(program[ip + 1], &registers),
            1 => registers[1] = registers[1].bitxor(program[ip + 1]),
            2 => registers[1] = combo(program[ip + 1], &registers) % 8,
            3 => {
                if registers[0] == 0 {
                    ip += 2;
                    continue;
                }
                ip = program[ip + 1];
                continue;
            }
            4 => registers[1] = registers[1].bitxor(registers[2]),
            5 => output.push(combo(program[ip + 1], &registers) % 8),
            6 => registers[1] = registers[0] >> combo(program[ip + 1], &registers),
            7 => registers[2] = registers[0] >> combo(program[ip + 1], &registers),
            _ => unreachable!(),
        }
        ip += 2;
    }

    output
}

fn run_backwards(program: &[usize]) -> usize {
    let unrolled_program = unroll_loops(program);
    let mut registers = vec![0, 0, 0];
    let mut output_index = program.len();

    if registers[0] == 0 {
        registers[0] = 1;
    }
    
    for ip in (0..unrolled_program.len()).step_by(2).rev() {
        let arg = unrolled_program[ip + 1];
        match unrolled_program[ip] {
            0 => registers[0] <<= combo(arg, &registers),
            1 => registers[1] = registers[1].bitxor(arg),
            2 => {
                assert!(registers[1] <= 7);
                registers[1] = 0;
            }
            4 => registers[1] = registers[1].bitxor(registers[2]),
            5 => {
                output_index -= 1;
                match arg {
                    4 => registers[0] = program[output_index],
                    5 => registers[1] = program[output_index],
                    6 => registers[2] = program[output_index],
                    _ => {}
                }
            }
            6 => registers[1] = registers[0] << combo(arg, &registers),
            7 => registers[2] = registers[0] << combo(arg, &registers),
            _ => {},
        }
    }
    registers[0]
}

fn unroll_loops(program: &[usize]) -> Vec<usize> {
    let mut outputs = 0;
    let mut unrolled_program = Vec::new();

    let mut ip = 0;
    while ip < program.len() {
        match program[ip] {
            3 => {
                if outputs == program.len() {
                    ip += 2;
                    continue;
                }
                ip = program[ip + 1];
                continue;
            }
            5 => outputs += 1,
            _ => {}
        }
        unrolled_program.extend_from_slice(&program[ip..ip + 2]);
        ip += 2;
    }

    unrolled_program
}

fn main() {
    let input = include_str!("input.txt");

    let (registers, program) = input.split_once("\n\n").unwrap();

    let mut registers = registers.lines().map(|line| {
        line.split_once(": ").unwrap().1.parse::<usize>().unwrap()
    }).collect::<Vec<_>>();

    let program = program
        .split_once(": ").unwrap().1.trim()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let output = run(&mut registers, &program);
    println!("Part 1: {}", output.iter().join(","));

    let output = run_backwards(&program);
    println!("Part 2: {}", output);
}
