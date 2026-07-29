use entering_sleep_mode_command::{
    Animation, CosmosWorld, EngineRuntime, EntityLogic, InputAction, NodeState, Prefab, RigidBody,
    Renderable, Script, ScriptCommand, System, TileKind, Transform, Vector4,
};

#[test]
fn entering_sleep_mode_enables_sleep_state() {
    let mut runtime = EngineRuntime::new();

    let response = runtime.run_command("Enter Sleep Mode");

    assert!(response.sleep_active);
    assert!(response.message.contains("Sleep mode active"));
}

#[test]
fn build_command_spawns_entities_and_advances_world() {
    let mut runtime = EngineRuntime::new();

    let response = runtime.run_command("Build");

    assert!(response.message.contains("BUILD"));
    assert!(runtime.world.entity_count() >= 2);
}

#[test]
fn engine_can_spawn_entities_and_render_a_frame() {
    let mut runtime = EngineRuntime::new();
    runtime.spawn_entity(
        Transform::new(Vector4::new(2.0, 0.0, 0.0, 0.0), 1.5),
        Some(RigidBody::new(Vector4::new(0.2, 0.0, 0.0, 0.0), Vector4::zero(), 15.0)),
        Some(EntityLogic::new(NodeState::AutonomousEntity, 0.6, 0.0)),
        Some(Renderable::new("star")),
    );

    let frame = runtime.render_frame();

    assert_eq!(frame.entity_count, 1);
    assert!(frame.renderables.contains(&"star".to_string()));
    assert_eq!(frame.scene_name, "cosmos".to_string());
}

#[test]
fn stepping_the_engine_advances_frame_count_and_motion() {
    let mut runtime = EngineRuntime::new();
    runtime.run_command("Build");

    runtime.step(0.25);
    let frame = runtime.render_frame();

    assert!(runtime.frame_count >= 1);
    assert!(frame.frame_number >= 1);
    assert!(frame.entity_count >= 2);
}

#[test]
fn runtime_can_manage_named_scenes_and_render_text() {
    let mut runtime = EngineRuntime::new();
    runtime.create_scene("nebula");

    let switched = runtime.load_scene("nebula");
    let rendered = runtime.render_to_string();

    assert!(switched);
    assert!(rendered.contains("cosmos") || rendered.contains("nebula"));
}

#[test]
fn event_log_tracks_scene_and_entity_activity() {
    let mut runtime = EngineRuntime::new();
    runtime.create_scene("nebula");
    runtime.load_scene("nebula");

    let entity_id = runtime.spawn_entity(
        Transform::new(Vector4::new(0.0, 0.0, 0.0, 0.0), 1.0),
        None,
        None,
        Some(Renderable::new("orb")),
    );
    runtime.trigger_event(entity_id);

    assert!(runtime.event_log.events.iter().any(|event| matches!(event, entering_sleep_mode_command::EngineEvent::SceneCreated(_))));
    assert!(runtime.event_log.events.iter().any(|event| matches!(event, entering_sleep_mode_command::EngineEvent::EntitySpawned(id) if *id == entity_id)));
    assert!(runtime.event_log.events.iter().any(|event| matches!(event, entering_sleep_mode_command::EngineEvent::EntityUpdated(id) if *id == entity_id)));
}

#[test]
fn engine_can_place_objects_and_process_input() {
    let mut runtime = EngineRuntime::new();
    let entity_id = runtime.place_object("player", 3.0, 4.0);
    assert_eq!(runtime.world.entity_count(), 1);

    runtime.handle_input(InputAction::Jump, true);
    runtime.handle_input(InputAction::MoveRight, true);

    assert!(runtime.input_state.is_pressed(InputAction::Jump));
    assert!(runtime.input_state.is_pressed(InputAction::MoveRight));
    assert!(runtime.world.entity_count() >= 1);
    assert_eq!(runtime.world.transforms.contains_key(&entity_id), true);
}

#[test]
fn engine_can_build_tilemaps_and_detect_collisions() {
    let mut runtime = EngineRuntime::new();
    let tile_map = runtime.build_level(8, 6);

    assert_eq!(tile_map.get_tile(0, 5), TileKind::Ground);
    assert_eq!(tile_map.get_tile(2, 4), TileKind::Platform);
    assert!(runtime.world.collides_with_tile(2.0, 4.0));
    assert!(!runtime.world.collides_with_tile(2.0, 2.0));
}

