use std::ptr::NonNull;
use std::fmt::{self, Display, Formatter};

pub struct Node<T> {
    pub val: T,
    pub next: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    start: Option<NonNull<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            start: None,
            length: 0,
        }
    }

    // 改为将新元素添加到链表的尾部
    pub fn add(&mut self, value: T) {
        let new_node = Box::new(Node {
            val: value,
            next: None,
        });

        let new_node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });

        match self.start {
            None => {
                // 如果链表为空，新节点作为链表的开始
                self.start = new_node_ptr;
            }
            Some(mut node) => {
                // 找到链表的末尾并将新节点添加到末尾
                while let Some(next) = unsafe { node.as_ref().next } {
                    node = next;
                }
                unsafe { node.as_mut().next = new_node_ptr };
            }
        }

        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        match self.start {
            None => None,
            Some(node) => unsafe { node.as_ref().get(index) },
        }
    }

    pub fn reverse(&mut self) {
        let mut prev: Option<NonNull<Node<T>>> = None;
        let mut current = self.start;

        while let Some(mut current_node) = current {
            unsafe {
                let next_node = current_node.as_mut().next;  // 保存下一个节点
                current_node.as_mut().next = prev;  // 当前节点指向前一个节点
                prev = Some(current_node);  // 更新前一个节点
                current = next_node;  // 移动到下一个节点
            }
        }

        self.start = prev;  // 反转后的链表起点是最后一个节点
    }
}

impl<T> Node<T> {
    fn get(&self, index: i32) -> Option<&T> {
        match index {
            0 => Some(&self.val),
            _ => match self.next {
                None => None,
                Some(ext_ptr) => unsafe { ext_ptr.as_ref().get(index - 1) },
            },
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}
