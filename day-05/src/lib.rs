
use regex::Regex;
use std::collections::HashMap;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

type StackID = u32;
type ContainerID = char;

pub struct Instruction {
    from_stack: StackID,
    to_stack: StackID,
    count: usize,
}

#[derive(Clone, Debug)]
pub struct Stack {
    pub stack_id: StackID,
    pub containers: Vec<ContainerID>,
}

// =============================================== AUXILIARY FUNCTIONS ===============================================

fn parse_stacks(line: &String) -> Vec<Option<ContainerID>> {

    return line.chars().collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk_chars| *chunk_chars.get(1).unwrap())
        .map(|char| {

            if char == ' ' {
                return None;
            } else {
                return Some(char);
            }
        })
        .collect();

}

fn parse_stack_ids(line: &String) -> Vec<StackID> {

    return line.chars().collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk_chars| *chunk_chars.get(1).unwrap())
        .map(|char| char.to_digit(10).unwrap())
        .collect();

}

fn parse_instructions(line: &String, regex_capture_instruction: &Regex) -> Instruction  {

    let captures = regex_capture_instruction.captures(&line).unwrap();

    let count: usize = captures.get(1).unwrap().as_str().parse().unwrap();
    let from_stack: StackID = captures.get(2).unwrap().as_str().parse().unwrap();
    let to_stack: StackID = captures.get(3).unwrap().as_str().parse().unwrap();

    return Instruction { from_stack, to_stack, count };
}

pub fn parse_input(input: &Vec<String>) -> (HashMap<StackID, Stack>, Vec<Instruction>) {

    let regex_stack_containers: Regex = Regex::new(r"^(\[[A-Z]\]|\s)+$").unwrap();
    let regex_stack_ids: Regex = Regex::new(r"^(?:\s(?:\d+)\s\s)+\s(?:\d+)\s$").unwrap();
    let regex_instruction: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut stack_containers: Vec<Vec<Option<ContainerID>>> = Vec::new();
    let mut stack_ids: Vec<StackID> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input {

        if regex_stack_containers.is_match(line) { stack_containers.push(parse_stacks(line)) }
        else if regex_stack_ids.is_match(line) { stack_ids = parse_stack_ids(line) }
        else if regex_instruction.is_match(line) { instructions.push(parse_instructions(line, &regex_instruction)) }
    }

    let mut stacks: Vec<Stack> = stack_ids.into_iter()
        .map(|stack_id| Stack{ stack_id, containers: Vec::new() })
        .collect();

    for containers_line in stack_containers.into_iter().rev() {

        for (stack, container) in stacks.iter_mut().zip(containers_line.into_iter()) {
            if container.is_some() {
                stack.containers.push(container.unwrap());
            }
        }

    }

    let stacks_mapped: HashMap<StackID, Stack> = stacks.into_iter()
        .map(|stack| (stack.stack_id, stack))
        .collect();

    return (stacks_mapped, instructions);
}

pub fn make_iteration_9000(stacks: &mut HashMap<StackID, Stack>, instruction: &Instruction) {

    let from_stack: &mut Stack = stacks.get_mut(&instruction.from_stack).unwrap();
    let mut containers_removed: Vec<ContainerID> = Vec::new();

    for _ in 0..instruction.count {

        let container: ContainerID = from_stack.containers.pop().unwrap();
        containers_removed.push(container);
    }

    let to_stack: &mut Stack = stacks.get_mut(&instruction.to_stack).unwrap();
    to_stack.containers.append(&mut containers_removed);

}

pub fn make_iteration_9001(stacks: &mut HashMap<StackID, Stack>, instruction: &Instruction) {

    let from_stack: &mut Stack = stacks.get_mut(&instruction.from_stack).unwrap();
    let mut containers_removed: Vec<ContainerID> = Vec::new();

    for _ in 0..instruction.count {

        let container: ContainerID = from_stack.containers.pop().unwrap();
        containers_removed.push(container);
    }

    containers_removed = containers_removed.into_iter().rev().collect();
    let to_stack: &mut Stack = stacks.get_mut(&instruction.to_stack).unwrap();
    to_stack.containers.append(&mut containers_removed);
}

// ================================================= IMPLEMENTATIONS =================================================


