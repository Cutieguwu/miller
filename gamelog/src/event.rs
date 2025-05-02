use crate::{Down, Play, Quarter, TerrainState};
use serde::Deserialize;

type Offence = Team;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Event {
    Kickoff(Offence),
    Play(Play),
    Turnover(Offence),
    Penalty(TerrainState),
    Score(ScorePoints),
    Quarter(Quarter),
}

impl Event {
    /// Returns the terrain delta between the self and given following events.
    /// Returns `None` if no delta can be calculated between
    /// `self` and following events.
    pub fn delta(&self, following: &Self) -> Option<i8> {
        let preceeding = self.to_play()?;
        let following = if let Event::Turnover(_) = following {
            // I should really just early return
            // but this is too funny to look at.
            None?
        } else {
            following.to_play()?
        };

        if following.down? == Down::First {
            if let TerrainState::Yards(yrds) = preceeding.terrain? {
                Some(yrds as i8)
            } else {
                None
            }
        } else {
            let a = if let TerrainState::Yards(yrds) = preceeding.terrain? {
                yrds
            } else {
                0_u8
            };

            let b = if let TerrainState::Yards(yrds) = following.terrain? {
                yrds
            } else {
                0_u8
            };

            Some(a as i8 - b as i8)
        }
    }

    /// Returns the team for variants which possess this attribute.
    /// Errors if `self` has no team attribute.
    pub fn team(&self) -> Option<Team> {
        match self {
            Self::Kickoff(team) => Some(team.to_owned()),
            Self::Turnover(team) => Some(team.to_owned()),
            _ => None,
        }
    }

    /// Returns the team for variants which possess this attribute.
    /// Returns `None` if `self` has no team attribute.
    pub fn quarter(&self) -> Option<Quarter> {
        if let Event::Quarter(quarter) = self {
            Some(quarter.to_owned())
        } else {
            None
        }
    }

