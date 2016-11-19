use std::collections::{HashMap, VecDeque};
// use std::iter::;

mod atoms;
mod silicon;
use std::error::Error as StdError;
// mod structure;
// mod electrical;


#[derive(Debug)]
pub struct Items(HashMap<&'static str, Item>);

#[derive(Debug)]
pub enum Error {
    Unresolvable(VecDeque<Item>),
}
use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Unresolvable(ref tail) => write!(f, "{}:\n{:#?}", self.description(), tail),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &'static str {
        match *self {
            Error::Unresolvable(_) => "Could not resolve items",
        }
    }
}

impl Items {
    pub fn compile() -> Result<Self, Error> {
        let builtin: Vec<Item> = silicon::ITEMS.into_iter()
            .cloned()
            .chain(atoms::ITEMS.into_iter()
                .map(|x| {
                    Item {
                        name: x,
                        components: &[],
                    }
                }))
            .collect();

        let mut tail: VecDeque<Item> = VecDeque::new();
        let mut items = HashMap::new();

        for item in builtin {
            if item.components.into_iter().all(|quantity| items.contains_key(quantity.item)) {
                items.insert(item.name, item);
            } else {
                tail.push_back(item);
            }
        }
        let mut skipped = 0;
        while let Some(item) = tail.pop_front() {
            if skipped > tail.len() {
                tail.push_back(item);
                return Err(Error::Unresolvable(tail));
            }
            if item.components.into_iter().all(|quantity| items.contains_key(quantity.item)) {
                skipped = 0;
                items.insert(item.name, item);
            } else {
                skipped += 1;
                tail.push_back(item);
            }
        }
        Ok(Items(items))
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: &'static str,
    pub components: &'static [Quantity],
}

// use std::hash::{Hash, Hasher};
// impl Hash for Item {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//     }
// }


#[derive(Debug)]
pub struct Quantity {
    pub item: &'static str,
    pub amount: u64,
}


pub struct Container {
    pub id: u64,
    pub items: Vec<Quantity>,
}
