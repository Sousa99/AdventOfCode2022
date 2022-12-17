mod read;
mod lib;

fn main() {

    let input: Vec<String> = read::read_lines("input.txt".to_owned());
    let mut device: lib::HandheldDeviceSetup = lib::HandheldDeviceSetup::new(input, vec![('X', 1)]);
    // Part 1

    let signal_strength: i32 = device.get_signal_strength(20, 40, 'X');
    println!("\r📡 Signal strength: '{}' (Part 1)", signal_strength);

    println!();
    
    // Part 2
    let display: Vec<Vec<char>> = device.display_screen(40, 1, 'X');
    println!("\r📡 Display screen: (Part 2)");
    display.into_iter()
        .map(|chars| chars.into_iter().collect())
        .for_each(|line: String| println!("{}", line));
    
}