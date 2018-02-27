use gateway::UnitGateway;
use std::sync::{Arc, Mutex};

pub struct SpawnUnit {
    unit_gateway: Arc<Mutex<UnitGateway>>
}

impl SpawnUnit {
    pub fn new(unit_gateway: Arc<Mutex<UnitGateway>>) -> Self {
        return SpawnUnit { unit_gateway };
    }

    pub fn execute(&self) {}
}
