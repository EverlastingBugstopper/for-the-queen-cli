use serde::{Deserialize, Serialize};

use std::fmt;

use crate::{titleize, Recipe};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Good {
    Fuel(Fuel),
    CraftingResource(CraftingResource),
    BuildingMaterial(BuildingMaterial),
    ConsumableItem(ConsumableItem),
    SimpleFood(SimpleFood),
    ComplexFood(ComplexFood),
    Clothing(Clothing),
}

impl fmt::Display for Good {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fuel(fuel) => titleize(fuel),
                Self::CraftingResource(crafting_resource) => titleize(crafting_resource),
                Self::BuildingMaterial(building_material) => titleize(building_material),
                Self::ConsumableItem(consumable_item) => titleize(consumable_item),
                Self::SimpleFood(simple_food) => titleize(simple_food),
                Self::ComplexFood(complex_food) => titleize(complex_food),
                Self::Clothing(clothing) => titleize(clothing),
            }
        )
    }
}

impl Recipe for Good {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Fuel(fuel) => fuel.recipe(),
            Self::CraftingResource(crafting_resource) => crafting_resource.recipe(),
            Self::BuildingMaterial(building_material) => building_material.recipe(),
            Self::ConsumableItem(consumable_item) => consumable_item.recipe(),
            Self::SimpleFood(simple_food) => simple_food.recipe(),
            Self::ComplexFood(complex_food) => complex_food.recipe(),
            Self::Clothing(clothing) => clothing.recipe(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Fuel {
    Oil,
    Coal,
    SeaMarrow,
    Wood,
}

impl Recipe for Fuel {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Oil => vec![vec![grain(), meat(), vegetables(), plant_fiber(), fish()]],
            Self::Coal => vec![vec![wood(), algae()]],
            Self::SeaMarrow => vec![],
            Self::Wood => vec![],
        }
    }
}

pub fn oil() -> Good {
    Good::Fuel(Fuel::Oil)
}
pub fn coal() -> Good {
    Good::Fuel(Fuel::Coal)
}
pub fn sea_marrow() -> Good {
    Good::Fuel(Fuel::SeaMarrow)
}
pub fn wood() -> Good {
    Good::Fuel(Fuel::Wood)
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CraftingResource {
    Pottery,
    Waterskins,
    Barrels,
    DrizzleWater,
    StormWater,
    ClearanceWater,
    Resin,
    Leather,
    Algae,
    PlantFiber,
    Scales,
    Reed,
    Herbs,
    Flour,
    Grain,
    Dye,
    CopperBars,
    CrystallizedDew,
    Stones,
    Clay,
    Salt,
    CopperOre,
}

impl Recipe for CraftingResource {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Pottery => vec![vec![clay()], vec![wood(), oil(), coal(), sea_marrow()]],
            Self::Waterskins => vec![vec![leather(), scales()], vec![oil(), meat(), salt()]],
            Self::Barrels => vec![vec![copper_bars(), crystallized_dew()], vec![planks()]],
            Self::Leather => vec![vec![algae(), reed(), grain(), vegetables()]],
            Self::Herbs => vec![vec![drizzle_water()]],
            Self::Flour => vec![vec![grain(), mushrooms(), roots(), algae()]],
            Self::Dye => vec![vec![insects(), berries(), copper_ore(), scales(), coal()]],
            Self::CopperBars => vec![
                vec![copper_ore(), scales()],
                vec![wood(), oil(), coal(), sea_marrow()],
            ],
            Self::CrystallizedDew => vec![
                vec![herbs(), insects(), resin(), vegetables(), algae()],
                vec![stone(), clay(), salt()],
                vec![storm_water(), drizzle_water(), clearance_water()],
            ],
            Self::Clay | Self::Reed | Self::Resin => vec![vec![clearance_water()]],
            Self::DrizzleWater
            | Self::StormWater
            | Self::ClearanceWater
            | Self::Algae
            | Self::PlantFiber
            | Self::Scales
            | Self::Grain
            | Self::Stones
            | Self::Salt
            | Self::CopperOre => vec![],
        }
    }
}

pub fn pottery() -> Good {
    Good::CraftingResource(CraftingResource::Pottery)
}

