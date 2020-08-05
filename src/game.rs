use crate::players::{Spymaster, FieldOperative};
use crate::map::{Map, Color};


pub fn opposite_player(color: Color) -> Color {
    match color {
        Color::Red => Color::Blue,
        Color::Blue => Color::Red,
        _ => panic!("Impossible player type!")
    }
}

pub fn get_actions(spymaster: &mut dyn Spymaster, field_op: &mut dyn FieldOperative, map: &Map) -> Vec<String>{
    let hint = spymaster.give_hint(map);
    println!("Spymaster gave {}, {} as hint.", hint.word, hint.count);
    field_op.choose_words(&hint, &map.remaining_words()).iter().map(|&x| x.to_string()).collect()
}

pub fn check_if_lost(current_player: Color, guesses: &[String], map: &mut Map) -> bool {
    for word in guesses {
        let color = map.reveal_cell(word);
        println!("Guess {} was {}", word, color);
        if color == Color::Black {
            return true;
        }
        if color != opposite_player(current_player) {
            return false;
        }
    }
    false
}

pub fn game(red_spymaster: &mut dyn Spymaster, red_field_op: &mut dyn FieldOperative,
            blue_spymaster: &mut dyn Spymaster, blue_field_op: &mut dyn FieldOperative,
            map: &mut Map) -> Color {
    let mut current_color = Color::Red;
    loop {
        println!("The turn of player {}", current_color);
        let guesses;
        if current_color == Color::Red {
            guesses = get_actions(red_spymaster, red_field_op, &map);
        } else {
            guesses = get_actions(blue_spymaster, blue_field_op, &map);
        };
        println!("{:?}", &guesses);
        if check_if_lost(Color::Blue, &guesses, map) {
            return opposite_player(current_color);
        }
        println!("The map is now: {}", map);
        if map.is_over() {
            return current_color;
        }
        current_color = opposite_player(current_color);
    }
}