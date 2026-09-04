pub mod llm;
pub mod neuro_symbolic;
pub mod server;

use iml_core_lib::{Arena, Node, NodeType};
use reqwest::Client;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use server::{AppState, create_router};

#[tokio::main]
async fn main() {
    // Initial dummy data for the AST
    let arena = Arena {
        nodes: vec![
            Node {
                node_type: NodeType::Num(42),
                children: vec![],
                rationale: Some("This is a constant number 42".to_string()),
                ownership: None,
            },
            Node {
                node_type: NodeType::Alloc,
                children: vec![0],
                rationale: Some("Allocate the number to memory".to_string()),
                ownership: Some("resource1".to_string()),
            },
            Node {
                node_type: NodeType::Drop,
                children: vec![1],
                rationale: Some("Drop the allocated memory".to_string()),
                ownership: None,
            },
        ],
    };

    let state = Arc::new(AppState {
        arena: Arc::new(Mutex::new(arena)),
        client: Client::new(),
    });

    let app = create_router(state);
    
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
