use ui_core::prelude::*;
use bevy::prelude::*;

pub fn scale_background(
    window: Query<&Window>,
    mut query: Query<(&mut Transform, &Handle<Image>), (With<UiBackground>, With<Ui>)>,
    images: Res<Assets<Image>>,
) {
    let win_event = window.single();
    let (width, height) = (win_event.width() as u32, win_event.height() as u32);

    if width == 0 || height == 0 {
        return;
    }

    for (mut transform, image_handle) in query.iter_mut() {
        let Some(image) = images.get(image_handle) else {
            return;
        };

        let image_aspect = image.size().x / image.size().y;
        let window_aspect = width / height;

        let scale = if window_aspect > image_aspect {
            // Window is wider, scale to match width
            UVec3::new(width / image.size().x, width / image.size().x, 1).as_vec3()
        } else {
            // Window is taller, scale to match height
            UVec3::new(height / image.size().y, height / image.size().y, 1).as_vec3()
        };

        transform.scale = scale;
    }
}