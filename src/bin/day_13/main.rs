use regex::Regex;

fn tokens(claw_machines: impl IntoIterator<Item=((usize, usize), (usize, usize), (usize, usize))>) -> usize {
    claw_machines.into_iter().fold(0, |tokens, (button_a, button_b, prize)| {
        let button_a = (button_a.0 as f64, button_a.1 as f64);
        let button_b = (button_b.0 as f64, button_b.1 as f64);
        let prize = (prize.0 as f64, prize.1 as f64);

        let r = button_a.0 / button_a.1;
        let b = (prize.0 - r * prize.1) / (button_b.0 - r * button_b.1);
        let a = (prize.0 - b * button_b.0) / button_a.0;

        const EPSILON: f64 = 0.01;

        if (a - a.round()).abs() < EPSILON && (b - b.round()).abs() < EPSILON {
            tokens + 3 * a.round() as usize + b.round() as usize
        } else {
            tokens
        }
    })
}

fn main() {
    let button_regex: Regex = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let input = include_str!("input.txt");

    let claw_machines = input.split("\n\n").map(|machine| {
        let mut line = machine.splitn(3, '\n');
        let [button_a, button_b] = [line.next().unwrap(), line.next().unwrap()]
            .map(|button| {
                let button_capture = button_regex.captures(button).unwrap();
                let x = button_capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let y = button_capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                (x, y)
            });

        let prize = prize_regex.captures(line.next().unwrap()).unwrap();
        let prize = (
            prize.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            prize.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        );

        (button_a, button_b, prize)
    }).collect::<Vec<_>>();

    println!("Part 1: {}", tokens(claw_machines.iter().copied()));

    let claw_machines = claw_machines.into_iter().map(|(button_a, button_b, prize)| {
        let prize = (prize.0 + 10000000000000, prize.1 + 10000000000000);
        (button_a, button_b, prize)
    });

    println!("Part 2: {}", tokens(claw_machines));
}
