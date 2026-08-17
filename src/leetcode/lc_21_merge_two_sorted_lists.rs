#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}
pub struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur = &mut l1;
        while l2.is_some() {
            if cur.is_none() || cur.as_ref().unwrap().val > l2.as_ref().unwrap().val {
                std::mem::swap(cur, &mut l2);
            }
            cur = &mut cur.as_mut()?.next;
        }
        l1
    }
}

fn gen_from_arr(v: &[i32]) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(-1));
    let mut cur = &mut head;
    for val in v {
        cur.next = Some(Box::new(ListNode::new(*val)));
        cur = cur.next.as_mut().unwrap();
    }
    head.next
}

fn main() {
    println!("{:?}", Solution::merge_two_lists(gen_from_arr(&[1, 2, 4]), gen_from_arr(&[1, 3, 4])));
}
