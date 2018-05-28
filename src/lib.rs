/// A library for procedurally generating tilemaps
use std::collections::HashMap;

type Map<'a> = HashMap<u32, HashMap<u32, &'a TileType>>;

fn trans_is_in_bounds(mut pos: CoOrd, dir: RelativeDirection, w: u32, h: u32) -> bool {
    use self::RelativeDirection::*;
    match dir {
        UP => pos.y += 1,
        RIGHT => pos.x += 1,
        DOWN => {
            if pos.y < 1 {
                return false;
            }
            pos.y -= 1
        }
        LEFT => {
            if pos.x < 1 {
                return false;
            }
            pos.x -= 1
        }
    };
    pos.y < h && pos.x < w
}

fn get_tile<'a>(map: &Map<'a>, coord: CoOrd) -> Option<&'a TileType> {
    if map.contains_key(&coord.y) && map.get(&coord.y).unwrap().contains_key(&coord.x) {
        return Some(map.get(&coord.y).unwrap().get(&coord.x).unwrap());
    }
    None
}

fn put_tile<'a>(map: &mut Map<'a>, tile: &'a TileType, coord: CoOrd) {
    if !map.contains_key(&coord.y) {
        map.insert(coord.y, HashMap::new());
    }
    map.get_mut(&coord.y).unwrap().insert(coord.x, tile);
}
fn remove_tile(map: &mut Map, coord: CoOrd) {
    if let Some(col) = map.get_mut(&coord.y) {
        col.remove(&coord.x);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoOrd {
    x: u32,
    y: u32,
}

/// Where a given tile must be/not be
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RelativeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl RelativeDirection {
    pub fn flip(&self) -> RelativeDirection {
        use self::RelativeDirection::*;
        match self {
            UP => DOWN,
            DOWN => UP,
            LEFT => RIGHT,
            RIGHT => LEFT,
        }
    }
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
    pub name: String,
    pub must: Vec<Requirement>,
    pub must_not: Vec<Requirement>,
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
    pub fn gen_retry(&self, w: u32, h: u32, tries: u32) -> Option<Map> {
        for _ in 0..tries {
            if let Some(map) = self.try_gen(w, h) {
                return Some(map);
            }
        }
        None
    }
    pub fn try_gen(&self, w: u32, h: u32) -> Option<Map> {
        // start at the bottom left
        let start = CoOrd { x: 0, y: 0 };

        // generate squares around that point.
        let mut map: Map = HashMap::new();

        if self.gen_adjacent_recursive(w, h, &mut map, start, start) {
            return Some(map);
        }
        None
    }

    fn gen_adjacent_recursive<'a>(
        &'a self,
        w: u32,
        h: u32,
        map: &mut Map<'a>,
        start: CoOrd,
        prev: CoOrd,
    ) -> bool {
        if start.x >= w || start.y >= h {
            return true;
        }
        if let Some(_) = get_tile(map, start) {
            return true;
        }

        // start with all tiles as possibilities.
        let mut possibilities: Vec<&TileType> = self.tiles.values().collect();

        // populate the adjacent tiles we have to rule on.
        let mut adjacent_coords: Vec<(CoOrd, RelativeDirection)> = Vec::new();

        if start.x > 0 {
            adjacent_coords.push((
                CoOrd {
                    x: start.x - 1,
                    y: start.y,
                },
                RelativeDirection::LEFT,
            ));
        }
        if start.y > 0 {
            adjacent_coords.push((
                CoOrd {
                    x: start.x,
                    y: start.y - 1,
                },
                RelativeDirection::DOWN,
            ));
        }
        adjacent_coords.push((
            CoOrd {
                x: start.x + 1,
                y: start.y,
            },
            RelativeDirection::RIGHT,
        ));
        adjacent_coords.push((
            CoOrd {
                x: start.x,
                y: start.y + 1,
            },
            RelativeDirection::UP,
        ));

        let mut adjacent_tiles: HashMap<RelativeDirection, &TileType> = HashMap::new();
        for (coord, dir) in &adjacent_coords {
            if let Some(tile) = get_tile(&map, *coord) {
                adjacent_tiles.insert(*dir, tile);
            }
        }

        let mut current_is_required = false;

        // if any adjacent tiles specify what must be here; that's what we need
        for (dir, tile) in &adjacent_tiles {
            let applying_reqs = tile.must.iter().filter(|r| r.dir == dir.flip());
            if applying_reqs.clone().count() > 0 {
                // this is targeted so must be a certain thing
                // unless two require different things, in which case we can't solve it.
                let req = applying_reqs.last().unwrap();
                if current_is_required && req.tile != possibilities[0].name {
                    return false;
                }
                println!("required: {:?}", req);
                current_is_required = true;
                possibilities = vec![self.borrow_tile(req.tile.clone()).unwrap()];
            }
        }
        if !current_is_required {
            // we still don't have only one option; so narrow it down more.
            possibilities.retain(|possibility| {
                // if something that this tile needs to be there isn't, remove it.
                for req in &possibility.must {
                    if (adjacent_tiles.contains_key(&req.dir)
                        && adjacent_tiles.get(&req.dir).unwrap().name != req.tile)
                        || !trans_is_in_bounds(start, req.dir, w, h)
                    {
                        return false; // a requirement isn't satisfied; not possible.
                    }
                }

                // if something that mustn't be there is there, remove it.
                for req in &possibility.must_not {
                    if adjacent_tiles.contains_key(&req.dir)
                        && adjacent_tiles.get(&req.dir).unwrap().name == req.tile
                    {
                        return false; // a requirement isn't satisfied; not possible.
                    }
                }

                // if another tile type says we mustn't be this type, remove it.
                for (dir, tile) in &adjacent_tiles {
                    for req in &tile.must_not {
                        if req.dir == dir.flip() && req.tile == possibility.name {
                            return false;
                        }
                    }
                }
                true
            });
        }
        // if we have some options left, loop through them all
        for tile in possibilities {
            put_tile(map, tile, start);
            // and for each one generate each adjacent tile, unless we reach one that fails; in which case skip.
            let mut any_failed = false;
            for (pos, _) in &adjacent_coords {
                if *pos == prev {
                    continue;
                }
                if !self.gen_adjacent_recursive(w, h, map, pos.clone(), start) {
                    any_failed = true;
                    break;
                }
            }
            // if each one passes, this is a valid option & we're done.
            if !any_failed {
                return true;
            }
        }
        // if none pass, we couldn't find a valid option.
        // remove whatever tile we placed, leaving the map untouched.
        remove_tile(map, start);
        false
    }
}

#[cfg(test)]
mod test;
