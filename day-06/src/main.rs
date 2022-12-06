
mod read;
mod lib;

fn main() {

    let input: Vec<char> = read::read_lines("input.txt".to_owned())
        .pop().unwrap()
        .chars().collect();
    
    let device: lib::Device = lib::Device::new(input);

    // Part 1

    let start_transmission_marker_position: usize = device.find_marker_start_transmission().unwrap();
    println!("\r📟 Start transmission marker detected in position: '{}' (Part 1)", start_transmission_marker_position);
    
    // Part 2

    let start_message_marker_position: usize = device.find_marker_start_message().unwrap();
    println!("\r📟 Start message marker detected in position: '{}' (Part 2)", start_message_marker_position);

}