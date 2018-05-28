use super::{Map, TileSystem};
use std::collections::HashMap;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key as u32, $value);
            )+
            m
        }
     };
);

/// Utility function; Compares a generated map with a map-like structure which specifies only names.
fn compare_maps(actual: Map, expected: HashMap<u32, HashMap<u32, &str>>) -> bool {
    for (y, v) in actual.iter() {
        for (x, t) in v.iter() {
            if t.name != *expected.get(y).unwrap().get(x).unwrap() {
                println!(
                    "expected ({},{}): {}  - actual: {}",
                    x,
                    y,
                    t.name,
                    *expected.get(y).unwrap().get(x).unwrap()
                );
                return false;
            }
        }
    }
    true
}

#[test]
fn adjacent_vertical() {
    let mut system = TileSystem::new();
    {
        system.add_tile("ground").unwrap();
    }
    {
        system
            .add_tile("floor")
            .unwrap()
            .above("ground")
            .below("roof");
    }
    {
        system.add_tile("roof").unwrap().above("floor");
    }

    // must be: roof    /\
    //          ground  ||
    //          floor   --

    // 1 wide, 3 tall map
    let map = system.gen_retry(1, 3, 10).unwrap();

    let expected = map!{
        0 => map!{0 => "floor"},
        1 => map!{0 => "ground"},
        2 => map!{0 => "roof"}
    };

    assert_eq!(compare_maps(map, expected), true);
}

#[test]
fn not_adjacent_vertical() {
    let mut system = TileSystem::new();
    {
        system.add_tile("red").unwrap().not_below("blue");
    }
    {
        system.add_tile("green").unwrap().not_below("red");
    }
    {
        system.add_tile("blue").unwrap().not_above("red");
    }

    // must be: blue
    //          green
    //          red

    // 1 wide, 3 tall map
    let map = system.gen_retry(1, 3, 10).unwrap();

    let expected = map!{
        0 => map!{0 => "red"},
        1 => map!{0 => "green"},
        2 => map!{0 => "blue"}
    };

    assert_eq!(compare_maps(map, expected), true);
}

#[test]
fn adjacent_horizontal() {
    let mut system = TileSystem::new();
    {
        system.add_tile("left").unwrap().left("middle");
    }
    {
        system
            .add_tile("middle")
            .unwrap()
            .right("left")
            .left("right");
    }
    {
        system.add_tile("right").unwrap().right("middle");
    }

    // must be: left middle right

    // 3 wide, 1 tall
    let map = system.gen_retry(3, 1, 10).unwrap();

    let expected = map!{
        0 => map!{0 => "left"},
        0 => map!{1 => "middle"},
        0 => map!{2 => "right"}
    };
    assert_eq!(compare_maps(map, expected), true);
}

#[test]
fn not_adjacent_horizontal() {
    let mut system = TileSystem::new();
    {
        system
            .add_tile("red")
            .unwrap()
            .not_right("green")
            .not_right("blue");
    }
    {
        system.add_tile("green").unwrap().not_right("blue");
    }
    {
        system
            .add_tile("blue")
            .unwrap()
            .not_left("red")
            .not_left("green");
    }

    // must be: red green blue

    // 3 wide, 1 tall map
    let map = system.gen_retry(3, 1, 10).unwrap();

    let expected = map!{
            0 => map!{0 => "red"},
            0 => map!{1 => "green"},
            0 => map!{2 => "blue"}
    };
    assert_eq!(compare_maps(map, expected), true);
}
