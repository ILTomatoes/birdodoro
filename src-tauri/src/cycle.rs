use serde::{Deserialize, Serialize};
use crate::timer::SessionType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Serialize, Clone, Debug)]
pub struct CycleState {
    pub completed_pomodoros: u32,
    pub current_phase: Phase,
    pub cycle_count: u32,
}

pub struct WorkCycle {
    completed_pomodoros: u32,
    cycle_count: u32,
    pomodoros_per_cycle: u32,
}

impl WorkCycle {
    pub fn new(pomodoros_per_cycle: u32) -> Self {
        Self {
            completed_pomodoros: 0,
            cycle_count: 0,
            pomodoros_per_cycle: pomodoros_per_cycle.max(1),
        }
    }

    pub fn set_pomodoros_per_cycle(&mut self, count: u32) {
        self.pomodoros_per_cycle = count.max(1);
    }

    pub fn record_completed(&mut self) -> Phase {
        self.completed_pomodoros += 1;
        if self.completed_pomodoros >= self.pomodoros_per_cycle {
            self.completed_pomodoros = 0;
            self.cycle_count += 1;
            Phase::LongBreak
        } else {
            Phase::ShortBreak
        }
    }

    pub fn current_phase(&self) -> Phase {
        Phase::Work
    }

    pub fn reset(&mut self) {
        self.completed_pomodoros = 0;
        self.cycle_count = 0;
    }

    pub fn state(&self) -> CycleState {
        CycleState {
            completed_pomodoros: self.completed_pomodoros,
            current_phase: self.current_phase(),
            cycle_count: self.cycle_count,
        }
    }

    pub fn next_session_type(&self) -> SessionType {
        if self.completed_pomodoros + 1 >= self.pomodoros_per_cycle {
            SessionType::LongBreak
        } else {
            SessionType::ShortBreak
        }
    }
}
