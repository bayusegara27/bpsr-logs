use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use log::{info, warn};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::live::bptimer_state::BPTimerEnabledMutex;
use crate::live::commands::{get_player_window, get_skill_window_impl, StatType};
use crate::live::opcodes_models::{Encounter, EncounterMutex};
use crate::live::player_state::{PlayerCacheMutex, PlayerStateMutex};

pub struct AppState {
    pub encounter: EncounterMutex,
    pub player_state: PlayerStateMutex,
    pub player_cache: PlayerCacheMutex,
    pub bptimer_enabled: BPTimerEnabledMutex,
}

pub async fn start_http_server(
    encounter: EncounterMutex,
    player_state: PlayerStateMutex,
    player_cache: PlayerCacheMutex,
    bptimer_enabled: BPTimerEnabledMutex,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("🔧 Building HTTP API server state...");
    
    let state = Arc::new(AppState {
        encounter,
        player_state,
        player_cache,
        bptimer_enabled,
    });

    info!("🔧 Configuring CORS...");
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    info!("🔧 Setting up API routes...");
    
    // Build routes first, then add state
    let api_routes = Router::new()
        .route("/header-info", get(api_get_header_info))
        .route("/dps-player-window", get(api_get_dps_player_window))
        .route("/dps-skill-window/:player_uid", get(api_get_dps_skill_window))
        .route(
            "/dps-boss-only-player-window",
            get(api_get_dps_boss_only_player_window),
        )
        .route(
            "/dps-boss-only-skill-window/:player_uid",
            get(api_get_dps_boss_only_skill_window),
        )
        .route("/heal-player-window", get(api_get_heal_player_window))
        .route(
            "/heal-skill-window/:player_uid",
            get(api_get_heal_skill_window),
        )
        .route("/test-player-window", get(api_get_test_player_window))
        .route(
            "/test-skill-window/:player_uid",
            get(api_get_test_skill_window),
        )
        .route("/reset-encounter", post(api_reset_encounter))
        .route("/toggle-pause-encounter", post(api_toggle_pause_encounter))
        .route("/hard-reset", post(api_hard_reset))
        .route("/set-bptimer-enabled", post(api_set_bptimer_enabled))
        .with_state(state);

    info!("🔧 Creating main router with CORS layer...");
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors);

    // Try ports 3000-3010 to find an available one
    info!("🚀 Attempting to start HTTP API server...");
    let mut port = 3000;
    let listener = loop {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        info!("📡 Attempting to bind HTTP API server to {}...", addr);
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                info!("✅ HTTP API server successfully bound to http://{}", addr);
                info!("🌐 Web browser can access the API at:");
                info!("   http://localhost:{}/api", port);
                info!("   http://127.0.0.1:{}/api", port);
                info!("📊 Available endpoints:");
                info!("   GET  http://localhost:{}/api/header-info", port);
                info!("   GET  http://localhost:{}/api/dps-player-window", port);
                info!("   GET  http://localhost:{}/api/heal-player-window", port);
                info!("   POST http://localhost:{}/api/reset-encounter", port);
                break listener;
            }
            Err(e) => {
                if port < 3010 {
                    warn!("⚠️  Port {} is already in use ({}), trying port {}...", port, e, port + 1);
                    port += 1;
                } else {
                    let err_msg = format!("Could not bind HTTP API server to any port 3000-3010: {}", e);
                    warn!("❌ {}", err_msg);
                    warn!("💡 Please close any applications using these ports and restart");
                    return Err(err_msg.into());
                }
            }
        }
    };

    info!("🎯 HTTP API server is ready - starting to serve requests...");
    
    match axum::serve(listener, app).await {
        Ok(_) => {
            info!("✅ HTTP API server shut down gracefully");
            Ok(())
        }
        Err(e) => {
            warn!("❌ HTTP API server error: {}", e);
            Err(Box::new(e))
        }
    }
}

fn get_header_info_from_encounter(encounter: &Encounter) -> Result<serde_json::Value, String> {
    if encounter.dmg_stats.value == 0 {
        return Err("No damage found".to_string());
    }

    let time_elapsed_ms = encounter.time_last_combat_packet_ms - encounter.time_fight_start_ms;
    #[allow(clippy::cast_precision_loss)]
    let time_elapsed_secs = time_elapsed_ms as f64 / 1000.0;
    let encounter_stats = &encounter.dmg_stats;

    #[allow(clippy::cast_precision_loss)]
    let total_dps = if time_elapsed_secs > 0.0 {
        encounter_stats.value as f64 / time_elapsed_secs
    } else {
        0.0
    };

    Ok(serde_json::json!({
        "totalDps": if total_dps.is_nan() || total_dps.is_infinite() { 0.0 } else { total_dps },
        "totalDmg": encounter_stats.value as f64,
        "elapsedMs": time_elapsed_ms as f64,
        "timeLastCombatPacketMs": encounter.time_last_combat_packet_ms as f64,
    }))
}

