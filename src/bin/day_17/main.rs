use std::ops::BitXor;
use std::thread::sleep;
use itertools::Itertools;

fn combo(n: usize, registers: &[usize]) -> usize {
    match n {
        0..=3 => n,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!(),
    }
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

    let mut regs = registers.clone();
    // regs[0] = 45000000000000;
    // regs[0] = a;
    let output = run(&mut regs, &program);
    println!("output: {:?}", output);
    println!("a: {:?}", regs[0]);
    println!("b: {:?}", regs[1]);
    println!("c: {:?}", regs[2]);

    // let mut a = 0;
    // loop {
    //     let mut regs = registers.clone();
    //     regs[0] = a;
    //     let output = run(&mut regs, &program);
    //     println!("a: {}, output: {:?}", a, output);
    //     // sleep(std::time::Duration::from_millis(10));
    //     a += 1;
    // }
}