pub fn waterskins() -> Good {
    Good::CraftingResource(CraftingResource::Waterskins)
}

pub fn barrels() -> Good {
    Good::CraftingResource(CraftingResource::Barrels)
}

pub fn drizzle_water() -> Good {
    Good::CraftingResource(CraftingResource::DrizzleWater)
}

pub fn storm_water() -> Good {
    Good::CraftingResource(CraftingResource::StormWater)
}

pub fn clearance_water() -> Good {
    Good::CraftingResource(CraftingResource::ClearanceWater)
}

pub fn resin() -> Good {
    Good::CraftingResource(CraftingResource::Resin)
}

pub fn leather() -> Good {
    Good::CraftingResource(CraftingResource::Leather)
}

pub fn algae() -> Good {
    Good::CraftingResource(CraftingResource::Algae)
}

pub fn plant_fiber() -> Good {
    Good::CraftingResource(CraftingResource::PlantFiber)
}

pub fn scales() -> Good {
    Good::CraftingResource(CraftingResource::Scales)
}

pub fn reed() -> Good {
    Good::CraftingResource(CraftingResource::Reed)
}

pub fn herbs() -> Good {
    Good::CraftingResource(CraftingResource::Herbs)
}

pub fn flour() -> Good {
    Good::CraftingResource(CraftingResource::Flour)
}

pub fn grain() -> Good {
    Good::CraftingResource(CraftingResource::Grain)
}

pub fn dye() -> Good {
    Good::CraftingResource(CraftingResource::Dye)
}

pub fn copper_bars() -> Good {
    Good::CraftingResource(CraftingResource::CopperBars)
}

pub fn crystallized_dew() -> Good {
    Good::CraftingResource(CraftingResource::CrystallizedDew)
}

pub fn stone() -> Good {
    Good::CraftingResource(CraftingResource::Stones)
}

pub fn clay() -> Good {
    Good::CraftingResource(CraftingResource::Clay)
}

pub fn salt() -> Good {
    Good::CraftingResource(CraftingResource::Salt)
}

pub fn copper_ore() -> Good {
    Good::CraftingResource(CraftingResource::CopperOre)
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BuildingMaterial {
    Planks,
    Fabric,
    Bricks,
}

impl Recipe for BuildingMaterial {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Planks => vec![vec![wood()]],
            Self::Fabric => vec![vec![plant_fiber(), reed(), algae()]],
            Self::Bricks => vec![vec![clay(), stone()]],
        }
    }
}

pub fn planks() -> Good {
    Good::BuildingMaterial(BuildingMaterial::Planks)
}

pub fn fabric() -> Good {
    Good::BuildingMaterial(BuildingMaterial::Fabric)
}

pub fn bricks() -> Good {
    Good::BuildingMaterial(BuildingMaterial::Bricks)
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ConsumableItem {
    Scrolls,
    Incense,
    TrainingGear,
    Wine,
    Ale,
    Tea,
}

impl Recipe for ConsumableItem {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Scrolls => vec![vec![leather(), plant_fiber(), wood()], vec![dye(), wine()]],
            Self::Incense => vec![
                vec![herbs(), roots(), insects(), scales(), salt(), resin()],
                vec![wood(), oil(), coal(), sea_marrow()],
            ],
            Self::TrainingGear => vec![
                vec![stone(), copper_bars(), crystallized_dew()],
                vec![planks(), reed(), leather()],
            ],
            Self::Wine => vec![
                vec![berries(), mushrooms(), reed()],
                vec![pottery(), barrels(), waterskins()],
            ],
            Self::Ale => vec![
                vec![grain(), roots()],
                vec![pottery(), barrels(), waterskins()],
            ],
            Self::Tea => vec![
                vec![herbs(), dye(), resin(), mushrooms(), roots()],
                vec![pottery(), barrels(), waterskins()],
            ],
        }
    }
}

pub fn scrolls() -> Good {
    Good::ConsumableItem(ConsumableItem::Scrolls)
}

pub fn incense() -> Good {
    Good::ConsumableItem(ConsumableItem::Incense)
}

pub fn training_gear() -> Good {
    Good::ConsumableItem(ConsumableItem::TrainingGear)
}

pub fn wine() -> Good {
    Good::ConsumableItem(ConsumableItem::Wine)
}

