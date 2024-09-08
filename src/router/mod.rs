use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, RwLock};
use crate::http_parser::http_response::HttpResponse;
use crate::http_parser::http_request::HttpRequest;

type HandlerFn =  fn(&HttpRequest) -> Result<HttpResponse, Box<dyn Error>>;

#[derive(Default)]
struct TrieNode {
    children: RwLock<HashMap<String, Arc<TrieNode>>>,
    handler: RwLock<Option<HandlerFn>>,
    wildcard: Arc<RwLock<bool>>,
}

pub struct Router {
    root: Arc<TrieNode>,
}

impl Router {
    pub const FILEPATH : &'static str = "/tmp/data/codecrafters.io/http-server-tester/";
    pub fn new() -> Router {
        Router {
            root: Arc::new(TrieNode::default()),
        }
    }

    pub(crate) fn add_route(&mut self, path: &str, handler: HandlerFn) -> &mut Router {
        let mut current_node = Arc::clone(&self.root);
        for part in path.split('/') {
            if part.is_empty() {
                continue;
            }
            if part == "*" {
                *current_node.wildcard.write().unwrap() = true;
                break;
            }
            // This is a workaround to avoid holding the write lock after or_insert_with returns
            let next_node = { 
                let mut children = current_node.children.write().unwrap();
                children.entry(part.to_string()).or_insert_with(|| Arc::new(TrieNode::default())).clone()
            };
            // Because write lock is still held by children, we can't use the following code
            // let mut children = current_node.children.write().unwrap();
            // let next_node = children.entry(part.to_string()).or_insert_with(|| Arc::new(TrieNode::default())).clone();
            current_node = next_node;
        }
        *current_node.handler.write().unwrap() = Some(handler);
        self
    }

    pub(crate) fn find_route(&self, path: &str) -> Option<HandlerFn> {
        let mut current_node = Arc::clone(&self.root);
        // ignore last string path as a parameter
        for part in path.split('/') {
            println!("part: [{}]", part);
            if part.is_empty() {
                continue;
            }
            let wildcard = {
                *current_node.wildcard.read().unwrap()
            };
            println!("wildcard: {}", wildcard);
            if wildcard {
                break;
            }
            // Because we are using read lock, we can't use the following code
            // let children = current_node.children.read().unwrap().get(part)?;
            // match children.get(part) {
            //    Some(node) => current_node = Arc::clone(node),
            //   None => ...,
            // }
            // Instead, we need to use the following code
            let next_node = {
                let children = current_node.children.read().unwrap();
                match children.get(part) {
                    Some(node) =>  Arc::clone(node),
                    None => return None,
                }
            };
            current_node = next_node;
        }
        let handler = *current_node.handler.read().unwrap();
        Some(handler?)
    }
}