use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Instant;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SessionStatus {
    Running,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Serialize, Clone, Debug)]
pub struct TimerState {
    pub session_type: SessionType,
    pub duration_secs: u64,
    pub remaining_secs: u64,
    pub progress: f64,
    pub status: SessionStatus,
}

pub struct Timer {
    session_type: SessionType,
    duration_secs: u64,
    started_at: Option<Instant>,
    paused_at: Option<Instant>,
    total_paused_ms: u64,
    status: SessionStatus,
}

impl Timer {
    pub fn new(session_type: SessionType, duration_secs: u64) -> Self {
        Self {
            session_type,
            duration_secs,
            started_at: None,
            paused_at: None,
            total_paused_ms: 0,
            status: SessionStatus::Cancelled,
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
        self.paused_at = None;
        self.total_paused_ms = 0;
        self.status = SessionStatus::Running;
    }

    pub fn pause(&mut self) {
        if self.status == SessionStatus::Running {
            self.paused_at = Some(Instant::now());
            self.status = SessionStatus::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.status == SessionStatus::Paused {
            if let Some(paused_at) = self.paused_at.take() {
                self.total_paused_ms += paused_at.elapsed().as_millis() as u64;
            }
            self.status = SessionStatus::Running;
        }
    }

    pub fn stop(&mut self) {
        self.status = SessionStatus::Cancelled;
        self.started_at = None;
    }

    pub fn state(&self) -> TimerState {
        let remaining = self.remaining_secs();
        TimerState {
            session_type: self.session_type.clone(),
            duration_secs: self.duration_secs,
            remaining_secs: remaining,
            progress: if self.duration_secs > 0 {
                1.0 - (remaining as f64 / self.duration_secs as f64)
            } else {
                0.0
            },
            status: self.status.clone(),
        }
    }

    pub fn is_completed(&self) -> bool {
        self.status == SessionStatus::Running && self.remaining_secs() == 0
    }

    pub fn session_type(&self) -> &SessionType {
        &self.session_type
    }

    fn remaining_secs(&self) -> u64 {
        if let Some(started_at) = self.started_at {
            let elapsed_ms = started_at.elapsed().as_millis() as u64;
            let paused_ms = self.total_paused_ms
                + self.paused_at
                    .map(|p| p.elapsed().as_millis() as u64)
                    .unwrap_or(0);
            let elapsed_secs = (elapsed_ms.saturating_sub(paused_ms)) / 1000;
            self.duration_secs.saturating_sub(elapsed_secs)
        } else {
            self.duration_secs
        }
    }
}

pub struct TimerManager {
    timer: Mutex<Option<Timer>>,
}

impl TimerManager {
    pub fn new() -> Self {
        Self {
            timer: Mutex::new(None),
        }
    }

    pub fn start(&self, session_type: SessionType, duration_secs: u64) -> TimerState {
        let mut timer = self.timer.lock().unwrap();
        let mut t = Timer::new(session_type, duration_secs);
        t.start();
        let state = t.state();
        *timer = Some(t);
        state
    }

    pub fn pause(&self) -> Option<TimerState> {
        let mut timer = self.timer.lock().unwrap();
        timer.as_mut().map(|t| {
            t.pause();
            t.state()
        })
    }

    pub fn resume(&self) -> Option<TimerState> {
        let mut timer = self.timer.lock().unwrap();
        timer.as_mut().map(|t| {
            t.resume();
            t.state()
        })
    }

    pub fn stop(&self) -> Option<TimerState> {
        let mut timer = self.timer.lock().unwrap();
        timer.as_mut().map(|t| {
            t.stop();
            t.state()
        })
    }

    pub fn tick(&self) -> Option<TimerState> {
        let mut timer = self.timer.lock().unwrap();
        timer.as_mut().and_then(|t| {
            if t.is_completed() {
                t.status = SessionStatus::Completed;
            }
            Some(t.state())
        })
    }

    pub fn is_completed(&self) -> bool {
        let timer = self.timer.lock().unwrap();
        timer.as_ref().map(|t| t.is_completed()).unwrap_or(false)
    }

    pub fn session_type(&self) -> Option<SessionType> {
        let timer = self.timer.lock().unwrap();
        timer.as_ref().map(|t| t.session_type().clone())
    }
}
