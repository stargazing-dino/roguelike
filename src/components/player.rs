use bevy::prelude::{Bundle, Component, Gamepad, GamepadButtonType, KeyCode};
use leafwing_input_manager::{prelude::InputMap, Actionlike, InputManagerBundle};

/// This component is used to mark an entity as playable.
#[derive(Component, Debug)]
pub enum Player {
    One,
    Two,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    #[bundle]
    pub input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    pub fn input_map(playable: Player) -> InputMap<PlayerAction> {
        let mut input_map = match playable {
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
            Player::Two => InputMap::new([
                (KeyCode::A, PlayerAction::Left),
                (KeyCode::D, PlayerAction::Right),
                (KeyCode::W, PlayerAction::Up),
                (KeyCode::S, PlayerAction::Down),
            ])
            .set_gamepad(Gamepad { id: 1 })
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
