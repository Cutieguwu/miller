use serde::Deserialize;
use strum::EnumIter;

#[derive(Debug, Deserialize, Clone, PartialEq, EnumIter)]
pub enum Quarter {
    First,
    Second,
    Third,
    Fourth,
    Overtime(u8),
}
