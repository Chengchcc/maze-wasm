use std::fmt::{self, Debug};

pub trait ElementType : Copy + Debug + Eq + Clone {
    /// Converts from `usize` to the element type.
    ///
    /// Returns `None` if the argument won’t fit in `Self`.
    fn from_usize(n: usize) -> Option<Self>;

    /// Converts from the element type to `usize`.
    fn to_usize(self) -> usize;
}

impl ElementType for usize {
    #[inline]
    fn from_usize(n: usize) -> Option<usize> { Some(n) }
    #[inline]
    fn to_usize(self) -> usize { self }
}

macro_rules! element_type_impl {
    ($type_:ident) => {
        impl ElementType for $type_ {
            #[inline]
            fn from_usize(u: usize) -> Option<Self> {
                let result = u as $type_;
                if result as usize == u { Some(result) } else { None }
            }

            #[inline]
            fn to_usize(self) -> usize {
                self as usize
            }
        }
    }
}

element_type_impl!(u8);
element_type_impl!(u16);
element_type_impl!(u32);
element_type_impl!(u64);

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnionFind<T: ElementType = usize> {
    elements: Vec<T>,
    ranks: Vec<u8>,
}


impl<T: Debug + ElementType> Debug for UnionFind<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "UnionFind({:?})", self.elements)
    }
}


impl<T: ElementType> Default for UnionFind<T> {
    fn default() -> Self {
        UnionFind::new(0)
    }
}


impl<T: ElementType> UnionFind<T> {

    // helpers
    /**
     * 获取element的rank
     */
    fn rank(&self, element: T) -> u8 {
        self.ranks[element.to_usize()]
    }

    /**
     * element的rank提升
     */
    fn increment_rank(&mut self, element: T) {
        let i = element.to_usize();
        self.ranks[i] = self.ranks[i].saturating_add(1)
    }

    /**
     * 获取element的parent
     */
    fn parent(&self, element: T) -> T {
        self.elements[element.to_usize()]
    }

    /**
     * 设置a的parent为b
     */
    fn set_parent(&mut self, a: T, b: T) {
        let _ = std::mem::replace(&mut self.elements[a.to_usize()], b);
    }

    pub fn new(size: usize) -> Self {
        UnionFind {
            elements: (0..size).map(|i| {
                let e = T::from_usize(i).expect("UnionFind::new overflow");
                e
            }).collect(),
            ranks: vec![0; size]
        }
    }

    pub fn union(&mut self, a: T, b:T) -> bool {
        let (a, b) = (self.find(a), self.find(b));
        if a == b {return false;}

        let (rank_a, rank_b) = (self.rank(a), self.rank(b));

        if rank_a > rank_b {
            self.set_parent(b, a);
        } else if rank_b > rank_a {
            self.set_parent(a, b);
        } else {
            self.set_parent(a, b);
            self.increment_rank(b);
        }
        true
    }

    pub fn equiv(& mut self, a:T, b:T) -> bool {
        self.find(a) == self.find(b)
    }


    pub fn find(&mut self, mut element: T) -> T {
        let mut parent = self.parent(element);
        while element != parent {
            let grandpa = self.parent(parent);
            self.set_parent(element, grandpa);
            element = parent;
            parent = grandpa;
        }
        element
    }
}





