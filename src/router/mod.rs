use std::collections::HashMap;
use crate::http_parser::http_response::HttpResponse;
use crate::http_parser::http_request::HttpRequest;

type HandlerFn = fn(&HttpRequest) -> HttpResponse;

#[derive(Default)]
struct TrieNode {
    children: HashMap<String, TrieNode>,
    handler: Option<HandlerFn>,
}

pub struct Router {
    root: TrieNode,
    wildcard_routes: Vec<(String, HandlerFn)>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            root: TrieNode::default(),
            wildcard_routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, handler: HandlerFn) -> &mut Router {
        let mut current_node = &mut self.root;
        if (path.ends_with("/*")) {
            self.wildcard_routes.push((path.trim_end_matches("/*").to_string(), handler));
            return self;
        }
        for part in path.split('/') {
            if part.is_empty() {
                continue;
            }
            current_node = current_node.children.entry(part.to_string()).or_insert(TrieNode::default());
        }
        current_node.handler = Some(handler);
        self
    }

    pub fn find_route(&self, path: &str) -> Option<HandlerFn> {
        let mut current_node = &self.root;
        // ignore last string path as a parameter
        for part in path.split('/') {
            println!("{}", part);
            if part.is_empty() {
                continue;
            }
            match current_node.children.get(part) {
                Some(node) => current_node = node,
                None => {
                    for (route, handler) in &self.wildcard_routes {
                        if path.starts_with(route) {
                            return Some(*handler);
                        }
                    }
                    return None;
                }
            }
        }
        current_node.handler
    }
}