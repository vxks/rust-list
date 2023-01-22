use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum List<A> {
    Empty,
    Cons(A, Box<List<A>>),
}

impl<A> List<A> {
    pub fn head_option(self) -> Option<A> {
        match self {
            List::Empty => None,
            List::Cons(head, _) => Some(head)
        }
    }

    pub fn tail(self) -> List<A> {
        match self {
            List::Empty => List::Empty,
            List::Cons(_, tail) => *tail
        }
    }

    pub fn flat_map<B>(self, f: fn(A) -> List<B>) -> List<B> {
        let folder = |acc: List<B>, a: A| {
            let list_b = f(a);
            acc.fold_right(list_b, |b, list_b| List::cons(b, list_b))
        };
        self.fold_left(List::Empty, folder)
    }

    pub fn map<B>(self, f: fn(A) -> B) -> List<B> {
        let folder = |list_b, a| {
            List::cons(f(a), list_b)
        };
        self.fold_left(List::Empty, folder).reverse()
    }

    pub fn filter(self, f: fn(&A) -> bool) -> List<A> {
        self.fold_right(
            List::empty(),
            |a, acc| {
                if f(&a) { List::cons(a, acc) } else { acc }
            },
        )
    }

    pub fn reverse(self) -> List<A> {
        self.fold_left(List::empty(), |b, a| List::cons(a, b))
    }

    pub fn fold_right<B, F>(self, empty: B, f: F) -> B where
        F: Fn(A, B) -> B
    {
        List::_fold_left(self.reverse(), empty, |b, a| f(a, b))
    }

    fn _fold_left<B, F>(left: List<A>, acc: B, mut f: F) -> B where
        F: FnMut(B, A) -> B
    {
        match left {
            List::Empty => acc,
            List::Cons(head, tail) =>
                List::_fold_left(*tail, f(acc, head), f)
        }
    }

    pub fn fold_left<B, F>(self, empty: B, f: F) -> B
        where F: Fn(B, A) -> B {
        List::_fold_left(self, empty, f)
    }

    pub fn append(self, a: A) -> List<A> {
        self.fold_right(
            List::cons(a, List::empty()),
            |a, b| List::cons(a, b),
        )
    }

    pub fn prepend_list(self, that: List<A>) -> List<A> {
        that.fold_right(self, |a, acc| List::cons(a, acc))
    }

    pub fn empty() -> List<A> {
        List::Empty
    }

    pub fn cons(head: A, tail: List<A>) -> List<A> {
        List::Cons(head, Box::new(tail))
    }
}

impl<A: Display> fmt::Display for List<A> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            List::Empty => write!(fmt, "", ),
            List::Cons(head, tail) =>
                write!(fmt, "[{}, {}]", tail.to_string(), head)
        }
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> List<A> {
        iter.into_iter().fold(List::empty(), |xs, x| {
            List::cons(x, xs)
        }).reverse()
    }
}