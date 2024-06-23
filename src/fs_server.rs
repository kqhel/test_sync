use std::{path::PathBuf, sync::{Arc, Mutex}};
use notify::{Config, RecommendedWatcher, Watcher};

use axum::{
	extract::State, routing::get, Json, Router
};

//

type InsideQueriedData = Vec<PathBuf>;
type ArcQueriedData = Arc<Mutex<InsideQueriedData>>;

pub struct FsServer {
	queried_data: ArcQueriedData
}

impl FsServer {
	pub fn new() -> Self {
		Self {
			queried_data: Arc::new(
				Mutex::new(vec![])
			)
		}
	}

	pub async fn watch(&mut self) -> tokio::task::JoinHandle<()> {
		let queried_data = Arc::clone(&self.queried_data);

		tokio::spawn(async move {
			let (tx, rx) = std::sync::mpsc::channel();
		
			let current_dir = std::env::current_dir().unwrap();
			let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
	
			let watch_result = watcher.watch(&current_dir, notify::RecursiveMode::Recursive);
	
			if let Err(error) = watch_result {
				panic!("Error: {:?}", error);
			}

			for result in rx {
				if let Ok(event) = result {
					let paths = event.paths;
					let mut data = queried_data.lock().unwrap();

					for path in paths {
						println!("{:?}", path);
						data.push(path)
					}
				}
			}
		})
	}

	pub async fn start(&self) -> tokio::task::JoinHandle<()> {
		let queried_data = Arc::clone(&self.queried_data);
		
		async fn get_queried_data(State(state): State<ArcQueriedData>) -> Json<Vec<PathBuf>> {
			let data_clone = state.lock().unwrap().clone();
			Json(data_clone)
		}

		let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
		let app = Router::new()
			.route(
				"/", 
				get(get_queried_data)
			)
			.with_state(queried_data);
		
		tokio::spawn(async move {
			axum::serve(listener, app).await.unwrap()
		})
	}

}