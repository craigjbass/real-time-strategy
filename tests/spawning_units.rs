extern crate game_use_cases;

use game_use_cases::use_cases;

#[test]
fn spawning_units() {
    let a = use_cases::SpawnUnit::new();
    a.execute();
    return ();
}
