use bevy::{
    ecs::component::TableStorage,
    prelude::{Bundle, Component},
};
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use crate::components::{obstacle::Obstacle, path::Path};

// The sprite sheet is 49x18
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Ground = 0,
    GroundWithDirt = 1,
    GroundPathPartial = 2,
    GroundPathSmall = 3,
    GroundPathLarge = 4,
    GroundWithGrass = 5,
    GroundWithWeeds = 6,
    GroundWithLatticedGrass = 7,
    RoadVertical = 8,
    RoadRightBend = 9,
    RoadVerticalWithRightBend = 10,
    Road4Way = 11,
    RoadEnd = 12,
    Dunno1 = 13,
    Dunno2 = 14,
    Dunno3 = 15,
    SideWalk = 16,
    Dunno4 = 17,
    PlatformTopLeft = 18,
    PlatformTop = 19,
    PlatformTopRight = 20,
    LadderTop = 21,
    Spikes = 22,
    LockOutline = 23,
    HumanWithRobe = 24,
    Human = 25,
    HumanWithSpear = 26,
    HumanWithSwordAndShield = 27,
    HumanWithSwordAndShieldAndHelmet = 28,
    HumanWithSpearAndShieldAndHelmet = 29,
    HumanWithHeavyArmorAndHelmet = 30,
    HumanWithSwordAndShieldAndHornedHelmet = 31,
    HelmetWithCheekGuards1 = 32,
    HelmetWithCheekGuards2 = 33,
    HelmetWithCheekGuards3 = 34,
    HelmetWithCheekGuards4 = 35,
    HelmetWithCheekGuards5 = 36,
    HelmetRacing = 37,
    HelmetMining = 38,
    ShoesWhite = 39,
    ShoesWhiteMetal = 40,
    Gloves1 = 41,
    Gloves2 = 42,
    CarProfileLarge = 43,
    CarProfileSmall = 44,
    CarProfileConvertible = 45,
    CarPortraitLarge = 46,
    Dunno7 = 47,
    WheelChair = 48,
    // Row 2
    Tree = 49,
    TreePine = 50,
    TreeRound = 51,
    Trees = 52,
    LargeTree = 53,
    TreeWithLongTrunk = 54,
    Cactus = 55,
    Cactuses = 56,
    DirtRoadVertical = 57,
    DirtRoadRightBend = 58,
    DirtRoadVerticalWithRightBend = 59,
    DirtRoad4Way = 60,
    DirtRoadEnd = 61,
    LineDottedVertical = 62,
    // L
    LineDottedRightBend = 63,
    LineDottedHorizontalT = 64,
    LineDotted4Way = 65,
    LineDottedDoubleVertical = 66,
    PlatformCenterLeft = 67,
    PlatformCenter = 68,
    PlatformCenterRight = 69,
    LadderBottom = 70,
    ColumnTop = 71,
    LockFilled = 72,
    HumanWithRobeAndStaff = 73,
    HumanWithStrawHatAndPole = 74,
    FemaleHumanWithShowl = 75,
    HumanWithVestAndHair = 76,
    HumanWithDiagonalVest = 77,
    HumanWithVestAndBeard = 78,
    HumanWithHoodedCowl = 79,
    HumanWithHoodedCowlAndStaff = 80,
    ChestArmor = 81,
    ChestArmorWithShoulderPads = 82,
    ChestArmorWithShoulderPadsVersion2 = 83,
    ChestArmorWithShoulderPadsVersion3 = 84,
    ShirtBrown = 85,
    CloakBrown = 86,
    ShoesBrown = 87,
    ShoesBrownMetal = 88,
    GlovesBrown = 89,
    GlovesBrownMetal = 90,
    // CarProfileSmall = 91,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Ground
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileGroup {
    Ground,
    Road,
    Wall,
    // FIXME: Implement the rest of these.
    Other,
}

impl TileType {
    pub fn index(&self) -> TileTextureIndex {
        TileTextureIndex(*self as u32)
    }

    /// Returns true if the tile is a path and thus can be walked on.
    pub fn is_walkable(&self) -> bool {
        self.group() == TileGroup::Ground
    }

    pub fn group(&self) -> TileGroup {
        match self {
            TileType::Ground
            | TileType::GroundWithDirt
            | TileType::GroundPathPartial
            | TileType::GroundPathSmall
            | TileType::GroundPathLarge
            | TileType::GroundWithGrass
            | TileType::GroundWithWeeds
            | TileType::GroundWithLatticedGrass
            | TileType::RoadVertical
            | TileType::RoadRightBend
            | TileType::RoadVerticalWithRightBend
            | TileType::Road4Way
            | TileType::RoadEnd
            | TileType::SideWalk => TileGroup::Ground,
            _ => TileGroup::Other,
        }
    }
}
