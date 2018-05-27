use super::TileSystem;

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

    // returns Vec<Vec<&TileType>> where the outer is Y
    assert_eq!(map[0][0].name, "floor");
    assert_eq!(map[1][0].name, "ground");
    assert_eq!(map[2][0].name, "roof");
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

    // returns Vec<Vec<&TileType>> where the outer is Y

    assert_eq!(map[0][0].name, "red");
    assert_eq!(map[1][0].name, "green");
    assert_eq!(map[2][0].name, "blue");
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

    // returns Vec<Vec<&TileType>> where the outer is Y

    assert_eq!(map[0][0].name, "left");
    assert_eq!(map[0][1].name, "middle");
    assert_eq!(map[0][2].name, "right");
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

    // returns Vec<Vec<&TileType>> where the outer is Y

    assert_eq!(map[0][0].name, "red");
    assert_eq!(map[0][1].name, "green");
    assert_eq!(map[0][2].name, "blue");
}
