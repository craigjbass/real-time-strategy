pub struct SpawnUnit {}

impl SpawnUnit {
    pub fn new() -> Self {
        return SpawnUnit {};
    }

    pub fn execute(self) {}
}


use std::sync::mpsc::*;
use std::thread;

#[derive(PartialEq, Eq, Debug)]
pub struct PresentableUnit {
    pub health: u8
}

pub trait ViewUnitsPresenter {
    fn present(&self, presentable_unit: PresentableUnit);
}

pub struct Unit {}

pub trait UnitGateway {
    fn get_units_stream(&self) -> Receiver<Unit>;
}

pub struct ViewUnits {
    unit_gateway: Box<UnitGateway>
}

impl ViewUnits {
    pub fn new(unit_gateway: Box<UnitGateway>) -> Self {
        return ViewUnits { unit_gateway };
    }

    pub fn execute(self, presenter: &mut ViewUnitsPresenter) {
        let result = self.unit_gateway.get_units_stream().recv();
        if !result.is_err() {
            presenter.present(PresentableUnit { health: u8::max_value() })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct SpyPresenter {
        last_presentable_unit: Cell<Option<PresentableUnit>>
    }

    impl SpyPresenter {
        fn new() -> Self {
            return SpyPresenter { last_presentable_unit: Cell::new(None) };
        }

        fn get_last_presentable_unit(&self) -> Option<PresentableUnit> {
            return self.last_presentable_unit.take();
        }
    }

    impl ViewUnitsPresenter for SpyPresenter {
        fn present(&self, presentable_unit: PresentableUnit) {
            self.last_presentable_unit.set(Some(presentable_unit))
        }
    }


    struct EmptyUnitsGatewayStub {}

    impl UnitGateway for EmptyUnitsGatewayStub {
        fn get_units_stream(&self) -> Receiver<Unit> {
            let (_, receiver) = channel();
            return receiver;
        }
    }

    struct OneUnitGatewayStub {}

    impl UnitGateway for OneUnitGatewayStub {
        fn get_units_stream(&self) -> Receiver<Unit> {
            let (sender, receiver) = channel();
            sender.send(Unit {});
            return receiver;
        }
    }


    #[test]
    fn can_present_no_units() {
        let gateway = EmptyUnitsGatewayStub {};

        let use_case = ViewUnits::new(Box::new(gateway));
        let mut presenter = SpyPresenter::new();
        use_case.execute(&mut presenter);

        assert_eq!(
            presenter.get_last_presentable_unit(),
            None
        );
    }

    #[test]
    fn can_present_a_unit() {
        let gateway = OneUnitGatewayStub {};
        let use_case = ViewUnits::new(Box::new(gateway));
        let mut presenter = SpyPresenter::new();
        use_case.execute(&mut presenter);

        assert_eq!(
            presenter.get_last_presentable_unit().unwrap(),
            PresentableUnit { health: u8::max_value() }
        );
    }
}