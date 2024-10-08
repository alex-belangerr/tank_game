use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Step{
    #[default]
    LoadMap = 0,
    GenerateMap = 1,
    Finished = 2
}