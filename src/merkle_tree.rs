extern crate sha2;

pub struct MerkleTree<'a> {
    root: NodeType<'a>
}

#[derive(Clone)]
struct Node<'a> {
    left: Option<Box<NodeType<'a>>>,
    right: Option<Box<NodeType<'a>>>,
    hash: &'a [u8],
}

#[derive(Clone)]
enum NodeType<'a> {
    // for now the data is represented as string
    DataNode(String),
    HashNode(Node<'a>)
}

impl<'a> MerkleTree<'a> {
    // TODO: use some algorithm that depending on the data count
    // determines the depth of the tree and does some
    // recursive thing
    pub fn new(data: &[String]) -> MerkleTree<'a> {
        let mut data_vec = Vec::from(data);
        if data_vec.len() % 2 != 0 {
            let last = data_vec.last().cloned().unwrap();
            data_vec.push(last);
        }

        let nodes = data_vec.into_iter().map(|d| NodeType::DataNode(d)).collect::<Vec<_>>();;
        MerkleTree {
            root: nodes[0].clone()
        }
    }
}

#[test]
fn merkle_tree_constructs() {
    let test_data: [String; 1] = [String::from("abc")];
    let tree = MerkleTree::new(&test_data[..]);
}
