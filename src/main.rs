use bevy::prelude::*;

mod assets;
mod window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(window::WindowInitPlugin)
        .add_plugins(assets::MenuInitPlugin)
        .run();
}

pub struct Card {
    level: CardLevel,
    suit: CardSuit,
    location: CardLocation,
}

#[derive(Component)]
pub enum CardSuit {
    Joker,
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
#[derive(Component)]
pub enum CardLevel {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Joker = 14,
}
#[derive(Component)]
pub enum ChipValue {
    White = 1,
    Red = 5,
    Blue = 10,
    Green = 25,
    Black = 100
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Default, States)]
pub enum CardLocation {
    #[default]
    Deck,
    Hand,
    Field,
    Grave,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Battle,
    Loading,
    SettingsMenu,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Default, States)]
pub enum Phase {
    #[default]
    Prep,
    Draw,
    Discard,
    Reveal,
    Wrapup,
}
