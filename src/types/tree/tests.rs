// pub(self) use super::*;

// #[cfg(test)]
// mod get_nodes_grouped_by_sibling {
//   pub(self) use super::*;

//   #[test]
//   fn case_a() {
//     //
//     let root_node = TreeNode::new(TreeNodeInput { parent_id: None });
//     let root_node_id = root_node.id.clone();

//     let node_a = TreeNode::new(TreeNodeInput {
//       parent_id: Some(root_node_id),
//     });
//     let node_a_id = node_a.id;

//     let node_b = TreeNode::new(TreeNodeInput {
//       parent_id: Some(root_node_id),
//     });
//     let node_b_id = node_b.id;

//     let nodes = vec![root_node, node_a, node_b];

//     let tree = Tree {
//       root_node_id: Some(root_node_id),
//       nodes,
//     };

//     let result = tree.get_nodes_grouped_by_siblings_top_down();

//     assert_eq!(
//       result,
//       vec![
//         (None, vec![root_node_id]),
//         (Some(root_node_id), vec![node_a_id, node_b_id])
//       ]
//     )
//   }

//   #[test]
//   fn case_b() {
//     //
//     let root_node = TreeNode::new(TreeNodeInput { parent_id: None });
//     let root_node_id = root_node.id.clone();

//     let node_a = TreeNode::new(TreeNodeInput {
//       parent_id: Some(root_node_id),
//     });
//     let node_a_id = node_a.id;

//     let node_b = TreeNode::new(TreeNodeInput {
//       parent_id: Some(root_node_id),
//     });
//     let node_b_id = node_b.id;

//     let node_c = TreeNode::new(TreeNodeInput {
//       parent_id: Some(node_b_id),
//     });
//     let node_c_id = node_c.id;

//     let node_d = TreeNode::new(TreeNodeInput {
//       parent_id: Some(node_b_id),
//     });
//     let node_d_id = node_d.id;

//     let tree = Tree {
//       root_node_id: Some(root_node_id),
//       nodes: vec![root_node, node_a, node_b, node_c, node_d],
//     };

//     let result = tree.get_nodes_grouped_by_siblings_top_down();

//     assert_eq!(
//       result,
//       vec![
//         (None, vec![root_node_id]),
//         (Some(root_node_id), vec![node_a_id, node_b_id]),
//         (Some(node_b_id), vec![node_c_id, node_d_id])
//       ]
//     )
//   }
// }
