pub fn run() {
    const INPUT: &'static str = include_str!("input.txt");
    let snacks: Vec<&str> = INPUT.lines().collect();
    // println!("Rust is fun! {:?}", lines);

    // Part 1
    let mut total_cals_per_elf: Vec<u32> = vec![];
    let mut current_elf_cals: Vec<u32> = vec![];
    for snack_calories in snacks {
        if snack_calories != "" {
            let x = snack_calories.to_string().parse::<u32>().unwrap();
            current_elf_cals.push(x);
        } else {
            total_cals_per_elf.push(current_elf_cals.iter().sum());
            current_elf_cals = vec![];
        }
    }
    // push the last elf
    total_cals_per_elf.push(current_elf_cals.iter().sum());
    println!("Part 1: {:?}", total_cals_per_elf.iter().max().unwrap());

    // Part 2
    total_cals_per_elf.sort();

    let top_elfs_count = match total_cals_per_elf.len() {
        0..=2 => total_cals_per_elf.len(),
        _ => 3,
    };

    let total_top_cals = total_cals_per_elf.splice((total_cals_per_elf.len() -top_elfs_count)..total_cals_per_elf.len(), []).collect::<Vec<_>>();

    println!("Part 2: {}", total_top_cals.iter().sum::<u32>());
}
