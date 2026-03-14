use lazy_static::lazy_static;
use enum_map::{enum_map, EnumMap, Enum};

use std::fmt::Display;
use std::collections::HashMap;

#[derive(Debug, Enum, Clone)]
pub enum Connection {
    NoConnection = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Left = 4,
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Connection::NoConnection => "NoConnection",
            Connection::Left => "Left",
            Connection::Right => "Right",
            Connection::Bottom => "Bottom",
            Connection::Top => "Top",
        })
    }
}

pub struct HasConnection {
    pub to_left: Connection,
    pub to_right: Connection,
    pub to_top: Connection,
    pub to_bottom: Connection,
}

impl HasConnection {
    pub fn as_map(&self) -> EnumMap<Connection, Connection> {
        enum_map! {
            Connection::Left => self.to_left.to_owned(),
            Connection::Right => self.to_right.to_owned(),
            Connection::Top => self.to_top.to_owned(),
            Connection::Bottom => self.to_bottom.to_owned(),
            Connection::NoConnection => Connection::NoConnection,
        }
    }
}

lazy_static! {
    pub static ref HAS_CONNECTION: HashMap<char, EnumMap<Connection, Connection>> = {
        let mut m: HashMap<char, EnumMap<Connection, Connection>> = HashMap::new();
        m.insert('S', HasConnection {
            to_left: Connection::Right,
            to_right: Connection::Left,
            to_bottom: Connection::Top,
            to_top: Connection::Bottom,
        }.as_map());
        m.insert('L', HasConnection {
            to_left: Connection::NoConnection,
            to_right: Connection::Top,
            to_bottom: Connection::NoConnection,
            to_top: Connection::Right,
        }.as_map());
        m.insert('J', HasConnection {
            to_left: Connection::Top,
            to_right: Connection::NoConnection,
            to_bottom: Connection::NoConnection,
            to_top: Connection::Left,
        }.as_map());
        m.insert('F', HasConnection {
            to_left: Connection::NoConnection,
            to_right: Connection::Bottom,
            to_bottom: Connection::Right,
            to_top: Connection::NoConnection,
        }.as_map());
        m.insert('7', HasConnection {
            to_left: Connection::Bottom,
            to_right: Connection::NoConnection,
            to_bottom: Connection::Left,
            to_top: Connection::NoConnection,
        }.as_map());
        m.insert('.', HasConnection {
            to_left: Connection::NoConnection,
            to_right: Connection::NoConnection,
            to_bottom: Connection::NoConnection,
            to_top: Connection::NoConnection,
        }.as_map());
        m.insert('-', HasConnection {
            to_left: Connection::Right,
            to_right: Connection::Left,
            to_bottom: Connection::NoConnection,
            to_top: Connection::NoConnection,
        }.as_map());
        m.insert('|', HasConnection {
            to_left: Connection::NoConnection,
            to_right: Connection::NoConnection,
            to_bottom: Connection::Top,
            to_top: Connection::Bottom,
        }.as_map());
        m
    };
}