use super::{Item, Quantity};
use super::atoms;

pub const WAFER: Item = Item {
    name: "Silicon Wafer",
    components: &[
        Quantity { amount: 14, item: &atoms::SILICON }
    ]
};

