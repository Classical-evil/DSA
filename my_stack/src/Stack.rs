#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }

        self.data.get(self.size - 1)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }

        self.data.get_mut(self.size - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    pub fn iter(&self) -> Iter<'_, T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for i in self.data.iter_mut() {
            iterator.stack.push(i);
        }
        iterator
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            self.stack.pop()
        }
    }
}

pub struct IterMut<'a, T> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            self.stack.pop()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let mut stack: Stack<i32> = Stack::new();

        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_increases_size() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn pop_returns_items_in_lifo_order() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn peek_returns_top_without_removing_it() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);

        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn peek_after_pop_returns_new_top() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);

        stack.pop();
        assert_eq!(stack.peek(), Some(&1));

        stack.pop();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn peek_mut_allows_modifying_the_top() {
        let mut stack = Stack::new();

        stack.push(10);
        match stack.peek_mut() {
            Some(top) => *top += 5,
            None => panic!("栈非空，peek_mut 不应返回 None"),
        }

        assert_eq!(stack.peek(), Some(&15));
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn peek_mut_on_empty_stack_returns_none() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.peek_mut().is_none());
    }

    #[test]
    fn clear_empties_the_stack() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.clear();

        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.pop(), None);

        // 清空后仍可继续正常使用
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn iter_yields_items_from_top_to_bottom() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let items: Vec<&i32> = stack.iter().collect();
        assert_eq!(items, vec![&3, &2, &1]);

        // iter 只借用栈，不会改变其内容
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn iter_on_empty_stack_yields_nothing() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.iter().count(), 0);
    }

    #[test]
    fn iter_mut_modifies_every_item() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        for item in stack.iter_mut() {
            *item *= 10;
        }

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    fn iter_mut_on_empty_stack_yields_nothing() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.iter_mut().count(), 0);
    }

    #[test]
    fn into_iter_consumes_stack_from_top_to_bottom() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let items: Vec<i32> = stack.into_iter().collect();
        assert_eq!(items, vec![3, 2, 1]);
    }

    #[test]
    fn works_with_non_copy_types() {
        let mut stack = Stack::new();

        stack.push(String::from("hello"));
        stack.push(String::from("world"));

        assert_eq!(stack.peek(), Some(&String::from("world")));
        assert_eq!(stack.pop(), Some(String::from("world")));
        assert_eq!(stack.pop(), Some(String::from("hello")));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stress_push_and_pop() {
        let mut stack = Stack::new();

        for i in 0..1000 {
            stack.push(i);
        }
        assert_eq!(stack.size(), 1000);

        for i in (0..1000).rev() {
            assert_eq!(stack.pop(), Some(i));
        }
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }
}
