pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, test: &str) {
        self.content.push_str(test);
    }

    pub fn content(&self) -> &str {
        // 调用as_ref时得到Option<&Box<dyn State>>
        self.state.as_ref().unwrap().content(&self)
    }

    // 请求审批文章的功能，将文章的状态从Draft变为PendingReview。
    pub fn request_review(&mut self) {
        // request_review方法需要获取状态值的所有权
        // 取出state字段的Some值
        // 并在原来的位置留下一个None。
        if let Some(s) = self.state.take() {
            // 我们需要临时把state设置为None来取得state值的所有权，
            // 而不能直接使用self.state = self.state.request_review();
            // 确保Post无法在我们完成状态转换后再次使用旧的state值
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 为content方法添加了默认的trait实现，它会返回一个空的字符串切片
    // 使得我们可以不必在Draft和PendingReview结构体中重复实现content。
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // 只能被包裹着当前类型的Box实例调用
    // 在调用过程中获取Box<Self>的所有权并使旧的状态失效，
    // 从而将Post的状态值转换为一个新的状态。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
