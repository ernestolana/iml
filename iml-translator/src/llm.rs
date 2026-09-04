use iml_core_lib::{Arena, Node};
use crate::neuro_symbolic::validate_update;
use reqwest::Client;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(rename = "responseMimeType")]
    response_mime_type: String,
    #[serde(rename = "responseSchema")]
    response_schema: Value,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Vec<CandidatePart>,
}

#[derive(Deserialize)]
struct CandidatePart {
    text: String,
}

pub async fn rewrite_node(
    client: &Client,
    arena: &Arena,
    node_index: usize,
    updated_text: &str,
) -> Result<Arena, String> {
    let api_key = env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY not set")?;
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={}", api_key);
    
    let original_node = arena.nodes.get(node_index).ok_or("Node index out of bounds")?;
    let schema = schema_for!(Node);
    let schema_json = serde_json::to_value(&schema).unwrap();

    let mut error_trace = String::new();
    let max_retries = 3;

    for _attempt in 0..max_retries {
        let mut prompt = format!(
            "You are an expert compiler engineer. Rewrite the following IML Node based on the human's updated rationale.\n\
             Original Node: {}\n\
             Updated Rationale: {}\n",
            serde_json::to_string(original_node).unwrap(),
            updated_text
        );

        if !error_trace.is_empty() {
            prompt.push_str(&format!("\nYour previous attempt failed neuro-symbolic validation. Fix the following error:\n{}\n", error_trace));
        }

        let req_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
            generation_config: GenerationConfig {
                response_mime_type: "application/json".to_string(),
                response_schema: schema_json.clone(),
            },
        };

        let res = client.post(&url).json(&req_body).send().await;
        let res = match res {
            Ok(r) => r,
            Err(e) => return Err(format!("Network error: {}", e)),
        };

        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Gemini API error: {}", text));
        }

        let gemini_resp: GeminiResponse = match res.json().await {
            Ok(r) => r,
            Err(e) => return Err(format!("Failed to parse Gemini response: {}", e)),
        };

        let rewritten_text = &gemini_resp.candidates[0].content.parts[0].text;
        
        let rewritten_node: Node = match serde_json::from_str(rewritten_text) {
            Ok(n) => n,
            Err(e) => {
                error_trace = format!("JSON Parse Error: {}", e);
                continue;
            }
        };

        let mut candidate_arena = arena.clone();
        candidate_arena.nodes[node_index] = rewritten_node;

        match validate_update(&candidate_arena) {
            Ok(_) => return Ok(candidate_arena),
            Err(e) => {
                error_trace = format!("{:?}", e);
            }
        }
    }

    Err("Failed after 3 retries due to neuro-symbolic validation errors".to_string())
}
