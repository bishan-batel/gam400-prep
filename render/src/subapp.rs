use bevy_app::AppLabel;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(AppLabel, Debug, Eq, PartialEq, Hash, Clone)]
pub struct RenderApp;

#[derive(ScheduleLabel, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Render;
