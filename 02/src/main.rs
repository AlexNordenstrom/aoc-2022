fn main() {
    let input = ::std::fs::read_to_string("input.txt").unwrap();
    let plays = input.split("\r\n").collect::<Vec<&str>>();

    let mut total_value: u32 = 0;
    let mut fixed_value: u32 = 0;
    for play in plays {
        let (temp_tot, temp_fix) = get_hand_value(play);
        total_value += temp_tot;
        fixed_value += temp_fix;
    }

    println!("Part one: {}", total_value);
    println!("Part two: {}", fixed_value);
}

fn get_hand_value(hand: &str) -> (u32, u32) {
    let my_hand = &hand[2..];
    let opponent_hand = &hand[..1];

    let fixed_hand = get_fixed_hand(opponent_hand, my_hand);

    let score = get_hand_score(my_hand, opponent_hand);
    let fixed_score = get_hand_score(fixed_hand, opponent_hand);

    return (score, fixed_score);
}

fn get_hand_score(my_hand: &str, opponent_hand: &str) -> u32 {
    let win_score = 6;
    let base_score_x = 1;
    let base_score_y = 2;
    let base_score_z = 3;

    let mut score = 0;

    if my_hand == "X" {
        score += base_score_x;
        match opponent_hand {
            "A" => {
                score += 3;
            },
            "C" => {
                score += 6;
            },
            "B" | _ => {}
        }
    }
    else if my_hand == "Y" {
        score += base_score_y;
        match opponent_hand {
            "A" => {
                score += 6;
            },
            "B" => {
                score += 3;
            },
            "C" | _ => {}
        }
    }
    else if my_hand == "Z" {
        score += base_score_z;
        match opponent_hand {
            "B" => {
                score += 6;
            },
            "C" => {
                score += 3;
            },
            "A" | _ => {}
        }
    }

    return score;
}

fn get_fixed_hand<'a>(opponent_hand: &'a str, result: &'a str) -> &'a str {
    match result {
        "X" => {
            if opponent_hand == "A" {
                return "Z";
            }
            else if opponent_hand == "B" {
                return "X";
            }
            else if opponent_hand == "C" {
                return "Y";
            }
        },
        "Y" => {
            if opponent_hand == "A" {
                return "X";
            }
            else if opponent_hand == "B" {
                return "Y";
            }
            else if opponent_hand == "C" {
                return "Z";
            }
        },
        "Z" => {
            if opponent_hand == "A" {
                return "Y";
            }
            else if opponent_hand == "B" {
                return "Z";
            }
            else if opponent_hand == "C" {
                return "X";
            }
        },
        _ => {}
    }

    return "A";
}