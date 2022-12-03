#[test]
fn day3_part1() {
    let input = include_str!("../resources/day3.txt");
    let prios = create_char_array().into_iter().collect::<String>();
    let mut count = 0;
    for line in input.lines() {
        let mid = line.split_at(line.len() / 2);
        let one = mid.0.to_owned();
        let two = mid.1.to_owned();
        let mut found_chars = Vec::new();
        for c in one.chars() {
            if two.contains(c) && !found_chars.contains(&c) {
                found_chars.push(c);
                count += prios.find(c).unwrap() + 1;
            }
        }
    }
    println!("{count}");
}
// used in test
#[allow(dead_code)]
fn create_char_array() -> Vec<char> {
    let mut priotirities = ('a'..='z').collect::<Vec<char>>();
    let mut priotirities2 = ('A'..='Z').collect::<Vec<char>>();
    //combine the arrays
    priotirities.append(&mut priotirities2);
    priotirities
}
