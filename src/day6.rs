use std::str;

fn eval_buffer(buffer: &[u8], buffer_size: usize) -> bool{
    for i in 0..buffer_size{
        for y in (i + 1)..buffer_size{
            if buffer[i] == buffer[y]{
                return false;
            }
        }
    }
    return true;
}

fn find_marker(all_chars: &[u8], buffer_size: usize) -> usize{
    let mut buffer: Vec<u8> = Vec::new();
    let mut i:usize = 0;

    for char in all_chars{
        buffer.push(*char);
        if i >= (buffer_size - 1) && eval_buffer(&buffer[(i - (buffer_size - 1))..(i + 1)], buffer_size){
            return i + 1;
        }
        i += 1;
    }
    return 0; 
}

#[aoc(day6, part1)]
pub fn part1(original_input: &str) -> usize{
    let buffer_size = 4;
    return find_marker(original_input.as_bytes(), buffer_size);
}

#[aoc(day6, part2)]
pub fn part2(original_input: &str) -> usize{
    let buffer_size = 14;
    return find_marker(original_input.as_bytes(), buffer_size);
}