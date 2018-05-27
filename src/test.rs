#[test]
fn adjacent_vertical() {
    let system = TileSystem::new();

    let ground_tile = system.add_tile("ground");

    let floor_tile = system.add_tile("floor").above("ground");

    let roof_tile = system.add_tile("roof").above("floor");

    floor_tile.below("roof");

    // must be: roof    /\
    //          ground  ||
    //          floor   --

    // 1 wide, 3 tall map
    let map = system.gen_retry(1, 3, 10).unwrap();

    // returns Vec<Vec<&TileType>> where the outer is Y

    assert_eq!(map, vec![[ground_tile], [floor_tile], [roof_tile]]);
}

#[test]
fn not_adjacent_vertical() {
    let system = TileSystem::new();

    let red_tile = system.add_tile("red");

    let green_tile = system.add_tile("green");

    let blue_tile = system.add_tile("blue");

    red_tile.not_below("blue");
    blue_tile.not_above("red");
    green_tile.not_below("red");

    // must be: blue
    //          green
    //          red

    // 1 wide, 3 tall map
    let map = system.gen_retry(1, 3, 10).unwrap();

    // returns Vec<Vec<&TileType>> where the outer is Y

    assert_eq!(map, vec![[red_tile], [green_tile], [blue_tile]]);
}
fn adjacent_horizontal() {
    let system = TileSystem::new();

    let left_tile = system.add_tile("left");

    let middle_tile = system.add_tile("middle").right("left");

    let right_tile = system.add_tile("right").right("middle");

    left_tile.left("middle");
    middle_tile.left("right");

    // must be: left middle right

    // 3 wide, 1 tall
    let map = system.gen_retry(3, 1, 10).unwrap();

    // returns Vec<Vec<&TileType>> where the outer is Y

    assert_eq!(map, vec![[left_tile, middle_tile, right_tile]]);
}

#[test]
fn not_adjacent_horizontal() {
    let system = TileSystem::new();

    let red_tile = system.add_tile("red");

    let green_tile = system.add_tile("green");

    let blue_tile = system.add_tile("blue");

    red_tile.not_right("green");
    red_tile.not_right("blue");
    blue_tile.not_left("red");
    blue_tile.not_left("green");
    green_tile.not_right("blue");

    // must be: red green blue

    // 3 wide, 1 tall map
    let map = system.gen_retry(3, 1, 10).unwrap();

    // returns Vec<Vec<&TileType>> where the outer is Y

    assert_eq!(map, vec![[red_tile, green_tile, blue_tile]]);
}