async fn api_get_header_info(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let encounter = state.encounter.lock().unwrap();
    match get_header_info_from_encounter(&encounter) {
        Ok(info) => Ok(Json(info)),
        Err(e) => {
            warn!("Error getting header info: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn api_get_dps_player_window(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let encounter = state.encounter.lock().unwrap();
    let player_cache = state.player_cache.lock().unwrap();
    let player_state = state.player_state.lock().unwrap();
    let result = get_player_window(encounter, StatType::Dmg, &player_cache, &player_state);
    Json(serde_json::to_value(result).unwrap())
}

async fn api_get_dps_skill_window(
    State(state): State<Arc<AppState>>,
    Path(player_uid): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let player_uid_i64 = player_uid.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?;
    let encounter = state.encounter.lock().unwrap();
    let player_cache = state.player_cache.lock().unwrap();
    let player_state = state.player_state.lock().unwrap();
    match get_skill_window_impl(
        encounter,
        player_uid_i64,
        StatType::Dmg,
        &player_cache,
        &player_state,
    ) {
        Ok(result) => Ok(Json(serde_json::to_value(result).unwrap())),
        Err(e) => {
            warn!("Error getting DPS skill window: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn api_get_dps_boss_only_player_window(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let encounter = state.encounter.lock().unwrap();
    let player_cache = state.player_cache.lock().unwrap();
    let player_state = state.player_state.lock().unwrap();
    let result = get_player_window(
        encounter,
        StatType::DmgBossOnly,
        &player_cache,
        &player_state,
    );
    Json(serde_json::to_value(result).unwrap())
}

async fn api_get_dps_boss_only_skill_window(
    State(state): State<Arc<AppState>>,
    Path(player_uid): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let player_uid_i64 = player_uid.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?;
    let encounter = state.encounter.lock().unwrap();
    let player_cache = state.player_cache.lock().unwrap();
    let player_state = state.player_state.lock().unwrap();
    match get_skill_window_impl(
        encounter,
        player_uid_i64,
        StatType::DmgBossOnly,
        &player_cache,
        &player_state,
    ) {
        Ok(result) => Ok(Json(serde_json::to_value(result).unwrap())),
        Err(e) => {
            warn!("Error getting DPS boss-only skill window: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn api_get_heal_player_window(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let encounter = state.encounter.lock().unwrap();
    let player_cache = state.player_cache.lock().unwrap();
    let player_state = state.player_state.lock().unwrap();
    let result = get_player_window(encounter, StatType::Heal, &player_cache, &player_state);
    Json(serde_json::to_value(result).unwrap())
}

async fn api_get_heal_skill_window(
    State(state): State<Arc<AppState>>,
    Path(player_uid): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let player_uid_i64 = player_uid.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?;
    let encounter = state.encounter.lock().unwrap();
    let player_cache = state.player_cache.lock().unwrap();
    let player_state = state.player_state.lock().unwrap();
    match get_skill_window_impl(
        encounter,
        player_uid_i64,
        StatType::Heal,
        &player_cache,
        &player_state,
    ) {
        Ok(result) => Ok(Json(serde_json::to_value(result).unwrap())),
        Err(e) => {
            warn!("Error getting heal skill window: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn api_get_test_player_window(
    State(_state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    use crate::live::commands::get_test_player_window as get_test_impl;
    let result = get_test_impl();
    Json(serde_json::to_value(result).unwrap())
}

async fn api_get_test_skill_window(
    State(_state): State<Arc<AppState>>,
    Path(player_uid): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    use crate::live::commands::get_test_skill_window as get_test_skill_impl;
    match get_test_skill_impl(player_uid) {
        Ok(result) => Ok(Json(serde_json::to_value(result).unwrap())),
        Err(e) => {
            warn!("Error getting test skill window: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn api_reset_encounter(State(state): State<Arc<AppState>>) -> StatusCode {
    let mut encounter = state.encounter.lock().unwrap();
    encounter.clone_from(&Encounter::default());
    info!("Encounter reset via HTTP API");
    StatusCode::OK
}

async fn api_toggle_pause_encounter(State(state): State<Arc<AppState>>) -> StatusCode {
    let mut encounter = state.encounter.lock().unwrap();
    encounter.is_encounter_paused = !encounter.is_encounter_paused;
    info!("Encounter pause toggled via HTTP API");
    StatusCode::OK
}

async fn api_hard_reset(State(state): State<Arc<AppState>>) -> StatusCode {
    use crate::packets::packet_capture::request_restart;
    let mut encounter = state.encounter.lock().unwrap();
    encounter.clone_from(&Encounter::default());
    request_restart();
    info!("Hard reset via HTTP API");
    StatusCode::OK
}

#[derive(serde::Deserialize)]
struct SetBptimerRequest {
    enabled: bool,
}

async fn api_set_bptimer_enabled(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SetBptimerRequest>,
) -> StatusCode {
    use crate::live::bptimer_state::set_bptimer_enabled;
    set_bptimer_enabled(&state.bptimer_enabled, payload.enabled);
    info!("BPTimer enabled set to {} via HTTP API", payload.enabled);
    StatusCode::OK
}