    /// Converts an event into it's associated Play object, if there is one.
    fn to_play(self: &Event) -> Option<Play> {
        if let Event::Play(play) = self {
            if play.down.is_none() || play.terrain.is_none() {
                None
            } else {
                Some(play.to_owned())
            }
        } else if let Event::Kickoff(_) | Event::Turnover(_) = self {
            Some(Play::default())
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub enum Team {
    ArizonaState,
    #[deprecated(since = "0.2.0", note = "Team left the project.")]
    BoiseState,
    Colorado,
    Iowa,
    #[default]
    Nebraska,
    #[deprecated(since = "0.7.3", note = "Team left the project.")]
    SouthCarolina,
    Syracuse,
    TexasAnM,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub enum ScorePoints {
    #[default]
    Touchdown,
    FieldGoal,
    Safety,
    PatFail,
    PatTouchdown,
    PatFieldGoal,
    PatSafety,
}

impl ScorePoints {
    pub fn to_points(&self) -> u8 {
        match &self {
            Self::Touchdown => 6,
            Self::FieldGoal => 3,
            Self::Safety => 2,
            Self::PatFail => 0,
            Self::PatTouchdown => 2,
            Self::PatFieldGoal => 1,
            Self::PatSafety => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn delta() {
        let kickoff = Event::Kickoff(Team::Nebraska);

        let first_down = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::First),
            terrain: Some(TerrainState::Yards(10)),
        });

        let second_down = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::Second),
            terrain: Some(TerrainState::Yards(10)),
        });

        let third_down = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::Third),
            terrain: Some(TerrainState::Yards(13)),
        });

        let fourth_down = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::Fourth),
            terrain: Some(TerrainState::Yards(5)),
        });

        let penalty = Event::Penalty(TerrainState::Yards(15));

        let turnover = Event::Turnover(Team::Nebraska);

        let noned_down = Event::Play(Play {
            action: Action::Unknown,
            down: None,
            terrain: None,
        });

        let score = Event::Score(ScorePoints::default());

        let goal_line = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::First),
            terrain: Some(TerrainState::GoalLine),
        });

        let inches = Event::Play(Play {
            action: Action::Unknown,
            down: Some(Down::First),
            terrain: Some(TerrainState::Inches),
        });

        let quarter = Event::Quarter(Quarter::First);

        assert!(None == quarter.delta(&kickoff));
        assert!(None == quarter.delta(&first_down));
        assert!(None == quarter.delta(&second_down));
        assert!(None == quarter.delta(&third_down));
        assert!(None == quarter.delta(&fourth_down));
        assert!(None == quarter.delta(&turnover));
        assert!(None == quarter.delta(&penalty));
        assert!(None == quarter.delta(&goal_line));
        assert!(None == quarter.delta(&inches));
        assert!(None == quarter.delta(&score));
        assert!(None == quarter.delta(&quarter));

        assert!(10_i8 == kickoff.delta(&first_down).unwrap());
        assert!(0_i8 == kickoff.delta(&second_down).unwrap());
        assert!(None == kickoff.delta(&penalty));
        assert!(None == kickoff.delta(&score));

        assert!(10_i8 == first_down.delta(&kickoff).unwrap());
        assert!(10_i8 == first_down.delta(&first_down).unwrap());
        assert!(0_i8 == first_down.delta(&second_down).unwrap());
        assert!(None == first_down.delta(&turnover));
        assert!(None == first_down.delta(&penalty));
        assert!(None == first_down.delta(&score));
        assert!(10_i8 == first_down.delta(&goal_line).unwrap());
        assert!(10_i8 == first_down.delta(&inches).unwrap());
        assert!(None == first_down.delta(&noned_down));

        assert!(10_i8 == second_down.delta(&kickoff).unwrap());
        assert!(10_i8 == second_down.delta(&first_down).unwrap());
        assert!(-3_i8 == second_down.delta(&third_down).unwrap());
        assert!(None == second_down.delta(&turnover));
        assert!(None == second_down.delta(&penalty));
        assert!(None == second_down.delta(&score));
        assert!(10_i8 == second_down.delta(&goal_line).unwrap());
        assert!(10_i8 == second_down.delta(&inches).unwrap());
        assert!(None == second_down.delta(&noned_down));

        assert!(13_i8 == third_down.delta(&kickoff).unwrap());
        assert!(13_i8 == third_down.delta(&first_down).unwrap());
        assert!(8_i8 == third_down.delta(&fourth_down).unwrap());
        assert!(None == third_down.delta(&turnover));
        assert!(None == third_down.delta(&penalty));
        assert!(None == third_down.delta(&score));
        assert!(13_i8 == third_down.delta(&goal_line).unwrap());
        assert!(13_i8 == third_down.delta(&inches).unwrap());
        assert!(None == third_down.delta(&noned_down));

        assert!(5_i8 == fourth_down.delta(&kickoff).unwrap());
        assert!(5_i8 == fourth_down.delta(&first_down).unwrap());
        assert!(None == fourth_down.delta(&turnover));
        assert!(None == fourth_down.delta(&penalty));
        assert!(None == fourth_down.delta(&score));
        assert!(5_i8 == fourth_down.delta(&goal_line).unwrap());
        assert!(5_i8 == fourth_down.delta(&inches).unwrap());
        assert!(None == fourth_down.delta(&noned_down));

        assert!(10_i8 == turnover.delta(&first_down).unwrap());
        assert!(0_i8 == turnover.delta(&second_down).unwrap());
        assert!(None == turnover.delta(&turnover));
        assert!(None == turnover.delta(&penalty));
        assert!(None == turnover.delta(&score));
        assert!(10_i8 == turnover.delta(&goal_line).unwrap());
        assert!(10_i8 == turnover.delta(&inches).unwrap());
        assert!(None == turnover.delta(&noned_down));

        assert!(None == score.delta(&kickoff));
        assert!(None == score.delta(&first_down));
        assert!(None == score.delta(&second_down));
        assert!(None == score.delta(&third_down));
        assert!(None == score.delta(&fourth_down));
        assert!(None == score.delta(&turnover));
        assert!(None == score.delta(&penalty));
        assert!(None == score.delta(&goal_line));
        assert!(None == score.delta(&inches));
        assert!(None == score.delta(&score));

        assert!(None == goal_line.delta(&kickoff));
        assert!(None == goal_line.delta(&first_down));
        assert!(-10_i8 == goal_line.delta(&second_down).unwrap());
        assert!(-13_i8 == goal_line.delta(&third_down).unwrap());
        assert!(-5_i8 == goal_line.delta(&fourth_down).unwrap());
        assert!(None == goal_line.delta(&turnover));
        assert!(None == goal_line.delta(&penalty));
        assert!(None == goal_line.delta(&goal_line));
        assert!(None == goal_line.delta(&inches));
        assert!(None == goal_line.delta(&score));

        assert!(None == inches.delta(&kickoff));
        assert!(None == inches.delta(&first_down));
        assert!(-10_i8 == inches.delta(&second_down).unwrap());
        assert!(-13_i8 == inches.delta(&third_down).unwrap());
        assert!(-5_i8 == inches.delta(&fourth_down).unwrap());
        assert!(None == inches.delta(&turnover));
        assert!(None == inches.delta(&penalty));
        assert!(None == inches.delta(&goal_line));
        assert!(None == inches.delta(&inches));
        assert!(None == inches.delta(&score));

        assert!(None == noned_down.delta(&kickoff));
        assert!(None == noned_down.delta(&first_down));
        assert!(None == noned_down.delta(&second_down));
        assert!(None == noned_down.delta(&third_down));
        assert!(None == noned_down.delta(&fourth_down));
        assert!(None == noned_down.delta(&turnover));
        assert!(None == noned_down.delta(&penalty));
        assert!(None == noned_down.delta(&goal_line));
        assert!(None == noned_down.delta(&inches));
        assert!(None == noned_down.delta(&score));
    }
}
