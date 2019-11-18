mod components;
mod entities;
mod resources;
mod states;
mod systems;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystemDesc, Processor},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir},
    Application, GameDataBuilder,
};
use components::{AnimationId, AnimationPrefabData};
use systems::{AnimationControlSystem, CollisionSystem, DirectionSystem, KinematicsSystem, PlayerKinematicsSystem, PlayerAnimationSystem};
use resources::{Map, Tileset};

fn main() -> amethyst::Result<()> {
    // start logging in amethyst
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let assets_dir = app_root.join("assets");
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<AnimationPrefabData>::default(), "scene_loader", &[])
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<Tileset>::new(), "tileset_processor", &[])
        .with(Processor::<Map>::new(), "map_processor", &[])
        .with(
            systems::ControlsSystem,
            "controls_system",
            &["input_system"],
        )
        .with(
            PlayerKinematicsSystem,
            "player_kinematics_system", 
            &["controls_system"],
        )
        .with(CollisionSystem, "collision_system", &["player_kinematics_system"])
        .with(
            KinematicsSystem,
            "kinematics_system",
            &["player_kinematics_system"],
        )
        .with(systems::MovePersonSystem, "move_person_system", &["kinematics_system"])
        .with(
            PlayerAnimationSystem,
            "player_animation_system",
            &[], // &["transformation_system"],
        )
        .with(
            AnimationControlSystem,
            "animation_control_system",
            &[
                "player_animation_system",
            ],
        )
        .with(
            DirectionSystem,
            "direction_system",
            &[], // &["transformation_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, states::GameState::default())?.build(game_data)?;
    game.run();

    Ok(())
}
