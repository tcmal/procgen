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

// todo: duplication

fn print_hash(prefix: &str, map: HashMap<u32, HashMap<u32, &str>>) {
    println!("{}", prefix);
    for (y, v) in map {
        for (x, t) in v {
            println!("({}, {}) - {}", x, y, t);
        }
    }
}

fn print_map(prefix: &str, map: Map) {
    println!("{}", prefix);
    for (y, v) in map {
        for (x, t) in v {
            println!("({}, {}) - {}", x, y, t.name);
        }
    }
}

/// Utility function; Compares a generated map with a map-like structure which specifies only names.
fn compare_maps(actual: Map, expected: HashMap<u32, HashMap<u32, &str>>) -> bool {
    print_hash("expected: ", expected.clone());
    print_map("actual: ", actual.clone());
    for (y, v) in actual.iter() {
        for (x, t) in v.iter() {
            if t.name != *expected.get(y).unwrap().get(x).unwrap() {
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
        system.add_tile("ground").unwrap().below("floor");
    }
    {
        system
            .add_tile("floor")
            .unwrap()
            .below("roof")
            .above("ground");
    }
    {
        system.add_tile("roof").unwrap().above("floor");
    }

    // must be: roof    /\
    //          floor   ||
    //          ground  --

    // 1 wide, 3 tall map
    let map = system.gen_retry(1, 3, 10).unwrap();

    let expected = map!{
        2 => map!{0 => "roof"},
        1 => map!{0 => "floor"},
        0 => map!{0 => "ground"}
    };

    assert_eq!(compare_maps(map, expected), true);
}

#[test]
fn not_adjacent_vertical() {
    let mut system = TileSystem::new();
    {
        system
            .add_tile("red")
            .unwrap()
            .not_above("red")
            .not_above("green");
    }
    {
        system
            .add_tile("green")
            .unwrap()
            .not_above("blue")
            .not_above("green");
    }
    {
        system
            .add_tile("blue")
            .unwrap()
            .not_above("blue")
            .not_above("red")
            .not_below("red");
    }

    // must be: blue
    //          green
    //          red

    // 1 wide, 3 tall map
    let map = system.gen_retry(1, 3, 99999).unwrap();

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
        0 => map!{0 => "left",
                1 => "middle",
                2 => "right"}
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
            .not_right("red")
            .not_right("green")
            .not_right("blue");
    }
    {
        system
            .add_tile("green")
            .unwrap()
            .not_right("blue")
            .not_right("green");
    }
    {
        system
            .add_tile("blue")
            .unwrap()
            .not_left("red")
            .not_left("green")
            .not_right("blue");
    }

    // must be: red green blue

    // 3 wide, 1 tall map
    let map = system.gen_retry(3, 1, 10).unwrap();

    let expected = map!{
            0 => map!{0 => "red",
                    1 => "green",
                    2 => "blue"}
    };
    assert_eq!(compare_maps(map, expected), true);
}
