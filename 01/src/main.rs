fn main() {
    let input = ::std::fs::read_to_string("input.txt").unwrap();
    let elves_items_calories = input.split("\r\n\r\n").collect::<Vec<&str>>();
    let mut three_highest: [u32; 3] = [0, 0, 0];

    for elf_item_calories in elves_items_calories {
        let elf_total_calories = elf_item_calories.split("\r\n").map(|item| item.parse::<u32>().unwrap()).sum();
        enter_position(&mut three_highest, elf_total_calories);
    }
    
    println!("Part one: {}", three_highest[0]);
    println!("Part two: {}", three_highest.iter().sum::<u32>());
}

fn enter_position(three_highest: &mut[u32; 3], calories: u32) {
    if calories > three_highest[0] {
        three_highest[2] = three_highest[1];
        three_highest[1] = three_highest[0];
        three_highest[0] = calories;
    }
    else if calories > three_highest[1] {
        three_highest[2] = three_highest[1];
        three_highest[1] = calories;
    }
    else if calories > three_highest[2] {
        three_highest[2] = calories;
    }
}