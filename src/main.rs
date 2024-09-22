use std::collections::HashMap;
fn main() {

    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>
    }
         
    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode {
                next: None,
                val
            }
        }
    }
    
    pub fn two_sum(_numbers: Vec<i32>, _target: i32) -> Vec<i32> {
        let mut _hash: HashMap<i32,i32> = HashMap::new();

        for id in 0.._numbers.len() {
            let remainder = &(_target - &_numbers[id]);
            let int_id: i32 = id as i32;

            if _hash.contains_key(&_numbers[id]) && !_hash.contains_key(remainder) {
                continue
            }

            if _hash.contains_key(remainder) {
                let find_id = *_hash.get(remainder).unwrap();
                return [find_id, int_id].to_vec();
            }

            if !_hash.contains_key(&_numbers[id]) {
                _hash.insert(_numbers[id], int_id );
            };
        };

        return [].to_vec();
    }

    let mut numbers = [3,1,1,1,2,54,1,4,2,4,3,6,5,2,4].to_vec();

    let res = two_sum(numbers, 6);

    println!("Needed number pos for 6 from [3,1,1,1,2,54,1,4,2,4,3,6,5,2,4] is {} {}", res[0], res[1]);

    numbers = [1,2,3,4,5,6,7,8,9,0,9].to_vec();

    let res = two_sum(numbers, 18);

    println!("Needed number pos for 10 from [1,2,3,4,5,6,7,8,9,0,9] is {} {}", res[0], res[1]);
    

    pub fn is_palindrome(x: i32) -> bool {
        let front = x.to_string();
        let back = reverse_string(&front);

        if front == back {
            return true;
        }

        return false;
    }

    pub fn is_palindrome_math(x: i32) -> bool {
        let mut _rev = 0;
        let mut _num = x;

        while _num > _rev {
            _rev = _rev * 10 + _num % 10;
            _num /= 10; 
        }

        _num == _rev || _num == _rev / 10
    }


    pub fn reverse_string(_str: &String) -> String {
        return _str.chars().rev().collect();
    }

    let num = 10;
    let res = is_palindrome(num);

    println!("Palindrome test for {} is {}",num, res);

    let num = 999;
    let res = is_palindrome(num);

    println!("Palindrome test for {} is {}",num, res);

    let num = 10;
    let res = is_palindrome_math(num);

    println!("Palindrome test(math) for {} is {}",num, res);

    let num = 999;
    let res = is_palindrome_math(num);

    println!("Palindrome test(math) for {} is {}",num, res);


    pub fn add_two_numbers_bad(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut str_num1 = String::new();
        let mut str_num2 = String::new();
    
        if l1.is_some() {
            let mut current = l1;
            while let Some(ref mut n1) = current {
               let ins = n1.val.to_string().chars().collect::<Vec<_>>()[0];
               str_num1.insert(0, ins); 
               current = n1.next.take();
            }   
        }

        if l2.is_some() {
            let mut current = l2;
            while let Some(ref mut n2) = current {
               let ins = n2.val.to_string().chars().collect::<Vec<_>>()[0];
               str_num2.insert(0, ins); 
               current = n2.next.take();
            }   
        }

        //manually add the numbers 1 by 1, thanks leetcode for not letting me use bigint
        let max_len = std::cmp::max(str_num1.len(),str_num2.len());
        str_num1 = format!("{:0>width$}", str_num1, width = max_len);
        str_num2 = format!("{:0>width$}", str_num2, width = max_len);

        let chars_num1 = str_num1.chars().rev();
        let chars_num2 = str_num2.chars().rev().collect::<Vec<_>>();

        let mut str_res = String::new();
        let mut overflow = 0;

        for (id, char) in chars_num1.enumerate() {
            let n1 = char.to_digit(10).unwrap() as i32;
            let n2 = chars_num2[id].to_digit(10).unwrap() as i32;

            let mut digit = n1 + n2 + overflow;
            overflow = 0;

            if digit >= 10 {
                digit = digit -10;
                overflow = 1;
            }

            str_res.insert(0, digit.to_string().chars().collect::<Vec<_>>()[0]);
        }

        if overflow == 1 {
            str_res.insert(0, overflow.to_string().chars().collect::<Vec<_>>()[0]);
        }

        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;

        for num in str_res.chars().rev() {
            let digit = num.to_digit(10).unwrap() as i32;
            let new_node = Box::new(ListNode::new(digit));
            *tail = Some(new_node);

            tail = &mut tail.as_mut().unwrap().next;
        }
    
        return head;
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None; // Return for later
        let mut tail = &mut head; // used for working to not lose track of head
        let mut carry = 0;

        // mutable input
        let mut p1 = l1; 
        let mut p2 = l2; 

        // keep construction output as long as we have values to work with
        while p1.is_some() || p2.is_some() || carry > 0 {
            //add carry first
            let mut val = carry;
            carry = 0;

            if let Some(ref mut node) = p1 {
                val += node.val;
                p1 = node.next.take();
            }

            if let Some(ref mut node) = p2 {
                val += node.val;
                p2 = node.next.take();
            }

            // carry over, can never get more than 19 out of p1.val + p2.val + carry
            if val >= 10 {
                carry = 1;
                val -= 10;
            }

            let new_node = Box::new(ListNode::new(val));

            *tail = Some(new_node);
            
            tail = &mut tail.as_mut().unwrap().next;
        }

        return head;

    }

    pub fn print_linked_list(_list: &Option<Box<ListNode>>, _reverse: bool) -> String {
        let mut travel = _list;
        let mut out = Vec::new();

        while let Some(ref node) = travel {
            let cur = (node.val as u8 + b'0') as char;
            
            out.push(cur);

            travel = &node.next;
        }

        if _reverse {
            out.reverse();
        }

        return out.into_iter().collect();
    }

    let list1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None,
            })),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));

    let str1 = print_linked_list(&list1, true);
    let str2 = print_linked_list(&list2, true);

    let res = add_two_numbers(list1, list2);

    println!("Linked list number {} plus {} is equal to {}", str1, str2, print_linked_list(&res, true));
}
