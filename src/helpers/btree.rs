#![allow(unused)]

pub struct BTreeNode<T> {
    pub data: T,
    pub l_child: Option<Box<BTreeNode<T>>>,
    pub r_child: Option<Box<BTreeNode<T>>>,
}
impl<T> BTreeNode<T> {
    pub fn in_order_traversal<F>(node: &Option<Box<BTreeNode<T>>>, f: &F)
    where
        T: Sized,
        F: Fn(&T),
    {
        if let Some(this_node) = node {
            Self::in_order_traversal(&this_node.l_child, f);
            f(&this_node.data);
            Self::in_order_traversal(&this_node.r_child, f);
        };
    }

    pub fn pre_order_traversal<F>(node: &Option<Box<BTreeNode<T>>>, f: &F)
    where
        T: Sized,
        F: Fn(&T),
    {
        if let Some(this_node) = node {
            f(&this_node.data);
            Self::pre_order_traversal(&this_node.l_child, f);
            Self::pre_order_traversal(&this_node.r_child, f);
        };
    }

    pub fn post_order_traversal<F>(node: &Option<Box<BTreeNode<T>>>, f: &F)
    where
        T: Sized,
        F: Fn(&T),
    {
        if let Some(this_node) = node {
            Self::post_order_traversal(&this_node.l_child, f);
            Self::post_order_traversal(&this_node.r_child, f);
            f(&this_node.data);
        };
    }
}
