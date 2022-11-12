use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    // Node持有自身所有的子节点并通过变量来共享它们的所用权
    // 从而使我们可以直接访问树中的每个Node
    // 使用RefCell<T>包裹Vec<Rc<Node>>来实现内部可变性
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    // 创建leaf叶子节点和包含leaf子节点的branch节点
    // leaf中的Node现在分别拥有了leaf与branch两个所有者
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node{
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // 增加指向父节点的Weak<Node>引用
    // 获取branch中Rc<Node>的Weak<Node>引用，并将它存入leaf的parent字段中
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// leaf parent = Some(Node { value: 5, children: RefCell { value: [Node { value: 3, children: RefCell { value: [] }, parent: RefCell { value: (Weak) } }] }, parent: RefCell { value: (Weak) } })