mod blocklist;
mod store;

use std::{
	net::{IpAddr, Ipv4Addr, SocketAddr},
	sync::Arc,
	time::Duration,
};

use axum::{
	extract::{Path, State},
	response::IntoResponse,
	routing::get,
	Json, Router,
};
use clokwerk::{AsyncScheduler, Job, TimeUnits};
use store::Store;
use tokio::{sync::Mutex, task::JoinHandle};
use tracing::info;

/// The default IP address which the servers exposes
pub const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// The default PORT the server exposes so clients can connect to
pub const DEFAULT_PORT: u16 = 3000;
/// The time of the day the cron job should run at
pub const TIME_OF_DAY: &str = "3:30 am";
/// How long to pause the loop which checks if a new scheduler/cron job is ready to be run again
pub const HOUR_FROM_SEC: f64 = 60.0 * 60.0;

#[tokio::main]
async fn main() {
	// Setting the logging configuration globablly
	let subscriber = tracing_subscriber::FmtSubscriber::new();
	tracing::subscriber::set_global_default(subscriber).unwrap();

	// Create an asynchronous scheduler which serves as our cron job
	let scheduler = AsyncScheduler::new();

	// Creating an empty storage which can be mutated and passed around thread safely
	let store = Arc::new(Mutex::new(store::Store::default()));

	// Query the remote blocklist endpoint and set up the store once before the server starts
	fetch_and_update_blocklist(store.clone()).await;

	let server_handle = start_server(store.clone());
	let scheduler_handle = scheduling_update_store(scheduler, store.clone());

	// Start different tasks for starting the server and the cron job scheduler
	let _ = tokio::join!(server_handle.await, scheduler_handle.await);
}

async fn scheduling_update_store(
	mut scheduler: AsyncScheduler,
	store: Arc<Mutex<Store>>,
) -> JoinHandle<()> {
	scheduler.every(1.day()).at(TIME_OF_DAY).run(move || {
		info!("Run the midnight scheduler");
		fetch_and_update_blocklist(store.clone())
	});

	tokio::spawn(async move {
		loop {
			info!("Check for pending scheduler tasks");
			scheduler.run_pending().await;
			tokio::time::sleep(Duration::from_secs_f64(HOUR_FROM_SEC)).await;
		}
	})
}

async fn start_server(
	store: Arc<Mutex<Store>>,
) -> axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>> {
	let addr = SocketAddr::new(DEFAULT_IP, DEFAULT_PORT);

	let app = Router::new()
		.route("/ips/:ip", get(check_blocklist))
		.with_state(store);

	info!("Start the server at {}", addr);

	axum::Server::bind(&addr).serve(app.into_make_service())
}

async fn fetch_and_update_blocklist(store: Arc<Mutex<Store>>) {
	let store = store.clone();
	let list = blocklist::fetch_latest_blocklist().await;
	let mut store = store.lock().await;
	store.update(list);
}

async fn check_blocklist(
	Path(id): Path<String>,
	State(store): State<Arc<Mutex<Store>>>,
) -> impl IntoResponse {
	info!(id, "Check for IP address");
	Json(store.lock().await.contains(&id))
}
