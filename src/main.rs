use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::sync::Arc;
use sysinfo::{Components, Disks, Networks, System};
use tokio::sync::RwLock;

#[derive(Serialize, Clone)]
struct Stats {
    total_memory: u64,
    used_memory: u64,
    cpu_usage: f32,
    temp: f32,
    received: u64,
    transmitted: u64,
    total_disk: u64,
    used_disk: u64,
    free_disk: u64,
}

async fn get_stats(system: Arc<RwLock<System>>) -> Json<Stats> {
    let mut sys = system.write().await;
    sys.refresh_all();

    let cpu = sys.global_cpu_usage();
    let networks = Networks::new_with_refreshed_list();

    let total_received: u64 = networks
        .iter()
        .map(|(_, data)| data.total_received())
        .sum::<u64>()
        / 1_048_576;

    let total_transmitted: u64 = networks
        .iter()
        .map(|(_, data)| data.total_transmitted())
        .sum::<u64>()
        / 1_048_576; // Convert to MB

    // get temperature
    let components = Components::new_with_refreshed_list();
    let mut selecttemp = 0.0; // Default value if no temperature is available
    for component in components.iter() {
        if let Some(temp) = component.temperature() {
            // Assuming the first component's temperature is representative
            selecttemp = temp;
        }
    }

    let disks = Disks::new_with_refreshed_list();
    let mut disk_total = 0;
    let mut disk_free = 0;
    for disk in &disks {
        disk_total += disk.total_space();
        disk_free += disk.available_space();
    }
    let disk_used = disk_total - disk_free;
    Json(Stats {
        total_memory: sys.total_memory() / 1000024, // Convert to MB
        used_memory: sys.used_memory() / 1000024,   // Convert to MB
        cpu_usage: cpu,
        temp: selecttemp, // Use 0.0 if temperature is not available
        received: total_received,
        transmitted: total_transmitted,
        total_disk: disk_total / 1000024, // Convert to MB,
        used_disk: disk_used / 1000024,   // Convert to MB
        free_disk: disk_free / 1000024,   // Convert to MB
    })
}

#[tokio::main]
async fn main() {
    let shared_system = Arc::new(RwLock::new(System::new_all()));
    let app = Router::new()
        .route(
            "/api/stats",
            get({
                let shared_system = shared_system.clone();
                move || get_stats(shared_system)
            }),
        )
        .route(
            "/",
            get(|| async { axum::response::Html(include_str!("../static/index.html")) }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
