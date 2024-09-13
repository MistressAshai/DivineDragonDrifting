use unity::prelude::*;

// todo: grab unity vsync_count to check what fps we're on
static TARGET_FPS: f32 = 60.0;

fn speed_modifier() -> f32 {
    let speed_mod = 30.0 / TARGET_FPS;
    return speed_mod * speed_mod;
}

#[unity::hook("App", "HubUtil", "get_PlayerDashStopTime")]
pub fn get_player_dash_stop_time_hook(method_info: OptionalMethod) -> f32 {
    let dash_stop_time = call_original!(method_info);

    return speed_modifier() * dash_stop_time;
}

#[unity::hook("App", "HubUtil", "GetPlayerSpeedCurve")]
pub fn get_player_speed_curve_hook(method_info: OptionalMethod) -> f32 {
    let speed_curve = call_original!(method_info);

    return speed_modifier() * speed_curve;
}