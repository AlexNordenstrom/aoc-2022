const OFFSET_UPPERCASE: u8 = 38;
const OFFSET_LOWERCASE: u8 = 96;

fn main() {
    let input = ::std::fs::read_to_string("input.txt").unwrap();
    let rucksacks = input.split("\r\n").collect::<Vec<&str>>();
    let mut matches: Vec<u32> = vec![];

    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
        println!("{:?} {:?}", first_compartment, second_compartment);
        for current_item in first_compartment.chars() {
            if second_compartment.contains(current_item) {
                let offset: u32;
                if current_item.is_uppercase() {
                    offset = OFFSET_UPPERCASE as u32;
                }
                else {
                    offset = OFFSET_LOWERCASE as u32;
                }
                matches.push((current_item as u32) - offset);
            }
        }
    }
    println!("Part one: {}", matches.iter().sum::<u32>());
}
