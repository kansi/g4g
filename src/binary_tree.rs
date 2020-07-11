mod binary_tree {
    #[derive(Debug, Default)]
    pub struct Node<'a> {
        pub value: u32,
        pub left: Option<&'a Node<'a>>,
        pub right: Option<&'a Node<'a>>,
    }

    pub fn new_node<'a>(value: &'a u32) -> Node {
        Node {
            value: *value,
            ..Default::default()
        }
    }

    pub fn preorder(node: &Node) {
        print!("{} ", node.value);
        match node.left {
            Some(node_left) => preorder(node_left),
            None => (),
        }

        match node.right {
            Some(node_right) => preorder(node_right),
            None => (),
        }
    }

    pub fn inorder(node: &Node) {
        match node.left {
            Some(node_left) => inorder(node_left),
            None => (),
        }

        print!("{} ", node.value);

        match node.right {
            Some(node_right) => inorder(node_right),
            None => (),
        }
    }

    pub fn postorder(node: &Node) {
        match node.left {
            Some(node_left) => postorder(node_left),
            None => (),
        }

        match node.right {
            Some(node_right) => postorder(node_right),
            None => (),
        }

        print!("{} ", node.value);
    }

    // TODO
    pub fn bfs<'a>(
        node: &Node,
        height: u32,
        parent_index: usize,
        acc: &'a mut Vec<u32>,
    ) -> &'a mut Vec<u32> {
        let level_index = 2_usize.pow(height + 1) - 1;

        match node.left {
            Some(node_left) => {
                let node_index = level_index + parent_index;
                acc[node_index] = node_left.value;
                bfs(node_left, height + 1, parent_index, acc);
            }

            None => (),
        }

        match node.right {
            Some(node_right) => {
                let node_index = level_index + parent_index + 1;
                acc[node_index] = node_right.value;
                bfs(node_right, height + 1, parent_index + 1, acc);
            }

            None => (),
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree() {
        let mut root = binary_tree::new_node(&1);
        let mut node2 = binary_tree::new_node(&2);
        let node3 = binary_tree::new_node(&3);
        let node4 = binary_tree::new_node(&4);
        let node5 = binary_tree::new_node(&5);

        node2.left = Some(&node4);
        node2.right = Some(&node5);
        root.left = Some(&node2);
        root.right = Some(&node3);

        print!("preorder: ");
        binary_tree::preorder(&root);
        println!("");

        print!("inorder: ");
        binary_tree::inorder(&root);
        println!("");

        print!("inorder: ");
        binary_tree::postorder(&root);
        println!("");

        let mut bfs: Vec<u32> = vec![0_u32; 4];
        bfs.insert(0, root.value);
        binary_tree::bfs(&root, 0, 0, &mut bfs);
        println!("{:?}", bfs);

        assert_eq!(1, 1);
    }
}
