#[test]
fn day4_part1() {
    let input = include_str!("../resources/day4.txt");
    let mut dupes = 0;
    for line in input.lines() {
        let (first, second) = line
            .split_once(',')
            .map(|x| {
                (
                    (
                        x.0.split_once('-').unwrap().0.parse::<i32>().unwrap(),
                        x.0.split_once('-').unwrap().1.parse::<i32>().unwrap(),
                    ),
                    (
                        x.1.split_once('-').unwrap().0.parse::<i32>().unwrap(),
                        x.1.split_once('-').unwrap().1.parse::<i32>().unwrap(),
                    ),
                )
            })
            .unwrap();
        if (first.0 <= second.0 && first.1 >= second.1)
            || (second.0 <= first.0 && second.1 >= first.1)
        {
            dupes += 1;
        }
    }
    println!("{dupes}");
    assert_eq!(dupes, 500);
}
#[test]
fn day4_part2() {
    let input = include_str!("../resources/day4.txt");
    let mut dupes = 0;
    for line in input.lines() {
        let (first, second) = line
            .split_once(',')
            .map(|x| {
                (
                    (
                        x.0.split_once('-').unwrap().0.parse::<i32>().unwrap(),
                        x.0.split_once('-').unwrap().1.parse::<i32>().unwrap(),
                    ),
                    (
                        x.1.split_once('-').unwrap().0.parse::<i32>().unwrap(),
                        x.1.split_once('-').unwrap().1.parse::<i32>().unwrap(),
                    ),
                )
            })
            .unwrap();

        'br: for i in first.0..=first.1 {
            for j in second.0..=second.1 {
                if i == j {
                    dupes += 1;
                    break 'br;
                }
            }
        }
    }
    println!("{dupes}");
}
