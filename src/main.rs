use core::cmp::PartialOrd;
use core::fmt::Display;
use std::cell::RefCell;
use std::rc::Rc;

enum Color {
    RED,
    BLACK,
}

struct Node<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Option<Rc<RefCell<Node<K, V>>>>,
    right: Option<Rc<RefCell<Node<K, V>>>>,
    parent: Option<Rc<RefCell<Node<K, V>>>>,
}

trait RedBlackTree<K: PartialOrd, V> {
    fn put(&mut self, key: K, value: V);
    fn left_rotate(x: &Rc<RefCell<Node<K, V>>>);
    fn right_rotate(x: &Rc<RefCell<Node<K, V>>>);
}

impl<K: PartialOrd, V: Display> Node<K, V> {
    fn new(k: K, v: V) -> Node<K, V> {
        Self {
            key: k,
            value: v,
            color: Color::RED,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    fn is_left(&self) -> bool {
        match self.parent {
            Some(ref parent) => match parent.borrow().left {
                Some(ref parent_left) => parent_left.borrow().key == self.key,
                None => false,
            },
            None => false,
        }
    }

    fn is_right(&self) -> bool {
        match self.parent {
            Some(ref parent) => match parent.borrow().right {
                Some(ref parent_right) => parent_right.borrow().key == self.key,
                None => false,
            },
            None => false,
        }
    }
}

impl<K: PartialOrd, V: core::fmt::Display> RedBlackTree<K, V> for Node<K, V> {
    fn put(&mut self, key: K, value: V) {
        // create a new node.
        let new_node = Some(Box::new(Node::new(key, value)));
    }

    // 整个旋转方法, 让我被编译器整整刁难了一天多.
    // 编译器告诉我, 我没法直接修改一个借用(*理解成引用和指针都行*)里的某个属性.
    // 红黑树在左右旋的时候, 时需要改变几个节点的指针. 有些节点我需要多重借用才能借到.
    // 由于我没法在一个节点借用(可变借用也不行)里修改它所借用的另一个节点的属性.
    // 这个有点复杂, 我修改属性可能会导致我失去当前节点的所有权, 或者提示我该节点是一个借用.

    // 后来我又读了一遍Rust圣经
    // 可以使用这个智能指针Rc<RefCell<T>>来存放多个同对象的引用
    // Rc<T> 可以保证对象能被多次引用 使用clone来多复制几个指针
    // RefCell<T> 可以修改引用的对象 borrow获取一个只读借用 borrow_mut获取一个可写的借用

    /**
     *
     *           x                             y
     *          / \                           / \
     *         /   \           =>            /   \
     *       1       y                      x     3
     *              / \                    / \
     *             /   \                  /   \
     *            2     3                1     2
     */
    fn left_rotate(x: &Rc<RefCell<Node<K, V>>>) {
        // 拿到 Y
        let y = match x.borrow().right.clone() {
            Some(v) => v,
            None => return,
        };

        // 把 y 的 left 变成 x 的 right
        x.borrow_mut().right = y.borrow().left.clone();
        // 为 x 的新 right 更新家庭关系
        if let Some(ref x_r) = x.borrow_mut().right {
            x_r.borrow_mut().parent = Some(x.clone());
        }

        // 得到 x 的父节点
        let parent = x.borrow().parent.clone();

        // 为 x 的父节点更换家庭关系, 因为x要下沉了
        if let Some(ref p) = parent {
            if x.borrow().is_left() {
                p.borrow_mut().left = Some(y.clone());
            }
            if x.borrow().is_right() {
                p.borrow_mut().right = Some(y.clone());
            }
        }

        // 为y换上新父母
        y.borrow_mut().parent = parent;
        // y成为x的父母
        x.borrow_mut().parent = Some(y.clone());
    }

    fn right_rotate(x: &Rc<RefCell<Node<K, V>>>) {}
}

fn main() {
    println!("Hello, world!");

    let tree = Node::new(1, 2);
}
