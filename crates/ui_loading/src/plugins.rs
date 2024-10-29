use decay_engine::prelude::*;
use bevy::prelude::*;
use crate::prelude::*;
use crate::systems::*;

#[derive(Default)]
pub struct UiLoadingPlugin {
    pub config: LoadingUiConfig,
}

impl Plugin for UiLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config)
            .add_systems(OnEnter(Loading), (setup, setup_copyright))
            .add_systems(Update, update_loading_ui
                .run_if(state_changed::<LoadingStates>
                    .and_then(not(in_state(LoadingStates::Finished)))
                )
            )
            .add_systems(OnEnter(BuildingStates::Building), update_ui_building)
            .add_systems(PostUpdate, stall_finished_build_ui
                .run_if(in_state(BuildingStates::Building))
            )
            .add_systems(OnExit(BuildingStates::Building), (cleanup_resources, cleanup_ui, update_cam));
    }
}