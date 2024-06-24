use day_6::part2::process;

#[tracing::instrument]
fn main() {
    let file = include_str!("../../input.txt");
    let result = process(file, 10_000);
    println!("{}", result);
}
