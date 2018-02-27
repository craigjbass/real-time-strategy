use gateway::UnitGateway;
use std::sync::{Arc, Mutex};

pub struct SpawnUnit {}

impl SpawnUnit {
    pub fn new(unit_gateway: Arc<Mutex<UnitGateway>>) -> Self {
        return SpawnUnit {};
    }

    pub fn execute(self) {}
}
