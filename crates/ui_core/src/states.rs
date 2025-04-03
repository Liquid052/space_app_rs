use bevy::prelude::*;
use space_engine::prelude::{AppState, BuildingStates, LoadingStates};


#[derive(SubStates, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[source(AppState = AppState::Menu)]
pub struct SettingsState(pub bool);

#[derive(Default,Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct InSettings;

impl ComputedStates for InSettings {
    type SourceStates = SettingsState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            SettingsState(true) => Some(Self),
            _ => None
        }
    }
}

#[derive(SubStates, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[source(AppState = AppState::Menu)]
pub struct LoadGameState(pub bool);

#[derive(Default,Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct InLoadGame;

impl ComputedStates for InLoadGame {
    type SourceStates = LoadGameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            LoadGameState(true) => Some(Self),
            _ => None
        }
    }
}

#[derive(Default,Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct InMainMenu;

impl ComputedStates for InMainMenu {
    type SourceStates = (AppState, Option<InLoadGame>, Option<InSettings>);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            (AppState::Menu, None, None) => Some(Self),
            _ => None
        }
    }
}

#[derive(Default,Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct InMenu;

impl ComputedStates for InMenu {
    type SourceStates = AppState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            AppState::Menu => Some(Self),
            _ => None
        }
    }
}

#[derive(Default,Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct InLoadingOrMenu;

impl ComputedStates for InLoadingOrMenu {
    type SourceStates = (Option<AppState>, Option<InMenu>);

    fn compute((app_state, in_menu): Self::SourceStates) -> Option<Self> {
        let Some(app_state) = app_state else {
            return Some(Self);
        };

        if app_state == AppState::Menu {
            return Some(Self);
        }

        None
    }
}