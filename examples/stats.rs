use std::time::Duration;

use warcraft3_stats_observer::ObserverHandle;

fn main() {
    let od = match ObserverHandle::new() {
        Ok(od) => od,
        Err(e) => {
            eprintln!("Error opening observer API. Is Warcraft3 running? Error: {e:?}");
            return;
        }
    };

    println!("version: {}", { od.version });
    println!("refresh rate: {}", { od.refresh_rate });

    let game = &od.game;

    println!("in game: {:?}", game.in_game);
    println!(
        "time: {:02}:{:02}",
        game.time().as_secs() / 60,
        game.time().as_secs() % 60
    );
    println!("active player count: {:?}", game.active_player_count);
    println!("game name: {}", game.game_name);
    println!("map name: {}", game.map_name);

    loop {
        for player in od.players.iter().take(od.game.active_player_count as usize) {
            println!("{} has {} gold {} lumber", player.name, { player.gold }, {
                player.lumber
            });
        }
        println!();
        std::thread::sleep(Duration::from_millis(od.refresh_rate as u64));
    }
}
