#![allow(unused)]

pub struct TreeNode<T> {
    pub data: T,
    pub children: Vec<TreeNode<T>>,
}
