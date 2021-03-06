use macroquad::prelude::*;

use macroquad_particles as particles;
use macroquad_profiler as profiler;
use macroquad_tiled as tiled;

use macroquad::{
    experimental::{collections::storage, scene},
    telemetry,
};

use particles::EmittersCache;
use physics_platformer::World as CollisionWorld;

mod nakama;

mod credentials {
    include!(concat!(env!("OUT_DIR"), "/nakama_credentials.rs"));
}

mod bullets;
mod camera;
mod global_events;
mod level_background;
mod net_syncronizer;
mod pickup;
mod player;
mod remote_player;

use bullets::Bullets;
use camera::Camera;
use global_events::GlobalEvents;
use level_background::LevelBackground;
use net_syncronizer::NetSyncronizer;
use pickup::Pickup;
use player::Player;
use remote_player::RemotePlayer;

pub mod consts {
    pub const GRAVITY: f32 = 900.0;
    pub const JUMP_SPEED: f32 = 480.0;
    pub const RUN_SPEED: f32 = 250.0;
    pub const PLAYER_SPRITE: u32 = 120;
    pub const BULLET_SPEED: f32 = 500.0;
    pub const JUMP_GRACE_TIME: f32 = 0.15;
    pub const NETWORK_FPS: f32 = 15.0;
    pub const GUN_THROWBACK: f32 = 700.0;
}

struct Resources {
    hit_fxses: EmittersCache,
    explosion_fxses: EmittersCache,
    disarm_fxses: EmittersCache,
    tiled_map: tiled::Map,
    collision_world: CollisionWorld,
    whale: Texture2D,
    gun: Texture2D,
    background_01: Texture2D,
    background_02: Texture2D,
    background_03: Texture2D,
    background_04: Texture2D,
}

pub const HIT_FX: &'static str = r#"{"local_coords":false,"emission_shape":{"Point":[]},"one_shot":true,"lifetime":0.2,"lifetime_randomness":0,"explosiveness":0.65,"amount":41,"shape":{"Circle":{"subdivisions":10}},"emitting":false,"initial_direction":{"x":0,"y":-1},"initial_direction_spread":6.2831855,"initial_velocity":73.9,"initial_velocity_randomness":0.2,"linear_accel":0,"size":5.6000004,"size_randomness":0.4,"blend_mode":{"Alpha":[]},"colors_curve":{"start":{"r":0.8200004,"g":1,"b":0.31818175,"a":1},"mid":{"r":0.71000004,"g":0.36210018,"b":0,"a":1},"end":{"r":0.02,"g":0,"b":0.000000007152557,"a":1}},"gravity":{"x":0,"y":0},"post_processing":{}}
"#;

pub const EXPLOSION_FX: &'static str = r#"{"local_coords":false,"emission_shape":{"Sphere":{"radius":0.6}},"one_shot":true,"lifetime":0.35,"lifetime_randomness":0,"explosiveness":0.6,"amount":131,"shape":{"Circle":{"subdivisions":10}},"emitting":false,"initial_direction":{"x":0,"y":-1},"initial_direction_spread":6.2831855,"initial_velocity":316,"initial_velocity_randomness":0.6,"linear_accel":-7.4000025,"size":5.5,"size_randomness":0.3,"size_curve":{"points":[[0.005,1.48],[0.255,1.0799999],[1,0.120000005]],"interpolation":{"Linear":[]},"resolution":30},"blend_mode":{"Additive":[]},"colors_curve":{"start":{"r":0.9825908,"g":1,"b":0.13,"a":1},"mid":{"r":0.8,"g":0.19999999,"b":0.2000002,"a":1},"end":{"r":0.101,"g":0.099,"b":0.099,"a":1}},"gravity":{"x":0,"y":-500},"post_processing":{}}
"#;

pub const WEAPON_DISARM_FX: &'static str = r#"{"local_coords":false,"emission_shape":{"Sphere":{"radius":0.6}},"one_shot":true,"lifetime":0.1,"lifetime_randomness":0,"explosiveness":1,"amount":100,"shape":{"Circle":{"subdivisions":10}},"emitting":false,"initial_direction":{"x":0,"y":-1},"initial_direction_spread":6.2831855,"initial_velocity":359.6,"initial_velocity_randomness":0.8,"linear_accel":-2.400001,"size":2.5,"size_randomness":0,"size_curve":{"points":[[0,0.92971194],[0.295,1.1297119],[1,0.46995974]],"interpolation":{"Linear":[]},"resolution":30},"blend_mode":{"Additive":[]},"colors_curve":{"start":{"r":0.99999994,"g":0.9699999,"b":0.37000006,"a":1},"mid":{"r":0.81000006,"g":0.6074995,"b":0,"a":1},"end":{"r":0.72,"g":0.54,"b":0,"a":1}},"gravity":{"x":0,"y":-300},"post_processing":{}}
"#;

