fn main() {
    let input = ::std::fs::read_to_string("input.txt").unwrap();
    let rucksacks = input.split("\r\n").collect::<Vec<&str>>();
    let mut matches: Vec<char> = vec![];

    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
        for current_item in first_compartment.chars() {
            if second_compartment.contains(current_item) {
                matches.push(current_item);
            }
        }
    }

    println!("{:?}", matches);
}
