use std::str;

#[derive(Debug)]
struct Instruction{
    quantity: u8,
    origin: u8,
    destination: u8
}

fn input_generator(input: &str) -> (Vec<Vec<u8>>, Vec<Instruction>) {
    let sectors = input
        .split_once("\n\n").unwrap();
    
    let mut bags = generate_bags(sectors.0);

    for line in sectors.0.lines().rev() {
        fill_bags(line, &mut bags);
    }

    let instructions = sectors
        .1
        .lines()
        .map(|line| {
            let parts : Vec<&str> = line.split(" ").collect();
            let quantity = parts[1].parse::<u8>().unwrap();
            let origin = parts[3].parse::<u8>().unwrap();
            let destination  = parts[5].parse::<u8>().unwrap();

            return Instruction{quantity, origin, destination};
        })
        .collect();

    return (bags.to_vec(), instructions);
    
}

fn generate_bags(lines: &str) -> Vec<Vec<u8>>{
    return lines
        .lines()
        .last()
        .unwrap()
        .chars()
        .filter(|i| *i != ' ')
        .map(|_i| Vec::new())
        .collect();
}

fn fill_bags(line: &str, bags: &mut Vec<Vec<u8>>){
    let mut bag_idx = 0;
    let mut char_idx = 0;
    let line_bytes = line.as_bytes();
    while char_idx < line_bytes.len(){
        if line_bytes[char_idx] as char == '['{
            bags[bag_idx].push(line_bytes[char_idx +1]);
        }

        char_idx += 4;
        bag_idx += 1;
    }
}

fn enact_move(bags: &mut Vec<Vec<u8>>, instruction: &Instruction){
    for _idx in 0..instruction.quantity {
        match bags[(instruction.origin - 1) as usize].pop() {
            Some(item) => {bags[(instruction.destination - 1) as usize].push(item);},
            None => {print!("{:?}", instruction); panic!("aa");}
        };
    } 
}

//TODO: make this O(1)
fn enact_multi_move(bags: &mut Vec<Vec<u8>>, instruction: &Instruction){
    let mut pulled: Vec<u8> = Vec::new();
    for _idx in 0..instruction.quantity {
        match bags[(instruction.origin - 1) as usize].pop() {
            Some(item) => pulled.push(item),
            None => {print!("{:?}", instruction); panic!("aa");}
        };
    } 

    while pulled.len() > 0 {
        bags[(instruction.destination - 1) as usize].push(pulled.pop().unwrap());
    }
}

#[aoc(day5, part1)]
pub fn part1(original_input: &str) -> String{
    let input = input_generator(original_input);
    let mut bags : Vec<Vec<u8>> = input.0.to_vec();
    for instruction in &input.1{
        enact_move(&mut bags, instruction);
    }

    let top_stacks = bags.iter().map(|o| *(o.last().unwrap())).collect::<Vec<u8>>(); 

    let top_of_stacks = str::from_utf8(&top_stacks).unwrap();
    return top_of_stacks.to_string();
}

#[aoc(day5, part2)]
pub fn part2(original_input: &str) -> String{
    let input = input_generator(original_input);
    let mut bags : Vec<Vec<u8>> = input.0.to_vec();

    for instruction in &input.1{
        enact_multi_move(&mut bags, instruction);
    }

    let top_stacks = bags.iter().map(|o| *(o.last().unwrap())).collect::<Vec<u8>>(); 

    let top_of_stacks = str::from_utf8(&top_stacks).unwrap();
    return top_of_stacks.to_string();
}

#[cfg(test)]
mod tests {
    use crate::day5::enact_move;

    use super::{Instruction, fill_bags, generate_bags};

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
        let bag_line = " 1   2   3   4   5   6   7   8   9 ";
        let mut bags = generate_bags(bag_line);
        let line_1 = "[H]     [G] [D] [Q]         [T] [J]";
        let line_2 = "        [C] [S]     [G]     [V] [M]";
        fill_bags(line_1, &mut bags);
        fill_bags(line_2, &mut bags);
        assert_eq!(bags[0].len(), 1);
        assert_eq!(bags[1].len(), 0); 
        assert_eq!(bags[2].len(), 2);
        assert_eq!(bags[3].len(), 2);
        assert_eq!(bags[4].len(), 1);
        assert_eq!(bags[5].len(), 1);
        assert_eq!(bags[6].len(), 0);
        assert_eq!(bags[7].len(), 2); 
        assert_eq!(bags[8].len(), 2);  
        let instruction = Instruction{quantity: 2, origin: 3, destination: 2};
        enact_move(&mut bags, &instruction);
        assert_eq!(bags[1].len(), 2);
        assert_eq!(bags[2].len(), 0);     
    }

    #[test]
    fn sample1_bags() {
        let mut vec: Vec<((i32,i32), (i32, i32))> = Vec::new();
        vec.push(((2,8), (3,7)));
        vec.push(((2,8), (3,7)));
        vec.push(((3,7), (2,8)));
        vec.push(((3,7), (2,8)));
        vec.push(((2,6), (4,8)));
        vec.push(((2,8), (2,7)));
        let line = " 1   2   3   4   5   6   7   8   9 ";
        let mut bags = generate_bags(line);
        let myBag = bags[8].push(2);
    }
}
