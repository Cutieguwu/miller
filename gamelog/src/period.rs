use crate::{Event, PlayHandle};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Period {
    pub start: Quarter,
    pub end: Option<Quarter>,
    pub events: Vec<Event>,
}

impl PlayHandle for Period {
    fn plays(&self) -> Vec<crate::Play> {
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Quarter {
    First,
    Second,
    Third,
    Fourth,
    Overtime(u8),
}