#[test]
fn engine_can_register_prefabs_and_focus_camera() {
    let mut runtime = EngineRuntime::new();
    runtime.register_prefab(Prefab::new("player", "player-sprite"));
    let spawned = runtime.spawn_prefab("player", 1.5, 2.5);

    runtime.focus_camera(10.0, 8.0);

    assert!(spawned.is_some());
    assert_eq!(runtime.camera.x, 10.0);
    assert_eq!(runtime.camera.y, 8.0);
}

#[test]
fn engine_can_save_and_reload_scene_data() {
    let mut runtime = EngineRuntime::new();
    runtime.register_prefab(Prefab::new("player", "player-sprite"));
    runtime.spawn_prefab("player", 1.0, 2.0);

    let saved = runtime.save_scene("cosmos").unwrap();
    let reloaded = runtime.load_scene_from_text("sandbox", &saved);

    assert!(reloaded);
    assert!(saved.contains("scene:cosmos"));
    assert!(runtime.scenes.contains_key("sandbox"));
}

#[test]
fn engine_can_register_animations_and_persist_project_files() {
    let mut runtime = EngineRuntime::new();
    runtime.register_animation(Animation::new("walk", vec!["frame1".to_string(), "frame2".to_string()], 8));

    let project_path = std::env::temp_dir().join("cosmos-engine-project.txt");
    runtime.save_project(project_path.to_str().unwrap()).unwrap();
    let mut loaded = EngineRuntime::new();
    let result = loaded.load_project(project_path.to_str().unwrap());

    assert!(result.is_ok());
    assert!(std::fs::remove_file(&project_path).is_ok());
}

#[test]
fn engine_can_persist_scene_camera_and_tilemap_state() {
    let mut runtime = EngineRuntime::new();
    runtime.focus_camera(4.5, 6.0);
    runtime.build_level(6, 4);
    let scene_text = runtime.save_scene("cosmos").unwrap();
    let mut reloaded = EngineRuntime::new();
    let restored = reloaded.load_scene_from_text("cosmos", &scene_text);

    assert!(restored);
    assert!(scene_text.contains("camera:"));
    assert!(scene_text.contains("tile:"));
    assert_eq!(reloaded.scenes.get("cosmos").unwrap().world.tile_map.get_tile(0, 3), TileKind::Ground);
    assert_eq!(reloaded.scenes.get("cosmos").unwrap().camera.x, 4.5);
}

#[test]
fn engine_can_manage_scene_layers_and_objects() {
    let mut runtime = EngineRuntime::new();
    runtime.create_scene("editor");

    assert!(runtime.create_layer("editor", "foreground"));
    assert!(runtime.place_object_on_layer("editor", "foreground", "enemy", 1.0, 2.0));
    assert!(runtime.move_scene_object("editor", "enemy", 3.0, 4.0));

    let summary = runtime.scene_summary("editor");
    assert!(summary.contains("foreground"));
    assert!(summary.contains("enemy"));
}

#[test]
fn engine_can_delete_scene_objects_and_list_layers() {
    let mut runtime = EngineRuntime::new();
    runtime.create_scene("editor");
    runtime.create_layer("editor", "foreground");
    runtime.place_object_on_layer("editor", "foreground", "enemy", 1.0, 2.0);

    assert!(runtime.delete_scene_object("editor", "enemy"));
    let layers = runtime.list_layers("editor");

    assert!(layers.contains(&"foreground".to_string()));
    assert!(!runtime.scene_summary("editor").contains("enemy"));
}

