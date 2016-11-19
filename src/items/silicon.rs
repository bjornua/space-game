use super::{Item, Quantity};

pub const ITEMS: &'static [Item] = &[Item {
                                         name: "Silicon Wafer",
                                         components: &[Quantity {
                                                           amount: 14,
                                                           item: "Silicon",
                                                       }],
                                     }];
