pub fn show(limit: u8) {
    let numbers = generate_sequence(limit);
    output_sequence(&numbers);
    let range = [1u8..=10u8];
    for i in range{
        println!("{:?}",i);
    }
}
fn generate_sequence(limit: u8) -> Vec<u8> {
    let mut numbers = Vec::new();
    for n in 1..=limit {
        numbers.push(n);
    }
    numbers}
fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}