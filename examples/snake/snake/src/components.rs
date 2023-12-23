use toxoid_api_macro::component;
use toxoid_api::IsComponent;
use toxoid_api::bindings::*;
use toxoid_api::*;

pub enum DirectionEnum {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

component! {
    Direction {
        direction: u8
    },
    Player {
        player: bool
    },
    Food {
        entity: u64
    },
    Head {
        head: bool
    },
    Tail {
        tail: bool
    },
    Stats {
        score: u32,
        high_score: u32,
        tail_length: u32
    }
}

pub fn init() {    
    // Snake Specific
    Direction::register();
    Player::register();
    Food::register();
    Head::register();
    Tail::register();
    Stats::register();

    // // Create a new entity.
    // let mut player = Entity::new();

    // player.add::<Position>();

    // let mut pos_component = player.get::<Position>();
    // pos_component.set_x(420);
    // pos_component.set_y(421);

    // let x = pos_component.get_x();
    // let y = pos_component.get_y();

    // // Print the component data.
    // print_i32(x as i32);
    // print_i32(y as i32);

    // let mut query = Query::new::<(Position,)>();
    // let iter = query.iter();
    // while iter.next() {
    //     let pos = iter.field::<Position>();
    //     pos.iter().for_each(|pos| {
    //         print_string("Position X Value:");
    //         print_i32(pos.get_x() as i32);
    //         print_string("Position Y Value:");
    //         print_i32(pos.get_y() as i32);
    //     });
    // }
}