use sha2::{Sha256, Digest};

pub struct MerkleTree {
    pub root: Node
}

#[derive(Clone)]
pub struct Node {
    pub left: Option<Box<NodeType>>,
    pub right: Option<Box<NodeType>>,
    pub hash: Option<Vec<u8>>
}

impl Node {
    fn new(left: NodeType, right: NodeType, hash: Vec<u8>) -> Node {
        Node {
            left: Option::Some(Box::new(left)),
            right: Option::Some(Box::new(right)),
            hash: Option::Some(hash)
        }
    }
}

#[derive(Clone)]
pub enum NodeType {
    // for now the data is represented as string
    DataNode(String),
    HashNode(Node)
}

impl MerkleTree {
    pub fn new(data: &[String]) -> MerkleTree {
        let mut data_vec = Vec::from(data);
        // clone the last element if they are an odd number
        if data_vec.len() % 2 != 0 {
            let last = data_vec.last().cloned().unwrap();
            data_vec.push(last);
        }

        let nodes: Vec<NodeType> = data_vec.into_iter().map(|d| NodeType::DataNode(d)).collect();

        let root = MerkleTree::get_root_node(nodes);

        MerkleTree {
            root: root
        }
    }

    fn get_root_node(nodes: Vec<NodeType>) -> Node {
        if nodes.len() == 1 {
            return match nodes[0].clone() {
                NodeType::HashNode(n) => n,
                NodeType::DataNode(_) => {
                    panic!("Root node is a data node.");
                }
            };
        }

        let mut new_row: Vec<NodeType> = Vec::new();

        let mut index = 0;
        while index < nodes.len() {
            let mut hasher = Sha256::default();

            // making a binary tree
            for _ in 0..2 {
                let hash_data = match nodes[index].clone() {
                    NodeType::DataNode(s) => Vec::from(s.as_bytes()),
                    NodeType::HashNode(n) => Vec::from(n.hash.unwrap())
                };
                hasher.input(&hash_data);
                index += 1;
            }

            let hash_result = hasher.result();

            let new_node = Node::new(
                nodes[index - 2].clone(),
                nodes[index - 1].clone(),
                Vec::from(&hash_result[..]));
            let new_hash_node = NodeType::HashNode(new_node);

            new_row.push(new_hash_node);
        }

        return MerkleTree::get_root_node(new_row);
    }
}

#[test]
fn merkle_tree_constructs() {
    let test_data: [String; 2] = [String::from("abc"), String::from("bca")];
    let tree = MerkleTree::new(&test_data[..]);
    let left = *tree.root.left.unwrap();
    let right = *tree.root.right.unwrap();
    match left {
        NodeType::DataNode(s) => assert_eq!(s, "abc"),
        NodeType::HashNode(_) => panic!("it should be 2 layers deep")
    };

    match right {
        NodeType::DataNode(s) => assert_eq!(s, "bca"),
        NodeType::HashNode(_) => panic!("it should be 2 layers deep")
    };
}
