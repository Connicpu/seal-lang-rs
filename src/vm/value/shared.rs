use std::any::Any;
use std::cell::RefCell;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Shared<T> {
    inner: Rc<RefCell<T>>,
}

#[derive(Clone)]
pub struct SharedRef {
    inner: Rc<Any>,
}

impl SharedRef {
    fn ptr(&self) -> *const u8 {
        let ptr = (&*self.inner) as *const Any;
        let pair: (*const u8, *const u8) = unsafe { ::std::mem::transmute(ptr) };
        pair.0
    }
}

impl<T> From<Shared<T>> for SharedRef
    where T: Any
{
    fn from(rc: Shared<T>) -> SharedRef {
        SharedRef { inner: rc.inner }
    }
}

impl PartialEq for SharedRef {
    fn eq(&self, other: &Self) -> bool {
        self.ptr().eq(&other.ptr())
    }
}

impl Eq for SharedRef {}

impl PartialOrd for SharedRef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SharedRef {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ptr().cmp(&other.ptr())
    }
}

impl Hash for SharedRef {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.ptr().hash(state)
    }
}

impl Debug for SharedRef {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "SharedRef({:?})", self.ptr())
    }
}
