use bevy::prelude::{Bundle, Component};
use leafwing_input_manager::InputManagerBundle;

use crate::input_managers::players::PlayerAction;

// TODO: I don't think this will be accurate with matchbox code
/// This component is used to mark an entity as playable.
#[derive(Component, Debug, Copy, Clone)]
pub enum Player {
    One,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    #[bundle]
    pub input_manager: InputManagerBundle<PlayerAction>,
}
