use rocket::serde::{Serialize};


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Node {
    pub name: String,
    pub description: String
}

impl Node {
  pub fn new(name: String, description: String) -> Node {
    Node {
      name,
      description
    }
  }
}

pub fn get_node_list() -> Vec<Node> {
  vec![
    Node::new("Node 2".to_string(), "Node 1 description".to_string()),
    Node::new("Node 2".to_string(), "Node 2 description".to_string()),
  ]
}