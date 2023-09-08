mod models;

use models::{ Player, Character };

fn player_attack_with_target<T: Character, U: Character>(damager:&T, target:&mut U) {
    println!("Player {} attacking Player {}", damager.get_player_name(), target.get_player_name());
    let _ = target.damage_player(damager.attack());
    println!("Damaged Player {}", target.get_player_name());
    println!("New HP of {}: {}", target.get_player_name(), target.get_hp());
}

fn is_character_alive<T: Character>(player: &T) -> bool {
    match player.get_hp() {
        0 => {
            false
        }
        _ => {
            true
        }
    }
}

fn main() {
    println!("Traits Demo | YeXiuPH");
    let player0: Player = Player {
        player_name: "YeXiuPH".to_string(),
        health: 100,
        mana: 100,
        damage: 50,
        range: 10
    };
    
    let mut player1: Player = Player {
        player_name: "Dummy".to_string(),
        health: 100,
        mana: 100,
        damage:50,
        range: 10
    };

    player_attack_with_target(&player0, &mut player1);
    println!("Is Still Alive? : {}", is_character_alive(&player1));
    player_attack_with_target(&player0, &mut player1);
    println!("Is Still Alive? : {}", is_character_alive(&player1));
}
