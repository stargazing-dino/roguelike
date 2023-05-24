use bevy::prelude::{Gamepad, GamepadButtonType, KeyCode};
use leafwing_input_manager::{prelude::InputMap, Actionlike};

use crate::plugins::player::Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Left,
    Right,
    Down,
    Up,
}

impl PlayerAction {
    pub fn input_map_for(player: Player) -> InputMap<PlayerAction> {
        let mut input_map = match player {
            Player::One => InputMap::new([
                (KeyCode::Left, PlayerAction::Left),
                (KeyCode::Right, PlayerAction::Right),
                (KeyCode::Up, PlayerAction::Up),
                (KeyCode::Down, PlayerAction::Down),
            ])
            // This is a quick and hacky solution:
            // you should coordinate with the `Gamepads` resource to determine the correct gamepad for each playable
            // and gracefully handle disconnects
            // Note that this step is not required:
            // if it is skipped all input maps will read from all connected gamepads
            .set_gamepad(Gamepad { id: 0 })
            .build(),
        };

        // Each playable will use the same gamepad controls, but on seperate gamepads
        input_map.insert_multiple([
            (GamepadButtonType::DPadLeft, PlayerAction::Left),
            (GamepadButtonType::DPadRight, PlayerAction::Right),
            (GamepadButtonType::DPadUp, PlayerAction::Up),
            (GamepadButtonType::South, PlayerAction::Down),
        ]);

        input_map
    }
}
