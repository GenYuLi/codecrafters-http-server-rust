use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};
use crate::http_parser::http_response::HttpResponse;
use crate::http_parser::http_request::HttpRequest;

type HandlerFn =  fn(&HttpRequest) -> Result<HttpResponse, Box<dyn Error>>;

#[derive(Default)]
struct TrieNode {
    children: RwLock<HashMap<String, Arc<TrieNode>>>,
    handler: RwLock<Option<HandlerFn>>,
}

pub struct Router {
    root: Arc<TrieNode>,
    wildcard_routes: RwLock<Vec<(String, HandlerFn)>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            root: Arc::new(TrieNode::default()),
            wildcard_routes: RwLock::new(Vec::new()),
        }
    }

    pub(crate) fn add_route(&mut self, path: &str, handler: HandlerFn) -> &mut Router {
        let mut current_node = Arc::clone(&self.root);
        if path.ends_with("/*") {
            {
                // Limit the scope of wildcard_routes' write lock to this block
                let mut wildcard_routes = self.wildcard_routes.write().unwrap();
                wildcard_routes.push((path.trim_end_matches("/*").to_string(), handler));
            }
            // At this point, the write lock is dropped, and we can safely return `self`
            return self;
        }
        for part in path.split('/') {
            if part.is_empty() {
                continue;
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
            println!("{}", part);
            if part.is_empty() {
                continue;
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
                    None => {
                        for (route, handler) in self.wildcard_routes.read().unwrap().iter() {
                            if path.starts_with(route) {
                                return Some(*handler);
                            }
                        }
                        return None;
                    }
                }
            };
            current_node = next_node;
        }
        let handler = *current_node.handler.read().unwrap();
        Some(handler?)
    }
}