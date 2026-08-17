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
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(-1));
        let mut cur = &mut head;

        let mut l1 = list1.as_ref();
        let mut l2 = list2.as_ref();
        loop {
            let next_val = match (l1, l2) {
                (Some(v1), Some(v2)) => {
                    let res;
                    if v1.val < v2.val {
                        res = v1.val;
                        l1 = v1.next.as_ref();
                    } else {
                        res = v2.val;
                        l2 = v2.next.as_ref();
                    }
                    res
                }
                (Some(v1), None) => {
                    l1 = v1.next.as_ref();
                    v1.val
                }
                (None, Some(v2)) => {
                    l2 = v2.next.as_ref();
                    v2.val
                }
                (None, None) => { break; }
            };
            cur.next = Some(Box::new(ListNode::new(next_val)));
            cur = cur.next.as_mut().unwrap();
        }

        head.next
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
