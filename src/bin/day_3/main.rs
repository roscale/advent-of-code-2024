use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let sum: u32 = regex.captures_iter(input).map(|captures| {
        let terms: [u32; 2] = [1, 2].map(|i| captures.get(i).unwrap().as_str().parse().unwrap());
        terms[0] * terms[1]
    }).sum();

    println!("Sum: {}", sum);

    let regex = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let (_, sum) = regex.captures_iter(input).fold((true, 0), |(enabled, sum), captures| {
        match captures.get(0).unwrap().as_str() {
            "do()" => return (true, sum),
            "don't()" => return (false, sum),
            _ => if enabled {
                let terms: [u32; 2] = [1, 2].map(|i| captures.get(i).unwrap().as_str().parse().unwrap());
                (true, sum + terms[0] * terms[1])
            } else {
                (false, sum)
            }
        }
    });

    println!("Conditional sum: {}", sum);
}
