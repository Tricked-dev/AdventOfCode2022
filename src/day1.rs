#[test]
fn day1_part1() {
    let input = include_str!("../resources/day1.txt");
    let res: i32 = input
        .split("\n\n")
        .map(|x| x.split('\n').map(|x| x.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap();
    println!("part1 {}", res)
}

#[test]
fn day1_part2() {
    let input = include_str!("../resources/day1.txt");
    let mut res = input
        .split("\n\n")
        .map(|x| x.split('\n').map(|x| x.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
    res.sort();
    res.reverse();
    println!("part2 {}", res[0] + res[1] + res[2])
}
