/// A library for procedurally generating tilemaps
use std::collections::HashMap;

/// Where a given tile must be/not be
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum RelativeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

/// A requirement for a tile type
#[derive(Debug, PartialEq, Eq)]
pub struct Requirement {
    dir: RelativeDirection,
    tile: String,
}

/// A type of tile with certain restrictions placed on what it can be next to.
#[derive(Debug, PartialEq)]
pub struct TileType {
    name: String,
    must: Vec<Requirement>,
    must_not: Vec<Requirement>,
}

impl TileType {
    /// Create a new one with the given name
    fn new<S: Into<String>>(name: S) -> TileType {
        TileType {
            name: name.into(),
            must: Vec::new(),
            must_not: Vec::new(),
        }
    }
    /// Add a requirement that must be satisfied for this tile type
    pub fn add_must<S: Into<String>>(&mut self, dir: RelativeDirection, tile: S) -> &mut Self {
        self.must.push(Requirement {
            dir: dir,
            tile: tile.into(),
        });
        self
    }
    /// Add a requirement that must not be true for this tile type
    pub fn add_must_not<S: Into<String>>(&mut self, dir: RelativeDirection, tile: S) -> &mut Self {
        self.must_not.push(Requirement {
            dir: dir,
            tile: tile.into(),
        });
        self
    }
    // Convenience function, obvious.
    pub fn above<S: Into<String>>(&mut self, tile: S) -> &mut Self {
        self.add_must(RelativeDirection::DOWN, tile)
    }
    // Convenience function, obvious.
    pub fn below<S: Into<String>>(&mut self, tile: S) -> &mut Self {
        self.add_must(RelativeDirection::UP, tile)
    }
    // Convenience function, obvious.
    pub fn left<S: Into<String>>(&mut self, tile: S) -> &mut Self {
        self.add_must(RelativeDirection::RIGHT, tile)
    }
    // Convenience function, obvious.
    pub fn right<S: Into<String>>(&mut self, tile: S) -> &mut Self {
        self.add_must(RelativeDirection::LEFT, tile)
    }
    // Convenience function, obvious.
    pub fn not_above<S: Into<String>>(&mut self, tile: S) -> &mut Self {
        self.add_must_not(RelativeDirection::DOWN, tile)
    }
    // Convenience function, obvious.
    pub fn not_below<S: Into<String>>(&mut self, tile: S) -> &mut Self {
        self.add_must_not(RelativeDirection::UP, tile)
    }
    // Convenience function, obvious.
    pub fn not_left<S: Into<String>>(&mut self, tile: S) -> &mut Self {
        self.add_must_not(RelativeDirection::RIGHT, tile)
    }
    // Convenience function, obvious.
    pub fn not_right<S: Into<String>>(&mut self, tile: S) -> &mut Self {
        self.add_must_not(RelativeDirection::LEFT, tile)
    }
}

/// Used to generate maps
pub struct TileSystem {
    tiles: HashMap<String, TileType>,
}

impl TileSystem {
    /// Makes a new system
    pub fn new() -> TileSystem {
        TileSystem {
            tiles: HashMap::new(),
        }
    }
    /// Make & Add a tile, returning it if the name is taken or a reference to it if it isn't
    pub fn add_tile<S: Into<String>>(&mut self, name: S) -> Result<&mut TileType, TileType> {
        let name = name.into();
        let res = self.tiles.insert(name.clone(), TileType::new(name.clone()));
        if let Some(err) = res {
            return Err(err);
        }
        Ok(self.tiles.get_mut(&name).unwrap())
    }
    /// Get a reference to the tile with the given name
    pub fn borrow_tile<S: Into<String>>(&self, name: S) -> Option<&TileType> {
        self.tiles.get(&name.into())
    }
    /// Try to generate a map, retrying as many tries as given
    pub fn gen_retry(&self, _w: u32, _h: u32, _tries: u32) -> Result<Vec<Vec<&TileType>>, ()> {
        panic!("Not implemented")
    }
}

#[cfg(test)]
mod test;
