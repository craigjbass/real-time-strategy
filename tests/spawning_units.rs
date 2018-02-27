extern crate game_use_cases;

use game_use_cases::use_cases::*;
use game_use_cases::domain::*;
use game_use_cases::use_cases::boundary::*;
use game_use_cases::gateway::*;
use std::sync::mpsc::*;

struct MockPresenter {
    channel: std::sync::mpsc::SyncSender<view_units::PresentableUnit>
}

impl view_units::ViewUnitsPresenter for MockPresenter {
    fn present(&self, presentable_unit: view_units::PresentableUnit) {
        let channel = &self.channel;

        channel.send(presentable_unit).unwrap()
    }
}

#[test]
#[ignore]
fn spawning_units() {
    let spawn_unit = SpawnUnit::new();
    spawn_unit.execute();

    struct Placeholder {}
    impl UnitGateway for Placeholder {
        fn get_units_stream(&self) -> Receiver<Unit> {
            unimplemented!()
        }
    }
    let view_units = ViewUnits::new(Box::new(Placeholder {}));

    let (sender, receiver) = sync_channel(1);
    let mut presenter = MockPresenter { channel: sender };
    view_units.execute(&mut presenter);
    assert_eq!(receiver.try_recv().unwrap(), view_units::PresentableUnit { health: u8::max_value() });
}
