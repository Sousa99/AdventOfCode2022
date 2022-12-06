
mod read;
mod lib;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let (stacks, instructions) = lib::parse_input(&input);
    
    // Part 1

    let mut stacks_9000 = stacks.clone();
    for instruction in instructions.iter() { lib::make_iteration_9000(&mut stacks_9000, instruction) }
    let mut vec_stacks_9000: Vec<&lib::Stack> = stacks_9000.values().collect();
    vec_stacks_9000.sort_by_key(|stack| stack.stack_id);

    let crates_on_top_9000: String = vec_stacks_9000.iter()
        .map(|stack| stack.containers.last())
        .filter(|container_option| container_option.is_some())
        .map(|container_option| container_option.unwrap())
        .collect();
    println!("\r📦 Crates on top by the end with 'CrateMover 9000': '{}' (Part 1)", crates_on_top_9000);
    
    // Part 2

    let mut stacks_9001 = stacks.clone();
    for instruction in instructions.iter() { lib::make_iteration_9001(&mut stacks_9001, instruction) }
    let mut vec_stacks_9001: Vec<&lib::Stack> = stacks_9001.values().collect();
    vec_stacks_9001.sort_by_key(|stack| stack.stack_id);

    let crates_on_top_9001: String = vec_stacks_9001.iter()
        .map(|stack| stack.containers.last())
        .filter(|container_option| container_option.is_some())
        .map(|container_option| container_option.unwrap())
        .collect();
    println!("\r📦 Crates on top by the end with 'CrateMover 9001': '{}' (Part 2)", crates_on_top_9001);

}