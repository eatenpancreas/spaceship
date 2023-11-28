use spaceship::part::{CreatePartType, Part, PartType};
use spaceship::{Spaceship, SpaceshipBuilder};

fn main() {
    println!("Sara: {:?}", build_sara());
}

fn build_sara() -> Spaceship {
    let mut funds = 10.0;
    let mut sara = Spaceship::new("Sara".to_string());
    let mut builder = SpaceshipBuilder::new(&mut sara);

    println!("Base costs: {:?}", builder.costs());
    println!("Part costs: {:?}", builder.add_part_cost(CreatePartType::Cargo, 10, 1));
    builder.add_part(CreatePartType::Cargo, 10, 1);
    println!("{}", builder.size());
    println!("Part 2 costs: {:?}", builder.add_part_cost(CreatePartType::Cargo, 10, 1));
    builder.add_part(CreatePartType::Cargo, 10, 1);

    println!("Total costs: {:?}", builder.costs());
    println!("Status: {:?}", builder.complete(&mut funds));
    println!("Funds: {:?}", funds);
    
    sara
}