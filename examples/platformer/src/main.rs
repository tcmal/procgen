extern crate procgen;

use procgen::{TileSystem, TileType};

fn to_render(tile: &TileType) -> &str {
    match tile.name.as_str() {
        "ground" => "-",
        "air" => " ",
        "block" => "■",
        "special" => "×",
        _ => "?",
    }
}

fn main() {
    // register all our tiles
    let mut system = TileSystem::new();
    // the ground - always going to be one solid layer at the bottom
    {
        system
            .add_tile("ground")
            .unwrap()
            .not_above("ground")
            .not_above("air")
            .not_above("block");
    }
    {
        // air - most common
        system.add_tile("air").unwrap();
    }
    {
        // basic block - somewhere up in the air, solid platform going on for however long.
        system.add_tile("block").unwrap().above("air");
    }
    {
        // special block - inbetween two regular blocks.
        system.add_tile("special").unwrap().left("block").right("block");
    }

    // generate the world
    let map = system.try_gen(10, 10).unwrap();

    let mut y: u32 = 9;
    // render it to console
    while let Some(x_map) = map.get(&y) {
        let mut x: u32 = 0;
        while let Some(t) = x_map.get(&x) {
            print!("{}", to_render(t));
            x += 1
        }
        print!("\n");
        if y < 1 {
            break;
        }
        y -= 1;
    }
}
