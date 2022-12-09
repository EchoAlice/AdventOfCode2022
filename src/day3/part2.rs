use crate::helper::input_to_strings;

fn compare_types(c: char, other_elves: [&String; 2]) -> i32 {
    let mut matches = 0; 
    let mut priority = 0; 
   
    for i in 0..2 {
        for j in other_elves[i].chars() {
            if c == j {
                matches += 1;
                break    
            }
        }
    }   
    if matches == 2 {
        println!("Matching type: {}", c); 
        if c.is_lowercase() { 
            priority = (c.clone() as i32) - 96; 
        } else if c.is_uppercase() { 
            priority = (c.clone() as i32) - 38; 
        }
    }
    priority
}

// Each elf in the group shares one type.  The shared type == badge
fn find_badge(group: Vec<&String>) -> i32{
    let mut priority = 0; 
    let elf1 = group[0]; 
    let other_elves = [group[1], group[2]]; 
    for c in elf1.chars() {
        priority = compare_types(c, other_elves);
        if priority > 0 {
            break
        }
    } 
    priority
}



fn organize_groups() -> i32 {
    let mut line_index = 0;
    let mut priority_sum = 0;
    let mut group = Vec::new(); 
    let input = input_to_strings("src/day3/input.txt");
    
    for line in input.iter() {
        group.push(line); 
        line_index += 1;  
        if line_index % 3 == 0 {
            priority_sum += find_badge(group);
            group = Vec::new();
        } 
    } 
    priority_sum 
}

pub fn run_day3_part2() -> i32 {
    let answer = crate::day3::part2::organize_groups();
    answer
}