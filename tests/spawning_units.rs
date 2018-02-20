extern crate game_use_cases;

use game_use_cases::use_cases;
use std::sync::mpsc::*;

struct MockPresenter {
    channel: std::sync::mpsc::SyncSender<use_cases::PresentableUnit>
}

impl use_cases::ViewUnitsPresenter for MockPresenter {
    fn present(&self, presentable_unit: use_cases::PresentableUnit) {
        let channel = &self.channel;

        channel.send(presentable_unit).unwrap()
    }
}

#[test]
#[ignore]
fn spawning_units() {
    let spawn_unit = use_cases::SpawnUnit::new();
    spawn_unit.execute();

    struct Placeholder {}
    impl use_cases::UnitGateway for Placeholder {
        fn get_units_stream(&self) -> Receiver<use_cases::Unit> {
            unimplemented!()
        }
    }
    let view_units = use_cases::ViewUnits::new(Box::new(Placeholder {}));

    let (sender, receiver) = sync_channel(1);
    let mut presenter = MockPresenter { channel: sender };
    view_units.execute(&mut presenter);
    assert_eq!(receiver.try_recv().unwrap(), use_cases::PresentableUnit { health: u8::max_value() });
}
