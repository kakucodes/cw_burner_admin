use cw_storage_plus::Map;

pub const AMOUNT_BURNED: Map<String, u128> = Map::new("amount_burned");
