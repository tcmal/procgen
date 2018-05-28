pub mod lib;
use lib::TileSystem;
fn main() {
    let mut system = TileSystem::new();
    {
        system.add_tile("ground").unwrap()
            .not_above("ground")
            .below("floor");
    }
    {
        system
            .add_tile("floor")
            .unwrap()
            .above("ground")
            .not_below("floor")
            .below("roof");
    }
    {
        system.add_tile("roof").unwrap()
            .above("floor")
            .not_below("roof");
    }

    // must be: roof    /\
    //          ground  ||
    //          floor   --

    // 1 wide, 3 tall map
    let map = system.gen_retry(1, 3, 10).unwrap();
    for y in map.values() {
        for x in y.values() {
            println!("{:?}", x.name);
        }
    }
    // returns Vec<Vec<&TileType>> where the outer is Y
}
