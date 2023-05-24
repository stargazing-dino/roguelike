use bevy::prelude::{Bundle, Component, IntoSystemConfig, Plugin};
use leafwing_input_manager::{prelude::InputManagerPlugin, InputManagerBundle};

use crate::{
    input::player_action::PlayerAction,
    movement::move_player::move_player,
    vision::{
        update_viewshed::update_viewshed, viewable_tiles_for_player::viewable_tiles_for_player,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(move_player)
            .add_system(update_viewshed.after(move_player))
            .add_system(viewable_tiles_for_player.after(update_viewshed));
    }
}

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
