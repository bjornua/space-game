use super::{Item, Quantity};
use super::atoms;

pub const BEAM: Item = Item {
    name: "Beam",
    components: &[
        Quantity { amount: 1400, item: &atoms::ALUMINIUM }
    ]
};

pub BOLT: Item = Item {
    name: "Bolt",
    components
}
