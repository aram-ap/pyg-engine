/// Mesh Rendering Demo
///
/// Demonstrates GameObject + MeshComponent rendering with:
/// - Transform (position, scale, rotation)
/// - Layer + z-index ordering
/// - Solid-color mesh
/// - Textured mesh (with optional color tint)
///
/// To run:
/// `cargo run --example mesh_demo --no-default-features`
use pyg_engine_native::core::{
    Engine, FullscreenMode, GameObject, MeshComponent, MeshGeometry, WindowConfig,
};
use pyg_engine_native::types::{Color, Vec2};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    let window_config = WindowConfig::new()
        .with_title("PyG Engine - Mesh Demo")
        .with_size(1280, 720)
        .with_fullscreen(FullscreenMode::None)
        .with_background_color(Color::DARK_GRAY)
        .with_redraw_on_change_only(true);

    // NOTE: Positions and mesh dimensions are currently in clip-space units.
    let mut background = GameObject::new_named("Background".to_string());
    background.transform_mut().set_scale(Vec2::new(1.0, 1.0));
    background.add_mesh_component(
        MeshComponent::new("background")
            .with_geometry(MeshGeometry::rectangle(1.8, 1.2))
            .with_fill_color(Some(Color::rgb(20, 24, 32)))
            .with_layer(0)
            .with_z_index(-0.8),
    );

    let mut solid_quad = GameObject::new_named("SolidQuad".to_string());
    solid_quad
        .transform_mut()
        .set_position(Vec2::new(-0.35, 0.1));
    solid_quad.transform_mut().set_scale(Vec2::new(0.45, 0.45));
    solid_quad.transform_mut().set_rotation(0.3);
    solid_quad.add_mesh_component(
        MeshComponent::new("solid_quad")
            .with_geometry(MeshGeometry::rectangle(1.0, 1.0))
            .with_fill_color(Some(Color::ORANGE))
            .with_layer(1)
            .with_z_index(0.1),
    );

    let mut textured_quad = GameObject::new_named("TexturedQuad".to_string());
    textured_quad
        .transform_mut()
        .set_position(Vec2::new(0.35, -0.1));
    textured_quad
        .transform_mut()
        .set_scale(Vec2::new(0.45, 0.45));
    textured_quad.transform_mut().set_rotation(-0.2);
    textured_quad.add_mesh_component(
        MeshComponent::new("textured_quad")
            .with_geometry(MeshGeometry::rectangle(1.0, 1.0))
            .with_fill_color(Some(Color::WHITE))
            .with_image_path(Some("images/1.png".to_string()))
            .with_layer(2)
            .with_z_index(0.3),
    );

    engine.add_game_object(background);
    engine.add_game_object(solid_quad);
    engine.add_game_object(textured_quad);

    engine.run(window_config)?;
    Ok(())
}