pub fn ale() -> Good {
    Good::ConsumableItem(ConsumableItem::Ale)
}

pub fn tea() -> Good {
    Good::ConsumableItem(ConsumableItem::Tea)
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SimpleFood {
    Mushrooms,
    Roots,
    Vegetables,
    Fish,
    Meat,
    Eggs,
    Insects,
    Berries,
}

impl Recipe for SimpleFood {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Mushrooms => vec![vec![drizzle_water()]],
            Self::Roots | Self::Vegetables | Self::Fish | Self::Insects | Self::Berries => vec![],
            Self::Meat => vec![vec![plant_fiber(), reed(), algae(), grain(), vegetables()]],
            Self::Eggs => vec![vec![grain(), insects(), reed(), berries()]],
        }
    }
}

pub fn mushrooms() -> Good {
    Good::SimpleFood(SimpleFood::Mushrooms)
}

pub fn roots() -> Good {
    Good::SimpleFood(SimpleFood::Roots)
}

pub fn vegetables() -> Good {
    Good::SimpleFood(SimpleFood::Vegetables)
}

pub fn fish() -> Good {
    Good::SimpleFood(SimpleFood::Fish)
}

pub fn meat() -> Good {
    Good::SimpleFood(SimpleFood::Meat)
}

pub fn eggs() -> Good {
    Good::SimpleFood(SimpleFood::Eggs)
}

pub fn insects() -> Good {
    Good::SimpleFood(SimpleFood::Insects)
}

pub fn berries() -> Good {
    Good::SimpleFood(SimpleFood::Berries)
}

#[derive(Ord, PartialOrd, Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Clothing {
    Coats,
    Boots,
}

impl fmt::Display for Clothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

pub fn coats() -> Good {
    Good::Clothing(Clothing::Coats)
}

pub fn boots() -> Good {
    Good::Clothing(Clothing::Boots)
}

impl Recipe for Clothing {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Coats => vec![vec![fabric(), leather()], vec![dye(), resin()]],
            Self::Boots => vec![vec![leather(), scales()]],
        }
    }
}

#[derive(Ord, PartialOrd, Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ComplexFood {
    Porridge,
    Biscuits,
    Pie,
    PickledGoods,
    Jerky,
    Paste,
    Skewers,
}

impl fmt::Display for ComplexFood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

pub fn porridge() -> Good {
    Good::ComplexFood(ComplexFood::Porridge)
}

pub fn biscuits() -> Good {
    Good::ComplexFood(ComplexFood::Biscuits)
}

pub fn pie() -> Good {
    Good::ComplexFood(ComplexFood::Pie)
}

pub fn pickled_goods() -> Good {
    Good::ComplexFood(ComplexFood::PickledGoods)
}

pub fn jerky() -> Good {
    Good::ComplexFood(ComplexFood::Jerky)
}

pub fn paste() -> Good {
    Good::ComplexFood(ComplexFood::Paste)
}

pub fn skewers() -> Good {
    Good::ComplexFood(ComplexFood::Skewers)
}

impl Recipe for ComplexFood {
    fn recipe(&self) -> Vec<Vec<Good>> {
        match self {
            Self::Porridge => vec![
                vec![grain(), vegetables(), mushrooms(), herbs(), fish()],
                vec![clearance_water(), storm_water(), drizzle_water()],
            ],
            Self::Biscuits => vec![
                vec![flour()],
                vec![herbs(), berries(), roots(), eggs(), salt()],
            ],
            Self::Pie => vec![
                vec![flour()],
                vec![herbs(), meat(), insects(), berries(), fish()],
            ],
            Self::PickledGoods => vec![
                vec![vegetables(), mushrooms(), roots(), berries(), eggs()],
                vec![pottery(), barrels(), waterskins()],
            ],
            Self::Jerky => vec![
                vec![insects(), meat()],
                vec![salt(), wood(), sea_marrow(), coal(), oil()],
            ],
            Self::Paste => vec![vec![dye(), salt()], vec![eggs(), fish(), meat()]],
            Self::Skewers => vec![
                vec![insects(), meat(), mushrooms(), fish(), jerky()],
                vec![vegetables(), roots(), berries(), eggs()],
            ],
        }
    }
}
