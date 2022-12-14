mod read;
mod lib;

fn main() {

    let input: Vec<String> = read::read_lines("input.txt".to_owned());
    let filesystem: lib::FileSystem = lib::FileSystem::new(&input);

    // Part 1

    let threshold: usize = 100000;
    let sum_of_sizes_under_threshold: usize = filesystem.directories_under_threshold(threshold)
        .iter()
        .map(|(_, size)| *size)
        .sum();
    println!("\r📁 Sum of directories under threshold '{}': '{}' (Part 1)", threshold, sum_of_sizes_under_threshold);
    
    // Part 2
    let total_disk_space: usize = 70000000;
    let update_size: usize = 30000000;
    let (directory, size_freed): (&String, usize) = filesystem.get_directory_to_delete_for_update(total_disk_space, update_size);
    println!("\r📁 By deleting directory '{}': '{}' (Part 2)", directory, size_freed);

}