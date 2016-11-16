mod atoms;

pub enum Item {
    Component {
        classname: &'static str,
        description: &'static str,
        components: &'static [Quantity]
    },
    Atom {
        name: &'static str,
        mass: u64
    }
}

pub struct Quantity {
    pub item: &'static Item,
    pub amount: u64
}

pub struct Container {
    pub id: u64,
    pub items: Vec<Quantity>
}

pub const SOLAR_CELL: Item = Item::Component {
    classname: "Solar Cell",
    description: "Generates 1 watt of power per hour",
    components: &[Quantity{item: &atoms::SILICON, amount: 400000}],
};