impl Resources {
    async fn new() -> Resources {
        let mut collision_world = CollisionWorld::new();

        let tileset = load_texture("assets/tileset.png").await;
        set_texture_filter(tileset, FilterMode::Nearest);

        let tiled_map_json = load_string("assets/map.json").await.unwrap();
        let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
            static_colliders.push(tile.is_some());
        }
        collision_world.add_static_tiled_layer(
            static_colliders,
            32.,
            32.,
            tiled_map.raw_tiled_map.width as _,
            1,
        );

        let hit_fxses = EmittersCache::new(nanoserde::DeJson::deserialize_json(HIT_FX).unwrap());
        let explosion_fxses =
            EmittersCache::new(nanoserde::DeJson::deserialize_json(EXPLOSION_FX).unwrap());
        let disarm_fxses =
            EmittersCache::new(nanoserde::DeJson::deserialize_json(WEAPON_DISARM_FX).unwrap());

        let whale = load_texture("assets/Whale/Whale(76x66)(Orange).png").await;
        set_texture_filter(whale, FilterMode::Nearest);

        let gun = load_texture("assets/Whale/Gun(92x32).png").await;
        set_texture_filter(gun, FilterMode::Nearest);

        let background_01 = load_texture("assets/Background/01.png").await;
        set_texture_filter(background_01, FilterMode::Nearest);

        let background_02 = load_texture("assets/Background/02.png").await;
        set_texture_filter(background_02, FilterMode::Nearest);

        let background_03 = load_texture("assets/Background/03.png").await;
        set_texture_filter(background_03, FilterMode::Nearest);

        let background_04 = load_texture("assets/Background/04.png").await;
        set_texture_filter(background_04, FilterMode::Nearest);

        Resources {
            hit_fxses,
            explosion_fxses,
            disarm_fxses,
            tiled_map,
            collision_world,
            whale,
            gun,
            background_01,
            background_02,
            background_03,
            background_04,
        }
    }
}

#[macroquad::main("Fishgame")]
async fn main() {
    nakama::connect(
        credentials::NAKAMA_KEY,
        credentials::NAKAMA_SERVER,
        credentials::NAKAMA_PORT,
        credentials::NAKAMA_PROTOCOL,
    );

    #[cfg(target_arch = "wasm32")]
    {
        while nakama::connected() == false {
            clear_background(BLACK);
            draw_text(
                &format!(
                    "Connecting {}",
                    ".".repeat(((get_time() * 2.0) as usize) % 4)
                ),
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0,
                40.,
                WHITE,
            );

            next_frame().await;
        }
    }

    let network_id = nakama::self_id();

    let resources = Resources::new().await;

    let w = resources.tiled_map.raw_tiled_map.tilewidth * resources.tiled_map.raw_tiled_map.width;
    let h = resources.tiled_map.raw_tiled_map.tileheight * resources.tiled_map.raw_tiled_map.height;

    storage::store(resources);

    scene::add_node(LevelBackground::new());
    let player = scene::add_node(Player::new());

    scene::add_node(Bullets::new(player));
    let net_syncronizer = scene::add_node(NetSyncronizer::new(network_id));
    scene::add_node(GlobalEvents::new(player, net_syncronizer));

    let mut camera = Camera::new(Rect::new(0.0, 0.0, w as f32, h as f32), 400.0);

    loop {
        clear_background(BLACK);

        let pos = { scene::get_node::<Player>(player).unwrap().pos() };

        let cam = camera.update(pos);
        set_camera(cam);

        storage::store(cam.target);

        scene::update();

        {
            let _z = telemetry::ZoneGuard::new("draw particles");

            let mut resources = storage::get_mut::<Resources>().unwrap();

            resources.hit_fxses.draw();
            resources.explosion_fxses.draw();
            resources.disarm_fxses.draw();
        }

        set_default_camera();

        profiler::profiler(profiler::ProfilerParams {
            fps_counter_pos: vec2(50.0, 20.0),
        });

        next_frame().await;
    }
}
