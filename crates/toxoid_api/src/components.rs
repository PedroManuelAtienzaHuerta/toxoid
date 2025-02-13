use crate::*;

component! {
    // -- Components --
    // Space
    Position {
        x: i32,
        y: i32
    },
    Size {
        width: u32,
        height: u32
    },
    // Rendering
    Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32
    },
    RenderTarget {
        render_target: u64,
        z_depth: u32,
        flip_y: bool
    },
    BlendMode {
        blend_mode: u8,
        alpha: f32
    },
    Sprite {
        sprite: u64
    },
    Image {
        info: u64
    },
    // Fetch
    FetchRequest {
        data_type: u8,
        path: String,
        data: Vec::<u8>,
        user_data: u64
    },
    // Bone Animation
    Atlas {
        atlas: u64,
        filename: String,
        data: Vec::<u8>,
        loaded: bool,
    },
    Skeleton {
        skeleton: u64,
        filename: String,
        data: Vec::<u8>,
        loaded: bool,
    },
    Images {
        images: Vec::<u64>,
        loaded: bool,
    },
    BoneAnimation {
        animation_state: String,
        animation: String
    },
    SpineInstance {
        instance: u64,
        instantiated: bool
    },
    // Frame by Frame Animation
    FrameByFrameAnimation {},
    // Tilemaps
    TiledWorld {
        world: u64
    },
    TiledCell {
        cell: u64,
        index: u32
    },
    Tileset {
        tileset: String
    },

    // -- Tags --
    // Rendering
    Rect {},
    Blittable {},
    Renderable {},
    RenderableOnLoad {},
    // UI
    Window {},
    Text {},
    Button {},
    UIImage {
        texture_id: u64,
    },
    // General
    Loading {},
    Loaded {},
    Connected {},
    Disconnected {},
    // Relationships
    RenderTargetRelationship {},
    TilesetRelationship {},
    BoneAnimationRelationship {},
    FrameByFrameAnimationRelationship {},
    SpriteRelationship {},
    RectRelationship {},
    Player {},

    // -- Singletons --
    KeyboardInput {
        up: bool,
        down: bool,
        left: bool,
        right: bool, 
    },
    RenderSystems {
        entity: u64
    },
    GameConfig {
        width: u32,
        height: u32
    },

    // Camera components
    Camera {
        viewport_width: f32,
        viewport_height: f32,
        zoom: f32
    },

    // Add to singletons section
    MainCamera {
        entity: u64
    },

    // Update AnimationState component
    AnimationState {
        current_animation: String,
        last_valid_direction: u8
    },
    MovementState {
        is_moving: bool
    },
}

pub fn init() {
    // Register components
    // Space
    Position::register();
    Size::register();
    // Rendering
    Color::register();
    Sprite::register();
    Image::register();
    RenderTarget::register();
    BlendMode::register();
    // Fetch
    FetchRequest::register();
    // Animation
    Atlas::register();
    Skeleton::register();
    Images::register();
    BoneAnimation::register();
    SpineInstance::register();
    // Frame by Frame Animation
    FrameByFrameAnimation::register();
    // Tilemaps
    TiledWorld::register();
    TiledCell::register();
    Tileset::register();

    // Register tags
    // Rendering
    Rect::register();
    Blittable::register();
    Renderable::register();
    // UI
    Window::register();
    Text::register();
    Button::register();
    UIImage::register();
    // General
    Loading::register();
    Loaded::register();
    Connected::register();
    Disconnected::register();
    // Relationships
    RenderTargetRelationship::register();
    TilesetRelationship::register();
    BoneAnimationRelationship::register();
    FrameByFrameAnimationRelationship::register();
    SpriteRelationship::register();
    RectRelationship::register();
    Player::register();
    
    // Register singletons
    KeyboardInput::register();
    RenderSystems::register();
    GameConfig::register();

    // Register camera components
    Camera::register();
    MainCamera::register();
    AnimationState::register();
    MovementState::register();

    // Add singletons
    World::add_singleton::<KeyboardInput>();
    World::add_singleton::<RenderSystems>();
    World::add_singleton::<GameConfig>();
    World::add_singleton::<MainCamera>();
    World::add_singleton::<AnimationState>();
    World::add_singleton::<MovementState>();
}