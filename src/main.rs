// const_generics are unstable and need to be enabled with nightly and this feature macro
#![allow(incomplete_features)]
#![feature(const_generics)]

use std::fmt;
use std::ops;

macro_rules! make_soa {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }, $vec_name:ident, $arr_name:ident) => {
        struct $name {
            $($field_name: $field_type,)*
        }

        /*
        mashup! {
            m["SOA_vec_name"] = $name _SOA_vec;
            m["SOA_name"] = $name _SOA;
        }
        */

        struct $vec_name {
            $($field_name: Vec<$field_type>,)*
        }

        impl $vec_name {

            fn push(&mut self, item:$name) {
                $(self.$field_name.push(item.$field_name);)*
            }

            /**
             * constructs a
             * you CANNOT get a mutable reference this way
             * TODO: find a way to make the innards of the returned struct mutable
            */
            fn get(&self, index:usize) -> Option<$name>{
                return Option::from($name {
                    $($field_name :
                        match self.$field_name.get(index) {
                            Some (v) => v.clone(),
                            None => return None
                        },
                    )*
                });
                /*$(let $field_name =
                    match self.$field_name.get(index) {
                        Some (v) => v,
                        None => return None
                    };
                )**/
            }

            fn put(&mut self, item:$name, index:usize) {
                $(
                    match self.$field_name.get_mut(index) {
                        Some (v) => {*v = item.$field_name;},
                        _ => return
                    }
                )*
            }
        }

        struct $arr_name<const LEN:usize> {
            $($field_name: [$field_type; LEN],)*
        }
    }
}

make_soa! {
    struct Vector3 {
        x: f64,
        y: f64,
        z: f64,
    }, Vector3_SOA_vec, Vector3_SOA
}

fn v3_example() -> Option<Vector3> {
    Option::from(Vector3 {
        x: {
            match true {
                true => 1.0,
                false => 2.0,
            }
        },
        y: {
            match true {
                true => 1.0,
                false => 2.0,
            }
        },
        z: {
            match true {
                true => 1.0, // return None,
                false => 2.0,
            }
        },
    })
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, vec_to_add: Vector3) -> Vector3 {
        Vector3 {
            x: (self.x + vec_to_add.x),
            y: (self.y + vec_to_add.y),
            z: (self.z + vec_to_add.z),
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({x}, {y}, {z})", x = self.x, y = self.y, z = self.z)
    }
}

struct PhysicsObject {
    position: Vector3_SOA<4>,
    velocity: Vector3_SOA<4>,
}
make_soa! {
    struct Player {
        player_id: usize,
    }, Player_SOA_vec, Player_SOA
}

enum WaitUntilState<OutputState> {
    Completed,
    Pending,
}

enum ConditionFunction<StateToCheck> {
    True, // automatically is true
    Nullary(fn() -> bool),
    Unary(fn(StateToCheck) -> bool),
}

pub trait Condition<StateToCheck> {
    fn check() -> ConditionFunction<StateToCheck>;
}

struct WaitUntil<StateToCheck, OutputState> {
    state: WaitUntilState,
}

async fn mything() -> u8 {
    return 1;
}

fn notmain() {}

fn main() {
    let initial_position = Vector3 {
        x: 5.0,
        y: 0.0,
        z: 3.0,
    };

    let vec_to_add = Vector3 {
        x: 1.0,
        y: 3.0,
        z: 0.0,
    };

    let v3_soa_vec: Vector3_SOA_vec = Vector3_SOA_vec {
        x: vec![1.0],
        y: vec![1.0],
        z: vec![1.0],
    };

    let v3_soa_vec_x: Vec<f64> = v3_soa_vec.x;

    match v3_soa_vec_x.get(0) {
        Some(value) => println!("v3_soa_vec.x[0] is {}, which is correct", value),
        _ => println!("nothing here!, which is incorrect"),
    }
    match v3_soa_vec_x.get(1) {
        Some(value) => println!("v3_soa_vec.x[1] is {}, which is incorrect", value),
        _ => println!("nothing here, which is correct!"),
    }

    let v3_soa_arr: Vector3_SOA<1> = Vector3_SOA {
        x: [1.0],
        y: [1.0],
        z: [1.0],
    };

    let sum = initial_position + vec_to_add;

    println!("Sum of initial_position and vec_to_add = {}", sum);

    match v3_example() {
        Some(v) => println!("found a value {} ", v),
        _ => println!("no value"),
    }
}
