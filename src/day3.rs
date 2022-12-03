use std::collections::{HashMap, HashSet};

pub fn input_generator(input: &str) -> Vec<&str> {
    return input.lines().collect();
}

pub fn char_to_priority(input: &char) -> u32{
    let priority_normaliser;
    if input.is_lowercase(){
        priority_normaliser = 96;
    }
    else{
        priority_normaliser = 38;
    }
    return *input as u32 - priority_normaliser;
} 

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let parsed_input = input_generator(input);
    let mut tally = 0;
    for bag in parsed_input{
        let length = bag.len();
        let mut map : HashMap<u32, bool> = HashMap::new();
        
        let first_half = &bag[0..(length / 2)];
        for n in first_half.chars(){
            let digit = char_to_priority(&n);
            map.insert(digit, true);
        }
        let second_half = &bag[((length / 2)) ..];
        for n in second_half.chars(){
            let digit = char_to_priority(&n);
            if map.contains_key(&digit){
                tally += digit;
                break;
            }
        }
    }

    return tally;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let parsed_input = input_generator(input);

    let mut tally = 0;
    let group_count = parsed_input.len() / 3;
    for i in 0 .. group_count{
        let group = &parsed_input[i * 3 .. ((i * 3) + 3)];

        let mut group_tally : HashMap<u32, i8> = HashMap::new(); 

        for bag in group{
            let mut chars_in_bag: HashSet<u32> = HashSet::new();  
            for char in (*bag).chars(){
                let char_priority = char_to_priority(&char);
                chars_in_bag.insert(char_priority);
            }
            for char in chars_in_bag{
                match group_tally.get(&char) {
                    Some(val) => {
                        group_tally.insert(char, *val + 1);
                    },
                    None => {
                        group_tally.insert(char, 1);
                    }
                }
            }
        }

        tally += group_tally
            .iter()
            .filter(|x| *x.1 == 3)
            .map(|f| *f.0)
            .next()
            .unwrap();
    }
    return tally;
}

#[cfg(test)]
mod tests {
    use super::{part1,part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("AAAb\nAAAb"), 4);
    }

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample2() {
        assert_eq!(part2("fghA\nxyzA\ndefA"), 27);
    }

}
