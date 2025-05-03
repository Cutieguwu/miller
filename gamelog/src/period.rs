use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, Deserialize, Clone, PartialEq, EnumIter, Default)]
pub enum Quarter {
    #[default]
    First,
    Second,
    Third,
    Fourth,
    Overtime(u8),
}
