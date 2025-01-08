use serde::{Deserialize, Serialize};

use std::fmt::{self, Display};

use crate::{titleize, Recipe};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Resource {
    Fuel(Fuel),
    CraftingResource(CraftingResource),
    BuildingMaterial(BuildingMaterial),
    ConsumableItem(ConsumableItem),
    SimpleFood(SimpleFood),
    ComplexFood(ComplexFood),
    Clothing(Clothing),
}

impl Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fuel(fuel) => fuel.to_string(),
                Self::CraftingResource(crafting_resource) => crafting_resource.to_string(),
                Self::BuildingMaterial(building_material) => building_material.to_string(),
                Self::ConsumableItem(consumable_item) => consumable_item.to_string(),
                Self::SimpleFood(simple_food) => simple_food.to_string(),
                Self::ComplexFood(complex_food) => complex_food.to_string(),
                Self::Clothing(clothing) => clothing.to_string(),
            }
        )
    }
}

impl Recipe for Resource {
    fn recipe(&self) -> Vec<Vec<Resource>> {
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

pub fn all_goods() -> Vec<Resource> {
    [
        all_fuel(),
        all_crafting_resources(),
        all_building_materials(),
        all_clothing(),
        all_simple_food(),
        all_complex_food(),
        all_consumable_items(),
    ]
    .concat()
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Fuel {
    Oil,
    Coal,
    SeaMarrow,
    Wood,
}

impl Display for Fuel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

impl Recipe for Fuel {
    fn recipe(&self) -> Vec<Vec<Resource>> {
        match self {
            Self::Oil => vec![vec![grain(), meat(), vegetables(), plant_fiber(), fish()]],
            Self::Coal => vec![vec![wood(), algae()]],
            Self::SeaMarrow => vec![],
            Self::Wood => vec![],
        }
    }
}

pub fn oil() -> Resource {
    Resource::Fuel(Fuel::Oil)
}
pub fn coal() -> Resource {
    Resource::Fuel(Fuel::Coal)
}
pub fn sea_marrow() -> Resource {
    Resource::Fuel(Fuel::SeaMarrow)
}
pub fn wood() -> Resource {
    Resource::Fuel(Fuel::Wood)
}

pub fn all_fuel() -> Vec<Resource> {
    vec![oil(), coal(), sea_marrow(), wood()]
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

impl Display for CraftingResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

impl Recipe for CraftingResource {
    fn recipe(&self) -> Vec<Vec<Resource>> {
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

pub fn pottery() -> Resource {
    Resource::CraftingResource(CraftingResource::Pottery)
}

pub fn waterskins() -> Resource {
    Resource::CraftingResource(CraftingResource::Waterskins)
}

pub fn barrels() -> Resource {
    Resource::CraftingResource(CraftingResource::Barrels)
}

pub fn drizzle_water() -> Resource {
    Resource::CraftingResource(CraftingResource::DrizzleWater)
}

pub fn storm_water() -> Resource {
    Resource::CraftingResource(CraftingResource::StormWater)
}

pub fn clearance_water() -> Resource {
    Resource::CraftingResource(CraftingResource::ClearanceWater)
}

pub fn resin() -> Resource {
    Resource::CraftingResource(CraftingResource::Resin)
}

pub fn leather() -> Resource {
    Resource::CraftingResource(CraftingResource::Leather)
}

pub fn algae() -> Resource {
    Resource::CraftingResource(CraftingResource::Algae)
}

pub fn plant_fiber() -> Resource {
    Resource::CraftingResource(CraftingResource::PlantFiber)
}

pub fn scales() -> Resource {
    Resource::CraftingResource(CraftingResource::Scales)
}

pub fn reed() -> Resource {
    Resource::CraftingResource(CraftingResource::Reed)
}

pub fn herbs() -> Resource {
    Resource::CraftingResource(CraftingResource::Herbs)
}

pub fn flour() -> Resource {
    Resource::CraftingResource(CraftingResource::Flour)
}

pub fn grain() -> Resource {
    Resource::CraftingResource(CraftingResource::Grain)
}

pub fn dye() -> Resource {
    Resource::CraftingResource(CraftingResource::Dye)
}

pub fn copper_bars() -> Resource {
    Resource::CraftingResource(CraftingResource::CopperBars)
}

pub fn crystallized_dew() -> Resource {
    Resource::CraftingResource(CraftingResource::CrystallizedDew)
}

pub fn stone() -> Resource {
    Resource::CraftingResource(CraftingResource::Stones)
}

pub fn clay() -> Resource {
    Resource::CraftingResource(CraftingResource::Clay)
}

pub fn salt() -> Resource {
    Resource::CraftingResource(CraftingResource::Salt)
}

pub fn copper_ore() -> Resource {
    Resource::CraftingResource(CraftingResource::CopperOre)
}

pub fn all_crafting_resources() -> Vec<Resource> {
    vec![
        pottery(),
        waterskins(),
        barrels(),
        drizzle_water(),
        storm_water(),
        clearance_water(),
        resin(),
        leather(),
        algae(),
        plant_fiber(),
        scales(),
        reed(),
        herbs(),
        flour(),
        grain(),
        dye(),
        copper_bars(),
        crystallized_dew(),
        stone(),
        clay(),
        salt(),
        copper_ore(),
    ]
}

#[derive(Ord, PartialOrd, Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BuildingMaterial {
    Planks,
    Fabric,
    Bricks,
}

impl Display for BuildingMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

impl Recipe for BuildingMaterial {
    fn recipe(&self) -> Vec<Vec<Resource>> {
        match self {
            Self::Planks => vec![vec![wood()]],
            Self::Fabric => vec![vec![plant_fiber(), reed(), algae()]],
            Self::Bricks => vec![vec![clay(), stone()]],
        }
    }
}

pub fn planks() -> Resource {
    Resource::BuildingMaterial(BuildingMaterial::Planks)
}

pub fn fabric() -> Resource {
    Resource::BuildingMaterial(BuildingMaterial::Fabric)
}

pub fn bricks() -> Resource {
    Resource::BuildingMaterial(BuildingMaterial::Bricks)
}

pub fn all_building_materials() -> Vec<Resource> {
    vec![planks(), fabric(), bricks()]
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

impl Display for ConsumableItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

impl Recipe for ConsumableItem {
    fn recipe(&self) -> Vec<Vec<Resource>> {
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

pub fn scrolls() -> Resource {
    Resource::ConsumableItem(ConsumableItem::Scrolls)
}

pub fn incense() -> Resource {
    Resource::ConsumableItem(ConsumableItem::Incense)
}

pub fn training_gear() -> Resource {
    Resource::ConsumableItem(ConsumableItem::TrainingGear)
}

pub fn wine() -> Resource {
    Resource::ConsumableItem(ConsumableItem::Wine)
}

pub fn ale() -> Resource {
    Resource::ConsumableItem(ConsumableItem::Ale)
}

pub fn tea() -> Resource {
    Resource::ConsumableItem(ConsumableItem::Tea)
}

pub fn all_consumable_items() -> Vec<Resource> {
    vec![scrolls(), incense(), training_gear(), wine(), ale(), tea()]
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

impl Display for SimpleFood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

impl Recipe for SimpleFood {
    fn recipe(&self) -> Vec<Vec<Resource>> {
        match self {
            Self::Mushrooms => vec![vec![drizzle_water()]],
            Self::Roots | Self::Vegetables | Self::Fish | Self::Insects | Self::Berries => vec![],
            Self::Meat => vec![vec![plant_fiber(), reed(), algae(), grain(), vegetables()]],
            Self::Eggs => vec![vec![grain(), insects(), reed(), berries()]],
        }
    }
}

pub fn mushrooms() -> Resource {
    Resource::SimpleFood(SimpleFood::Mushrooms)
}

pub fn roots() -> Resource {
    Resource::SimpleFood(SimpleFood::Roots)
}

pub fn vegetables() -> Resource {
    Resource::SimpleFood(SimpleFood::Vegetables)
}

pub fn fish() -> Resource {
    Resource::SimpleFood(SimpleFood::Fish)
}

pub fn meat() -> Resource {
    Resource::SimpleFood(SimpleFood::Meat)
}

pub fn eggs() -> Resource {
    Resource::SimpleFood(SimpleFood::Eggs)
}

pub fn insects() -> Resource {
    Resource::SimpleFood(SimpleFood::Insects)
}

pub fn berries() -> Resource {
    Resource::SimpleFood(SimpleFood::Berries)
}

pub fn all_simple_food() -> Vec<Resource> {
    vec![
        mushrooms(),
        roots(),
        vegetables(),
        fish(),
        meat(),
        eggs(),
        insects(),
        berries(),
    ]
}

#[derive(Ord, PartialOrd, Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Clothing {
    Coats,
    Boots,
}

impl Display for Clothing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

pub fn coats() -> Resource {
    Resource::Clothing(Clothing::Coats)
}

pub fn boots() -> Resource {
    Resource::Clothing(Clothing::Boots)
}

pub fn all_clothing() -> Vec<Resource> {
    vec![coats(), boots()]
}

impl Recipe for Clothing {
    fn recipe(&self) -> Vec<Vec<Resource>> {
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

impl Display for ComplexFood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", titleize(self))
    }
}

pub fn porridge() -> Resource {
    Resource::ComplexFood(ComplexFood::Porridge)
}

pub fn biscuits() -> Resource {
    Resource::ComplexFood(ComplexFood::Biscuits)
}

pub fn pie() -> Resource {
    Resource::ComplexFood(ComplexFood::Pie)
}

pub fn pickled_goods() -> Resource {
    Resource::ComplexFood(ComplexFood::PickledGoods)
}

pub fn jerky() -> Resource {
    Resource::ComplexFood(ComplexFood::Jerky)
}

pub fn paste() -> Resource {
    Resource::ComplexFood(ComplexFood::Paste)
}

pub fn skewers() -> Resource {
    Resource::ComplexFood(ComplexFood::Skewers)
}

pub fn all_complex_food() -> Vec<Resource> {
    vec![
        porridge(),
        biscuits(),
        pie(),
        pickled_goods(),
        jerky(),
        paste(),
        skewers(),
    ]
}

impl Recipe for ComplexFood {
    fn recipe(&self) -> Vec<Vec<Resource>> {
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
