#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let contents = input
        .split("\n\n");
        
    let mut elf_calories : Vec<i32> = Vec::new();
    for elf in contents{
        let calorie_iter = elf.split("\n");
        let mut calories = 0;
        for calorie in calorie_iter{
            calories += calorie.parse::<i32>().unwrap();
        }
        elf_calories.push(calories);
    }

    elf_calories.sort();
    
    return elf_calories.iter().rev().take(1).sum();
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let contents = input
        .split("\n\n");
        
    let mut elf_calories : Vec<i32> = Vec::new();
    for elf in contents{
        let calorie_iter = elf.split("\n");
        let mut calories = 0;
        for calorie in calorie_iter{
            calories += calorie.parse::<i32>().unwrap();
        }
        elf_calories.push(calories);
    }

    elf_calories.sort();
    
    return elf_calories.iter().rev().take(3).sum();
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    // ((( and (()(()( both result in floor 3.
    #[test]
    fn sample2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    // ))((((( also results in floor 3.
    #[test]
    fn sample3() {
        assert_eq!(part1("))((((("), 3);
    }

    // ()) and ))( both result in floor -1 (the first basement level).
    #[test]
    fn sample4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    // ))) and )())()) both result in floor -3.
    #[test]
    fn sample5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample6() {
        assert_eq!(part2(")"), 1);
    }

    // ()()) causes him to enter the basement at character position 5.
    #[test]
    fn sample7() {
        assert_eq!(part2("()())"), 5);
    }
}
