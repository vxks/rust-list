use std::fmt;
use std::fmt::{Display, Formatter, Pointer};
use std::ops::{Add, Deref};
use crate::List::{Cons, Empty};

fn main() {
    let list: List<i32> = List::from_iter([1, 2, 3]);
    let list1 = list.append(5);
    list_print(list1);
    println!("{}", list);
    // println!("{}", &list.head_option().unwrap());

    let sum_folder = |acc: i32, a: i32| acc + a;
    let mapper = |n: i32| n + 1;
    let flat_mapper = |n: i32| List::cons(n + 1, Empty);

    // let fold_result = list.fold_left(0, sum_folder);
    // let map_result = &list.map(mapper);
    // let flat_map_result = &list.flat_map(flat_mapper);
    // println!("flat_map_result: {}", flat_map_result.to_string());
    // println!("map_result: {}", map_result.to_string());
    // println!("flat_map_result: {}", flat_map_result.to_string())
}

enum List<A> {
    Empty,
    Cons(A, Box<List<A>>),
}

impl<A> List<A> {
    fn head_option(self) -> Option<A> {
        match self {
            Empty => None,
            Cons(head, _) => Some(head)
        }
    }

    fn tail(self) -> List<A> {
        match self {
            Empty => Empty,
            Cons(_, tail) => *tail
        }
    }

    fn flat_map<B>(self, f: fn(A) -> List<B>) -> List<B> {
        let folder = |acc: List<B>, a: A| {
            let list_b = f(a);
            acc.fold(list_b, |list_b, b| List::cons(b, list_b))
        };
        self.fold(Empty, folder)
    }

    fn map<B>(self, f: fn(A) -> B) -> List<B> {
        let folder = |list_b, a| {
            List::cons(f(a), list_b)
        };
        self.fold(Empty, folder)
    }

    fn _fold<B, F>(left: List<A>, acc: B, mut f: F) -> B where
        F: FnMut(B, A) -> B
    {
        match left {
            Empty => acc,
            Cons(head, tail) =>
                List::_fold(*tail, f(acc, head), f)
        }
    }

    fn fold<B, F>(self, empty: B, f: F) -> B where
        F: Fn(B, A) -> B
    {
        List::_fold(self, empty, f)
    }

    fn append(self, a: A) -> List<A> {
        List::cons(a, Empty).prepend_list(self)
    }

    fn prepend_list(self, that: List<A>) -> List<A> {
        self.fold(that, |acc, a| List::cons(a, acc))
    }

    fn empty() -> List<A> {
        Empty
    }

    fn cons(head: A, tail: List<A>) -> List<A> {
        Cons(head, Box::new(tail))
    }
}

fn list_print<A: ToString>(list: List<A>) -> () {
    _list_print(list, String::new())
}

fn _list_print<A: ToString>(list: List<A>, agg: String) -> () {
    match list {
        Empty => println!("[{}_]", agg),
        Cons(head, tail) =>
            _list_print(*tail, head.to_string().add(", ").add(agg.as_str()))
    }
}


impl<A: Display> fmt::Display for List<A> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Empty => write!(fmt, "", ),
            Cons(head, tail) =>
                write!(fmt, "[{}, {}]", tail.to_string(), head)
        }
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> List<A> {
        iter.into_iter().fold(List::empty(), |xs, x| {
            xs.append(x)
        })
    }
}
