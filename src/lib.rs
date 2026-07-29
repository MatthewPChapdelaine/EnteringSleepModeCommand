use std::collections::HashMap;
use std::f64;
use std::io::{self, Write};

pub const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;

#[derive(Debug, Clone, Default)]
pub struct EngineConfig {
    pub scene_name: String,
    pub max_entities: usize,
    pub gravity_scale: f64,
}

impl EngineConfig {
    pub fn default() -> Self {
        Self {
            scene_name: "cosmos".to_string(),
            max_entities: 64,
            gravity_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }

    pub fn distance_3d(&self, other: &Self) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

pub type EntityId = usize;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vector4,
    pub scale: f64,
}

impl Transform {
    pub fn new(position: Vector4, scale: f64) -> Self {
        Self { position, scale }
    }
}

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub velocity: Vector4,
    pub acceleration: Vector4,
    pub mass: f64,
}

impl RigidBody {
    pub fn new(velocity: Vector4, acceleration: Vector4, mass: f64) -> Self {
        Self {
            velocity,
            acceleration,
            mass,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Singularity,
    StellarCore,
    AutonomousEntity,
    RegulatedEntity,
}

#[derive(Debug, Clone)]
pub struct EntityLogic {
    pub state: NodeState,
    pub autonomy_level: f32,
    pub cycle_frequency: f64,
}

#[derive(Debug, Clone)]
pub struct Renderable {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub frames: Vec<String>,
    pub fps: usize,
}

#[derive(Debug, Clone)]
pub struct AnimationPlayback {
    pub name: String,
    pub frame_index: usize,
    pub elapsed: f64,
}

impl Animation {
    pub fn new(name: impl Into<String>, frames: Vec<String>, fps: usize) -> Self {
        Self {
            name: name.into(),
            frames,
            fps,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptCommand {
    Move { entity_id: EntityId, dx: f64, dy: f64 },
    SetState { entity_id: EntityId, state: NodeState },
    Trigger { entity_id: EntityId },
}

#[derive(Debug, Clone)]
pub struct Script {
    pub name: String,
    pub commands: Vec<ScriptCommand>,
}

impl Script {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: ScriptCommand) {
        self.commands.push(command);
    }
}

#[derive(Debug, Clone)]
pub struct Prefab {
    pub name: String,
    pub renderable_name: String,
    pub tile: Option<TileKind>,
    pub animation_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ObjectLayer {
    pub name: String,
    pub objects: Vec<(String, f64, f64)>,
}

impl ObjectLayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, name: impl Into<String>, x: f64, y: f64) {
        self.objects.push((name.into(), x, y));
    }
}

impl Prefab {
    pub fn new(name: impl Into<String>, renderable_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            renderable_name: renderable_name.into(),
            tile: None,
            animation_name: None,
        }
    }

    pub fn with_tile(mut self, tile: TileKind) -> Self {
        self.tile = Some(tile);
        self
    }

    pub fn with_animation(mut self, animation_name: impl Into<String>) -> Self {
        self.animation_name = Some(animation_name.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Debug, Clone)]
pub struct UiText {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct UiLayer {
    pub texts: Vec<UiText>,
}

impl UiLayer {
    pub fn new() -> Self {
        Self { texts: Vec::new() }
    }

    pub fn add_text(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.texts.push(UiText {
            label: label.into(),
            value: value.into(),
        });
    }
}

#[derive(Debug, Clone)]
pub struct DebugOverlay {
    pub enabled: bool,
    pub entries: Vec<UiText>,
}

impl DebugOverlay {
    pub fn new() -> Self {
        Self {
            enabled: true,
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.entries.push(UiText {
            label: label.into(),
            value: value.into(),
        });
    }
}

impl Camera {
    pub fn new(x: f64, y: f64, zoom: f64) -> Self {
        Self { x, y, zoom }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Empty,
    Ground,
    Wall,
    Platform,
}

#[derive(Debug, Clone)]
pub struct TileMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileKind>>,
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![TileKind::Empty; width]; height],
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileKind) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileKind {
        if x < self.width && y < self.height {
            self.tiles[y][x]
        } else {
            TileKind::Empty
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    Jump,
    Interact,
}

#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub actions: HashMap<InputAction, bool>,
}

impl InputState {
    pub fn set(&mut self, action: InputAction, pressed: bool) {
        self.actions.insert(action, pressed);
    }

    pub fn is_pressed(&self, action: InputAction) -> bool {
        self.actions.get(&action).copied().unwrap_or(false)
    }
}

impl Renderable {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl EntityLogic {
    pub fn new(state: NodeState, autonomy_level: f32, cycle_frequency: f64) -> Self {
        Self {
            state,
            autonomy_level,
            cycle_frequency,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CosmosWorld {
    next_entity_id: EntityId,
    pub transforms: HashMap<EntityId, Transform>,
    pub rigid_bodies: HashMap<EntityId, RigidBody>,
    pub logic_nodes: HashMap<EntityId, EntityLogic>,
    pub renderables: HashMap<EntityId, Renderable>,
    pub tile_map: TileMap,
}

impl CosmosWorld {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            transforms: HashMap::new(),
            rigid_bodies: HashMap::new(),
            logic_nodes: HashMap::new(),
            renderables: HashMap::new(),
            tile_map: TileMap::new(8, 8),
        }
    }

    pub fn spawn(
        &mut self,
        transform: Transform,
        rigid_body: Option<RigidBody>,
        logic: Option<EntityLogic>,
        renderable: Option<Renderable>,
    ) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.transforms.insert(id, transform);
        if let Some(rb) = rigid_body {
            self.rigid_bodies.insert(id, rb);
        }
        if let Some(logic_node) = logic {
            self.logic_nodes.insert(id, logic_node);
        }
        if let Some(renderable_node) = renderable {
            self.renderables.insert(id, renderable_node);
        }
        id
    }

    pub fn entity_count(&self) -> usize {
        self.transforms.len()
    }

    pub fn logic_for(&self, entity_id: EntityId) -> Option<&EntityLogic> {
        self.logic_nodes.get(&entity_id)
    }

    pub fn render_names(&self) -> Vec<String> {
        self.renderables.values().map(|r| r.name.clone()).collect()
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileKind) {
        self.tile_map.set_tile(x, y, tile);
    }

    pub fn collides_with_tile(&self, x: f64, y: f64) -> bool {
        let tx = x.round() as usize;
        let ty = y.round() as usize;
        matches!(self.tile_map.get_tile(tx, ty), TileKind::Wall | TileKind::Platform)
    }
}

pub trait System {
    fn name(&self) -> &'static str;
    fn update(&mut self, world: &mut CosmosWorld, dt: f64);
}

pub struct GravitationalPhysicsSystem;

impl System for GravitationalPhysicsSystem {
    fn name(&self) -> &'static str {
        "GravitationalPhysicsSystem"
    }

    fn update(&mut self, world: &mut CosmosWorld, dt: f64) {
        let keys: Vec<EntityId> = world.rigid_bodies.keys().copied().collect();
        for i in 0..keys.len() {
            for j in (i + 1)..keys.len() {
                let id_a = keys[i];
                let id_b = keys[j];
                let pos_a = world.transforms.get(&id_a).map(|t| t.position).unwrap_or(Vector4::zero());
                let pos_b = world.transforms.get(&id_b).map(|t| t.position).unwrap_or(Vector4::zero());
                let mass_a = world.rigid_bodies.get(&id_a).map(|rb| rb.mass).unwrap_or(1.0);
                let mass_b = world.rigid_bodies.get(&id_b).map(|rb| rb.mass).unwrap_or(1.0);
                let distance = pos_a.distance_3d(&pos_b).max(1.0);
                let force_magnitude = (GRAVITATIONAL_CONSTANT * mass_a * mass_b) / (distance * distance);
                let direction = Vector4::new(
                    (pos_b.x - pos_a.x) / distance,
                    (pos_b.y - pos_a.y) / distance,
                    (pos_b.z - pos_a.z) / distance,
                    0.0,
                );
                if let Some(rb_a) = world.rigid_bodies.get_mut(&id_a) {
                    let acc_a = direction.scale(force_magnitude / mass_a);
                    rb_a.velocity = rb_a.velocity.add(&acc_a.scale(dt));
                }
                if let Some(rb_b) = world.rigid_bodies.get_mut(&id_b) {
                    let acc_b = direction.scale(-(force_magnitude / mass_b));
                    rb_b.velocity = rb_b.velocity.add(&acc_b.scale(dt));
                }
            }
        }
        for (id, rb) in world.rigid_bodies.iter() {
            if let Some(transform) = world.transforms.get_mut(id) {
                transform.position = transform.position.add(&rb.velocity.scale(dt));
            }
        }
    }
}

pub struct EntityLogicPipeline;

impl System for EntityLogicPipeline {
    fn name(&self) -> &'static str {
        "EntityLogicPipeline"
    }

    fn update(&mut self, world: &mut CosmosWorld, dt: f64) {
        for logic in world.logic_nodes.values_mut() {
            logic.cycle_frequency += dt;
            if logic.state == NodeState::AutonomousEntity {
                logic.autonomy_level = ((logic.cycle_frequency.sin() + 1.0) / 2.0) as f32;
                if logic.autonomy_level < 0.15 {
                    logic.state = NodeState::RegulatedEntity;
                }
            } else if logic.state == NodeState::RegulatedEntity {
                logic.autonomy_level = 0.0;
                if logic.cycle_frequency > 5.0 {
                    logic.state = NodeState::AutonomousEntity;
                    logic.cycle_frequency = 0.0;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandResponse {
    pub message: String,
    pub sleep_active: bool,
    pub entities_spawned: usize,
}

#[derive(Debug, Clone)]
pub struct FrameSnapshot {
    pub scene_name: String,
    pub frame_number: usize,
    pub entity_count: usize,
    pub renderables: Vec<String>,
    pub active_animation: Option<String>,
    pub animation_frame: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineEvent {
    SceneCreated(String),
    SceneLoaded(String),
    EntitySpawned(EntityId),
    EntityUpdated(EntityId),
}

#[derive(Debug, Clone, Default)]
pub struct EventLog {
    pub events: Vec<EngineEvent>,
}

impl EventLog {
    pub fn push(&mut self, event: EngineEvent) {
        self.events.push(event);
    }
}

impl CommandResponse {
    pub fn new(message: impl Into<String>, sleep_active: bool, entities_spawned: usize) -> Self {
        Self {
            message: message.into(),
            sleep_active,
            entities_spawned,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub world: CosmosWorld,
    pub layers: Vec<ObjectLayer>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            world: CosmosWorld::new(),
            layers: vec![ObjectLayer::new("default")],
            camera: Camera::new(0.0, 0.0, 1.0),
        }
    }
}

pub struct EngineRuntime {
    pub world: CosmosWorld,
    pub sleep_active: bool,
    pub frame_count: usize,
    pub config: EngineConfig,
    pub scenes: HashMap<String, Scene>,
    pub event_log: EventLog,
    pub input_state: InputState,
    pub camera: Camera,
    pub prefabs: HashMap<String, Prefab>,
    pub animations: HashMap<String, Animation>,
    pub ui_layer: UiLayer,
    pub debug_overlay: DebugOverlay,
    active_scene_name: String,
    active_animations: HashMap<EntityId, AnimationPlayback>,
}

impl EngineRuntime {
    pub fn new() -> Self {
        let mut scenes = HashMap::new();
        let default_scene = Scene::new("cosmos");
        scenes.insert(default_scene.name.clone(), default_scene);

        Self {
            world: CosmosWorld::new(),
            sleep_active: false,
            frame_count: 0,
            config: EngineConfig::default(),
            scenes,
            event_log: EventLog::default(),
            input_state: InputState::default(),
            camera: Camera::new(0.0, 0.0, 1.0),
            prefabs: HashMap::new(),
            animations: HashMap::new(),
            ui_layer: UiLayer::new(),
            debug_overlay: DebugOverlay::new(),
            active_scene_name: "cosmos".to_string(),
            active_animations: HashMap::new(),
        }
    }

    fn sync_active_scene(&mut self) {
        if let Some(scene) = self.scenes.get_mut(&self.active_scene_name) {
            scene.world = self.world.clone();
            scene.camera = self.camera.clone();
        }
    }

    pub fn create_scene(&mut self, name: &str) -> bool {
        let normalized = name.trim();
        if normalized.is_empty() || self.scenes.contains_key(normalized) {
            return false;
        }
        self.scenes.insert(normalized.to_string(), Scene::new(normalized));
        self.event_log.push(EngineEvent::SceneCreated(normalized.to_string()));
        true
    }

    pub fn load_scene(&mut self, name: &str) -> bool {
        let normalized = name.trim();
        if let Some(scene) = self.scenes.get(normalized) {
            self.active_scene_name = normalized.to_string();
            self.config.scene_name = normalized.to_string();
            self.world = scene.world.clone();
            self.camera = scene.camera.clone();
            self.event_log.push(EngineEvent::SceneLoaded(normalized.to_string()));
            true
        } else {
            false
        }
    }

    pub fn spawn_entity(
        &mut self,
        transform: Transform,
        rigid_body: Option<RigidBody>,
        logic: Option<EntityLogic>,
        renderable: Option<Renderable>,
    ) -> EntityId {
        let id = self.world.spawn(transform, rigid_body, logic, renderable);
        self.event_log.push(EngineEvent::EntitySpawned(id));
        self.sync_active_scene();
        id
    }

    pub fn render_frame(&self) -> FrameSnapshot {
        let (active_animation, animation_frame) = self
            .active_animations
            .values()
            .next()
            .and_then(|playback| {
                self.animations.get(&playback.name).map(|animation| {
                    let frame = animation.frames.get(playback.frame_index).cloned().unwrap_or_default();
                    (Some(playback.name.clone()), Some(frame))
                })
            })
            .unwrap_or((None, None));

        FrameSnapshot {
            scene_name: self.active_scene_name.clone(),
            frame_number: self.frame_count,
            entity_count: self.world.entity_count(),
            renderables: self.world.render_names(),
            active_animation,
            animation_frame,
        }
    }

    pub fn render_to_string(&self) -> String {
        let frame = self.render_frame();
        let renderables = if frame.renderables.is_empty() {
            "none".to_string()
        } else {
            frame.renderables.join(", ")
        };

        format!(
            "Scene [{}] | frame {} | entities {} | renderables: {}",
            frame.scene_name, frame.frame_number, frame.entity_count, renderables
        )
    }

    pub fn step(&mut self, dt: f64) {
        self.tick(dt);
        self.frame_count += 1;
        self.sync_active_scene();
    }

    pub fn place_object(&mut self, name: &str, x: f64, y: f64) -> EntityId {
        self.spawn_entity(
            Transform::new(Vector4::new(x, y, 0.0, 0.0), 1.0),
            None,
            None,
            Some(Renderable::new(name)),
        )
    }

    pub fn handle_input(&mut self, action: InputAction, pressed: bool) {
        self.input_state.set(action, pressed);
        if pressed {
            self.event_log.push(EngineEvent::EntityUpdated(usize::MAX));
        }
    }

    pub fn build_level(&mut self, width: usize, height: usize) -> TileMap {
        let mut tile_map = TileMap::new(width, height);
        for x in 0..width {
            tile_map.set_tile(x, height - 1, TileKind::Ground);
        }
        for x in 2..4 {
            tile_map.set_tile(x, height - 2, TileKind::Platform);
        }
        self.world.tile_map = tile_map.clone();
        tile_map
    }

    pub fn register_prefab(&mut self, prefab: Prefab) {
        self.prefabs.insert(prefab.name.clone(), prefab);
    }

    pub fn register_animation(&mut self, animation: Animation) {
        self.animations.insert(animation.name.clone(), animation);
    }

    pub fn play_animation(&mut self, entity_id: EntityId, animation_name: &str, dt: f64) -> bool {
        if let Some(animation) = self.animations.get(animation_name) {
            self.active_animations.insert(
                entity_id,
                AnimationPlayback {
                    name: animation_name.to_string(),
                    frame_index: 0,
                    elapsed: dt,
                },
            );
            if animation.frames.is_empty() {
                return false;
            }
            true
        } else {
            false
        }
    }

    pub fn run_script(&mut self, script: &Script) -> Vec<ScriptCommand> {
        let mut executed = Vec::new();
        for command in &script.commands {
            match command {
                ScriptCommand::Move { entity_id, dx, dy } => {
                    if let Some(transform) = self.world.transforms.get_mut(entity_id) {
                        transform.position = Vector4::new(
                            transform.position.x + dx,
                            transform.position.y + dy,
                            transform.position.z,
                            transform.position.w,
                        );
                    }
                }
                ScriptCommand::SetState { entity_id, state } => {
                    if let Some(logic) = self.world.logic_nodes.get_mut(entity_id) {
                        logic.state = *state;
                    }
                }
                ScriptCommand::Trigger { entity_id } => {
                    self.trigger_event(*entity_id);
                }
            }
            executed.push(command.clone());
        }
        executed
    }

    pub fn spawn_prefab(&mut self, prefab_name: &str, x: f64, y: f64) -> Option<EntityId> {
        let prefab = self.prefabs.get(prefab_name)?.clone();
        let entity_id = self.spawn_entity(
            Transform::new(Vector4::new(x, y, 0.0, 0.0), 1.0),
            None,
            None,
            Some(Renderable::new(prefab.renderable_name)),
        );
        if let Some(scene) = self.scenes.get_mut(&self.active_scene_name) {
            if let Some(layer) = scene.layers.iter_mut().find(|layer| layer.name == "default") {
                layer.add_object(prefab_name, x, y);
            }
        }
        Some(entity_id)
    }

    pub fn create_layer(&mut self, scene_name: &str, layer_name: &str) -> bool {
        let normalized = layer_name.trim();
        if normalized.is_empty() {
            return false;
        }
        if let Some(scene) = self.scenes.get_mut(scene_name) {
            if scene.layers.iter().any(|layer| layer.name == normalized) {
                return false;
            }
            scene.layers.push(ObjectLayer::new(normalized));
            true
        } else {
            false
        }
    }

    pub fn place_object_on_layer(&mut self, scene_name: &str, layer_name: &str, name: &str, x: f64, y: f64) -> bool {
        if let Some(scene) = self.scenes.get_mut(scene_name) {
            if let Some(layer) = scene.layers.iter_mut().find(|layer| layer.name == layer_name) {
                layer.add_object(name, x, y);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn move_scene_object(&mut self, scene_name: &str, object_name: &str, x: f64, y: f64) -> bool {
        if let Some(scene) = self.scenes.get_mut(scene_name) {
            for layer in &mut scene.layers {
                if let Some(object) = layer.objects.iter_mut().find(|(candidate, _, _)| candidate == object_name) {
                    object.1 = x;
                    object.2 = y;
                    return true;
                }
            }
        }
        false
    }

    pub fn delete_scene_object(&mut self, scene_name: &str, object_name: &str) -> bool {
        if let Some(scene) = self.scenes.get_mut(scene_name) {
            for layer in &mut scene.layers {
                let before = layer.objects.len();
                layer.objects.retain(|(candidate, _, _)| candidate != object_name);
                if layer.objects.len() != before {
                    return true;
                }
            }
        }
        false
    }

    pub fn list_layers(&self, scene_name: &str) -> Vec<String> {
        self.scenes
            .get(scene_name)
            .map(|scene| scene.layers.iter().map(|layer| layer.name.clone()).collect())
            .unwrap_or_default()
    }

    pub fn scene_summary(&self, scene_name: &str) -> String {
        let scene = match self.scenes.get(scene_name) {
            Some(scene) => scene,
            None => return format!("Scene '{scene_name}' not found"),
        };
        let layers = scene.layers.iter().map(|layer| layer.name.clone()).collect::<Vec<_>>().join(", ");
        let objects = scene
            .layers
            .iter()
            .flat_map(|layer| layer.objects.iter().map(|(name, x, y)| format!("{name}({x},{y})")))
            .collect::<Vec<_>>()
            .join(", ");
        format!("scene {scene_name} layers [{layers}] objects [{objects}]")
    }

    pub fn save_scene(&mut self, name: &str) -> Option<String> {
        self.sync_active_scene();
        let scene = self.scenes.get(name)?;
        let mut content = String::new();
        content.push_str(&format!("scene:{}\n", scene.name));
        content.push_str(&format!("camera:{}:{}:{}\n", scene.world.tile_map.width, scene.camera.x, scene.camera.y));
        for y in 0..scene.world.tile_map.height {
            let mut row = String::new();
            for x in 0..scene.world.tile_map.width {
                let tile = scene.world.tile_map.get_tile(x, y);
                row.push(match tile {
                    TileKind::Empty => '0',
                    TileKind::Ground => '1',
                    TileKind::Wall => '2',
                    TileKind::Platform => '3',
                });
            }
            content.push_str(&format!("tile:{row}\n"));
        }
        for layer in &scene.layers {
            content.push_str(&format!("layer:{}:{}\n", layer.name, layer.objects.len()));
            for (object_name, x, y) in &layer.objects {
                content.push_str(&format!("object:{object_name}:{x}:{y}\n"));
            }
        }
        Some(content)
    }

    pub fn load_scene_from_text(&mut self, name: &str, data: &str) -> bool {
        let mut scene = Scene::new(name);
        let mut tile_row_index = 0usize;
        for line in data.lines() {
            let mut parts = line.split(':');
            match parts.next() {
                Some("camera") => {
                    let width = parts.next().unwrap_or("0").parse::<usize>().unwrap_or(0);
                    let x = parts.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                    let y = parts.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                    scene.world.tile_map = TileMap::new(width, 4);
                    scene.camera = Camera::new(x, y, 1.0);
                }
                Some("tile") => {
                    let row = parts.next().unwrap_or("").to_string();
                    if row.is_empty() {
                        continue;
                    }
                    let mut tiles = Vec::new();
                    for ch in row.chars() {
                        let tile = match ch {
                            '1' => TileKind::Ground,
                            '2' => TileKind::Wall,
                            '3' => TileKind::Platform,
                            _ => TileKind::Empty,
                        };
                        tiles.push(tile);
                    }
                    let height = scene.world.tile_map.height.max(1);
                    if tile_row_index < height {
                        scene.world.tile_map.tiles[tile_row_index] = tiles.clone();
                    }
                    tile_row_index += 1;
                }
                Some("layer") => {
                    let layer_name = parts.next().unwrap_or("default").to_string();
                    scene.layers.push(ObjectLayer::new(layer_name));
                }
                Some("object") => {
                    let object_name = parts.next().unwrap_or("object").to_string();
                    let x = parts.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                    let y = parts.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                    if let Some(layer) = scene.layers.last_mut() {
                        layer.add_object(object_name, x, y);
                    }
                }
                _ => {}
            }
        }
        self.scenes.insert(name.to_string(), scene);
        true
    }

    pub fn focus_camera(&mut self, x: f64, y: f64) {
        self.camera.move_to(x, y);
    }

    pub fn update_ui(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.ui_layer.add_text(label, value);
    }

    pub fn update_debug(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.debug_overlay.add(label, value);
    }

    pub fn gameplay_loop(&mut self, dt: f64) -> String {
        self.update_debug("frame", self.frame_count.to_string());
        self.update_debug("scene", self.active_scene_name.clone());
        self.tick(dt);
        self.frame_count += 1;
        format!(
            "frame {} | scene {} | entities {}",
            self.frame_count,
            self.active_scene_name,
            self.world.entity_count()
        )
    }

    pub fn move_player(&mut self, entity_id: EntityId, dt: f64) -> bool {
        let mut moved = false;
        if let Some(transform) = self.world.transforms.get_mut(&entity_id) {
            let mut dx = 0.0;
            if self.input_state.is_pressed(InputAction::MoveRight) {
                dx += 1.0 * dt;
            }
            if self.input_state.is_pressed(InputAction::MoveLeft) {
                dx -= 1.0 * dt;
            }
            if self.input_state.is_pressed(InputAction::Jump) {
                transform.position.y += 0.25 * dt;
            }
            transform.position.x += dx;
            moved = true;
        }
        moved
    }

    pub fn save_project(&self, path: &str) -> std::io::Result<()> {
        let mut content = String::new();
        content.push_str(&format!("scene:{}\n", self.active_scene_name));
        for (name, animation) in &self.animations {
            content.push_str(&format!("anim:{}:{}:{}\n", name, animation.fps, animation.frames.join(",")));
        }
        std::fs::write(path, content)
    }

    pub fn load_project(&mut self, path: &str) -> std::io::Result<()> {
        let content = std::fs::read_to_string(path)?;
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("scene:") {
                self.load_scene(rest);
            } else if let Some(rest) = line.strip_prefix("anim:") {
                let mut parts = rest.split(':');
                let name = parts.next().unwrap_or("default").to_string();
                let fps = parts.next().unwrap_or("1").parse::<usize>().unwrap_or(1);
                let frames = parts.next().unwrap_or("").split(',').filter(|s| !s.is_empty()).map(str::to_string).collect();
                self.register_animation(Animation::new(name, frames, fps));
            }
        }
        Ok(())
    }

    pub fn trigger_event(&mut self, entity_id: EntityId) {
        self.event_log.push(EngineEvent::EntityUpdated(entity_id));
        if let Some(logic) = self.world.logic_nodes.get_mut(&entity_id) {
            logic.autonomy_level = (logic.autonomy_level + 0.2).min(1.0);
        }
    }

    pub fn run_command(&mut self, input: &str) -> CommandResponse {
        let normalized = input.trim();
        if normalized.eq_ignore_ascii_case("Enter Sleep Mode") {
            self.sleep_active = true;
            self.world.spawn(
                Transform::new(Vector4::new(0.0, 0.0, 0.0, 0.0), 1.0),
                Some(RigidBody::new(Vector4::zero(), Vector4::zero(), 10.0)),
                Some(EntityLogic::new(NodeState::RegulatedEntity, 0.0, 0.0)),
                Some(Renderable::new("sleep-node")),
            );
            self.tick(0.1);
            return CommandResponse::new(
                "COMMAND ACKNOWLEDGED. Sleep mode active. System resting.",
                self.sleep_active,
                1,
            );
        }

        if normalized.eq_ignore_ascii_case("Build") || normalized.to_lowercase().starts_with("build") {
            self.world.spawn(
                Transform::new(Vector4::new(1.0, 0.0, 0.0, 0.0), 1.0),
                Some(RigidBody::new(Vector4::new(0.1, 0.0, 0.0, 0.0), Vector4::zero(), 12.0)),
                Some(EntityLogic::new(NodeState::AutonomousEntity, 0.85, 0.0)),
                Some(Renderable::new("star")),
            );
            self.world.spawn(
                Transform::new(Vector4::new(-1.0, 0.0, 0.0, 0.0), 1.0),
                Some(RigidBody::new(Vector4::new(-0.1, 0.0, 0.0, 0.0), Vector4::zero(), 20.0)),
                Some(EntityLogic::new(NodeState::RegulatedEntity, 0.0, 0.0)),
                Some(Renderable::new("planet")),
            );
            self.tick(0.1);
            return CommandResponse::new(
                "BUILD ENVIRONMENT READY. CosmosEngine nodes online.",
                self.sleep_active,
                2,
            );
        }

        self.tick(0.05);
        self.sync_active_scene();
        CommandResponse::new(
            format!("Command '{normalized}' received. The engine remains idle."),
            self.sleep_active,
            0,
        )
    }

    pub fn tick(&mut self, dt: f64) {
        for playback in self.active_animations.values_mut() {
            playback.elapsed += dt;
            let frame_duration = 1.0 / (self.animations.get(&playback.name).map(|a| a.fps as f64).unwrap_or(1.0).max(1.0));
            if playback.elapsed >= frame_duration {
                playback.frame_index = (playback.frame_index + 1) % self.animations.get(&playback.name).map(|a| a.frames.len()).unwrap_or(1).max(1);
                playback.elapsed = 0.0;
            }
        }

        let mut physics = GravitationalPhysicsSystem;
        physics.update(&mut self.world, dt);
        let mut logic = EntityLogicPipeline;
        logic.update(&mut self.world, dt);
        self.sync_active_scene();
    }
}

pub fn run_interactive() -> io::Result<()> {
    println!("CosmosEngine ready. Type commands like 'scene <name>', 'place <name> <x> <y>', 'save <path>', 'quit'.");
    let mut runtime = EngineRuntime::new();
    loop {
        print!("cosmos> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command = input.trim();
        if command.eq_ignore_ascii_case("quit") || command.eq_ignore_ascii_case("exit") {
            break;
        }

        let mut parts = command.split_whitespace();
        match parts.next() {
            Some("scene") => {
                let name = parts.next().unwrap_or("default");
                if runtime.create_scene(name) {
                    println!("Created scene '{name}'.");
                } else {
                    println!("Scene '{name}' already exists.");
                }
            }
            Some("place") => {
                let prefab_name = parts.next().unwrap_or("player");
                let x = parts.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                let y = parts.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                if runtime.spawn_prefab(prefab_name, x, y).is_some() {
                    println!("Placed '{prefab_name}' at ({x}, {y}).");
                } else {
                    println!("No prefab named '{prefab_name}'. Register one first.");
                }
            }
            Some("save") => {
                let path = parts.next().unwrap_or("project.txt");
                if let Err(err) = runtime.save_project(path) {
                    println!("Save failed: {err}");
                } else {
                    println!("Saved project to '{path}'.");
                }
            }
            Some("load") => {
                let path = parts.next().unwrap_or("project.txt");
                if let Err(err) = runtime.load_project(path) {
                    println!("Load failed: {err}");
                } else {
                    println!("Loaded project from '{path}'.");
                }
            }
            Some("prefab") => {
                let name = parts.next().unwrap_or("player");
                runtime.register_prefab(Prefab::new(name, format!("{name}-sprite")));
                println!("Registered prefab '{name}'.");
            }
            _ => {
                let response = runtime.run_command(command);
                println!("{}", response.message);
            }
        }
    }
    Ok(())
}