#[test]
fn engine_can_execute_scripts_for_entities() {
    let mut runtime = EngineRuntime::new();
    let entity_id = runtime.spawn_entity(
        Transform::new(Vector4::new(0.0, 0.0, 0.0, 0.0), 1.0),
        None,
        Some(EntityLogic::new(NodeState::RegulatedEntity, 0.2, 0.0)),
        Some(Renderable::new("scripted")),
    );

    let mut script = Script::new("move-and-trigger");
    script.add_command(ScriptCommand::Move {
        entity_id,
        dx: 1.5,
        dy: -0.5,
    });
    script.add_command(ScriptCommand::SetState {
        entity_id,
        state: NodeState::AutonomousEntity,
    });
    script.add_command(ScriptCommand::Trigger { entity_id });

    let executed = runtime.run_script(&script);

    assert_eq!(executed.len(), 3);
    assert!(runtime.world.transforms.get(&entity_id).unwrap().position.x > 0.0);
    assert!(runtime.world.logic_nodes.get(&entity_id).unwrap().state == NodeState::AutonomousEntity);
}

#[test]
fn engine_can_update_ui_and_move_player_entities() {
    let mut runtime = EngineRuntime::new();
    let entity_id = runtime.spawn_entity(
        Transform::new(Vector4::new(0.0, 0.0, 0.0, 0.0), 1.0),
        None,
        Some(EntityLogic::new(NodeState::AutonomousEntity, 0.5, 0.0)),
        Some(Renderable::new("player")),
    );

    runtime.handle_input(InputAction::MoveRight, true);
    runtime.handle_input(InputAction::Jump, true);
    runtime.update_ui("status", "moving");
    let moved = runtime.move_player(entity_id, 0.5);

    assert!(moved);
    assert!(runtime.ui_layer.texts.iter().any(|entry| entry.label == "status"));
    assert!(runtime.world.transforms.get(&entity_id).unwrap().position.x > 0.0);
}

#[test]
fn engine_can_report_debug_overlay_and_gameplay_loop() {
    let mut runtime = EngineRuntime::new();
    runtime.spawn_entity(
        Transform::new(Vector4::new(0.0, 0.0, 0.0, 0.0), 1.0),
        None,
        None,
        Some(Renderable::new("debug")),
    );

    let summary = runtime.gameplay_loop(0.1);

    assert!(summary.contains("frame"));
    assert!(runtime.debug_overlay.entries.iter().any(|entry| entry.label == "frame"));
    assert!(runtime.debug_overlay.entries.iter().any(|entry| entry.label == "scene"));
}

#[test]
fn engine_can_play_animations_and_report_current_frame() {
    let mut runtime = EngineRuntime::new();
    runtime.register_animation(Animation::new("walk", vec!["frame1".to_string(), "frame2".to_string(), "frame3".to_string()], 2));
    runtime.register_prefab(Prefab::new("player", "player-sprite").with_animation("walk"));

    let entity_id = runtime.spawn_prefab("player", 0.0, 0.0).unwrap();
    let started = runtime.play_animation(entity_id, "walk", 0.5);
    let snapshot = runtime.render_frame();

    assert!(started);
    assert!(snapshot.animation_frame.is_some());
    assert_eq!(snapshot.active_animation.as_deref(), Some("walk"));
}

#[test]
fn editor_commands_can_create_scenes_and_save_projects() {
    let mut runtime = EngineRuntime::new();
    runtime.create_scene("sandbox");
    runtime.register_prefab(Prefab::new("player", "player-sprite"));

    let placed = runtime.spawn_prefab("player", 2.0, 3.0);
    let project_path = std::env::temp_dir().join("cosmos-editor-project.txt");
    let saved = runtime.save_project(project_path.to_str().unwrap());

    assert!(placed.is_some());
    assert!(saved.is_ok());
    assert!(std::fs::remove_file(&project_path).is_ok());
}

#[test]
fn logic_pipeline_can_toggle_state() {
    let mut world = CosmosWorld::new();
    let entity_id = world.spawn(
        Transform::new(Vector4::new(0.0, 0.0, 0.0, 0.0), 1.0),
        Some(RigidBody::new(Vector4::zero(), Vector4::zero(), 10.0)),
        Some(EntityLogic::new(NodeState::AutonomousEntity, 0.9, 0.0)),
        None,
    );

    let mut pipeline = entering_sleep_mode_command::EntityLogicPipeline;
    pipeline.update(&mut world, 0.1);

    let logic = world.logic_for(entity_id).unwrap();
    assert!(logic.autonomy_level >= 0.0);
    assert!(logic.cycle_frequency >= 0.1);
}
