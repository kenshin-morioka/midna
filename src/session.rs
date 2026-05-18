use crate::providers::Message;

#[derive(Debug, Default)]
pub struct Session {
    messages: Vec<Message>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_system_prompt(prompt: impl Into<String>) -> Self {
        Self {
            messages: vec![Message::system(prompt)],
        }
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn pop(&mut self) -> Option<Message> {
        self.messages.pop()
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::Role;

    #[test]
    fn pushes_messages_in_order() {
        let mut s = Session::new();
        s.push(Message::user("hi"));
        s.push(Message::assistant("hello"));

        assert_eq!(s.len(), 2);
        assert_eq!(s.messages()[0].role, Role::User);
        assert_eq!(s.messages()[0].content, "hi");
        assert_eq!(s.messages()[1].role, Role::Assistant);
        assert_eq!(s.messages()[1].content, "hello");
    }

    #[test]
    fn system_prompt_is_first() {
        let s = Session::with_system_prompt("you are midna");
        assert_eq!(s.len(), 1);
        assert_eq!(s.messages()[0].role, Role::System);
    }
}
