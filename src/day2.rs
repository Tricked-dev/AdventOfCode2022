enum Option {
    Rock,
    Paper,
    Scissors,
}
impl From<&str> for Option {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Option::Rock,
            "B" | "Y" => Option::Paper,
            "C" | "Z" => Option::Scissors,
            _ => panic!("Invalid option"),
        }
    }
}

#[test]
fn day2_part1() {
    let input = include_str!("../resources/day2.txt");
    let mut score = 0;
    for game in input.lines() {
        let (him, me) = game.split_once(' ').unwrap();
        let him = Option::from(him);
        let me = Option::from(me);
        score += match me {
            Option::Rock => 1,
            Option::Paper => 2,
            Option::Scissors => 3,
        };
        score += match (him, me) {
            (Option::Rock, Option::Rock) => 3,
            (Option::Rock, Option::Paper) => 6,

            (Option::Paper, Option::Paper) => 3,
            (Option::Paper, Option::Scissors) => 6,

            (Option::Scissors, Option::Scissors) => 3,
            (Option::Scissors, Option::Rock) => 6,
            _ => 0,
        };
    }

    println!("part1 {}", score);
    assert_eq!(score, 14264);
}

// i fucking give up someone else fix this shitty code plz
// #[test]
// fn day2_part2() {
//     let input = include_str!("../resources/day2.txt");
//     let mut score = 0;
//     for game in input.lines() {
//         let (him, me) = game.split_once(' ').unwrap();
//         let him = Option::from(him);
//         let me = Option::from(me);
//         score += match me {
//             Option::Rock => 0,
//             Option::Paper => 3,
//             Option::Scissors => 6,
//         };
//         score += match (him, me) {
//             (Option::Rock, Option::Rock) => 3,
//             (Option::Rock, Option::Paper) => 1,

//             (Option::Paper, Option::Paper) => 3,
//             (Option::Paper, Option::Scissors) => 1,

//             (Option::Scissors, Option::Scissors) => 3,
//             (Option::Scissors, Option::Rock) => 1,
//             _ => 2,
//         };
//     }

//     println!("part2 {}", score)
// }
