const CAPACITOR {

}
pub const SOLAR_CELL: Item = Item {
    name: "Solar Cell",
    components: &[Quantity{item: &silicon::WAFER, amount: 1}],
};

pub const SOLAR_ARRAY: Item = Item {
    name: "Solar Array",
    components: &[
        Quantity{item: &SOLAR_CELL, amount: 25},
        Quantity{item: &structure::BEAM, amount: 12},
    ],
};


pub const SOLAR_ARRA
