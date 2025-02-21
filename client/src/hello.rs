use crate::components::Name;
use crate::components::PlayerCharacter;
use crate::debug::DebugState;
use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

pub fn add_people(mut commands: Commands) {
    commands.spawn((PlayerCharacter, Name("Stefanie".to_string())));
    commands.spawn((PlayerCharacter, Name("Laura".to_string())));
    commands.spawn((PlayerCharacter, Name("Lucy".to_string())));
}

pub fn greet_people(
    time: Res<Time>,
    mut state: ResMut<DebugState>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<PlayerCharacter>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            state.debug_text = format!("{}hello {}!\n", state.debug_text, name.0);
            state.debug_color = Color::WHITE;
        }
    }
}

pub fn update_people(mut query: Query<&mut Name, With<PlayerCharacter>>) {
    for mut name in &mut query {
        if name.0 == "Stefanie" {
            name.0 = "Stefieany".to_string();
            break;
        }
    }
}
