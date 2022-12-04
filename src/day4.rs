#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<((i32,i32), (i32, i32))> {
    return input
        .lines()
        .map(|line| {
            let mut moves = line
                .split(',');
            let first = input_to_range(moves.next().unwrap());
            let second = input_to_range(moves.next().unwrap());
            return (first, second);
        })
        .collect();
}

fn input_to_range(input: &str) -> (i32, i32) {
    let mut range = input.split('-');
    let range_start = &range.next().unwrap().parse::<i32>().unwrap();
    let range_end = &range.next().unwrap().parse::<i32>().unwrap();
    return (*range_start, *range_end);
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<((i32,i32), (i32, i32))>) -> i32 {
    let mut overlapping_pairs = 0;
    for pair in input{
        if pair.0.0 >= pair.1.0{
            if pair.0.1 <= pair.1.1{
                overlapping_pairs += 1;
                continue;
            }
        }
        if pair.1.0 >= pair.0.0{
            if pair.1.1 <= pair.0.1{
                overlapping_pairs += 1;
            }
        }
    }
    
    return overlapping_pairs;
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<((i32,i32), (i32, i32))>) -> i32 {
    let mut overlapping_pairs = 0;
    for pair in input{
        if pair.0.0 >= pair.1.0{
            if pair.0.0 <= pair.1.1{
                overlapping_pairs += 1;
            }
        }
        else{
            if pair.1.0 <= pair.0.1{
                overlapping_pairs += 1;
            }
        }
    }
    
    return overlapping_pairs;
}

#[cfg(test)]
mod tests {
    use super::{part1};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        let mut vec: Vec<((i32,i32), (i32, i32))> = Vec::new();
        vec.push(((2,8), (3,7)));
        vec.push(((2,8), (3,7)));
        vec.push(((3,7), (2,8)));
        vec.push(((3,7), (2,8)));
        vec.push(((2,6), (4,8)));
        vec.push(((2,8), (2,7)));
        
        
        assert_eq!(part1(&vec), 5);
    }
}
