mod atoms;
// mod silicon;
// mod structure;
// mod electrical;

pub struct Items {

};

pub fn compile_items () -> Items {

};



pub struct Item {
    pub name: &'static str,
    pub components: &'static [Quantity]
}

pub struct Quantity {
    pub item: &'static Item,
    pub amount: u64
}


pub struct Container {
    pub id: u64,
    pub items: Vec<Quantity>
}


