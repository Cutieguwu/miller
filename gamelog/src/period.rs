use crate::{Event, Play, PlayHandle};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Period {
    pub start: Quarter,
    pub end: Option<Quarter>,
    pub events: Vec<Event>,
}

impl PlayHandle for Period {
    fn plays(&self) -> Vec<Play> {
        self.events
            .iter()
            .filter_map(|event| {
                if let Event::Play(play) = event {
                    Some(play.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Period {
    pub fn deltas(&self) -> Vec<i8> {
        let len = self.events.len() - 1;
        let mut idx: usize = 0;
        let mut deltas: Vec<i8> = vec![];

        while idx < len {
            if let Some(value) = self.events[idx].delta(&self.events[idx + 1]) {
                deltas.push(value);
            }

            idx += 1
        }

        deltas
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Quarter {
    First,
    Second,
    Third,
    Fourth,
    Overtime(u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Action, Down, Team, TerrainState};

    #[test]
    fn deltas() {
        let period = Period {
            start: Quarter::First,
            end: None,
            events: vec![
                Event::Kickoff(Team::Nebraska),
                Event::Play(Play {
                    action: Action::Unknown,
                    down: Some(Down::First),
                    terrain: Some(TerrainState::Yards(10)),
                }),
                Event::Play(Play {
                    action: Action::Unknown,
                    down: Some(Down::Second),
                    terrain: Some(TerrainState::Yards(13)),
                }),
                Event::Play(Play {
                    action: Action::Unknown,
                    down: Some(Down::Third),
                    terrain: Some(TerrainState::Yards(8)),
                }),
                Event::Turnover(Team::Nebraska),
                Event::Play(Play {
                    action: Action::Unknown,
                    down: Some(Down::First),
                    terrain: Some(TerrainState::Yards(10)),
                }),
                Event::Play(Play {
                    action: Action::Unknown,
                    down: Some(Down::First),
                    terrain: Some(TerrainState::Yards(10)),
                }),
            ],
        };

        let expected: Vec<i8> = vec![10, -3, 5, 10];

        assert!(period.deltas() == expected)
    }
}
