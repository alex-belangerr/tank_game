use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::env;

use bevy::prelude::KeyCode;

#[derive(Debug, )]
struct GameBuilder {
    render: bool,
    player_1: Option<bool>,
    player_2: Option<bool>,
}
impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            render: true,
            player_1: PlayerController::wasd(),
            player_2: PlayerController::arrow()
        }
    }
}


#[derive(Debug)]
enum PlayerController {
    Server{
        ip: IpAddr,
        port: u16
    },
    Control{
        move_forward: KeyCode,
        move_backward: KeyCode,
        rotate_left: KeyCode,
        rotate_right: KeyCode,

        spin_turret_left: KeyCode,
        spin_turret_right: KeyCode,
        shoot: KeyCode
    }
}
impl PlayerController {
    pub fn wasd() -> PlayerController {
        todo!();
        PlayerController{

        }
    }
    pub fn arrow() -> PlayerController{
        todo!();
    }
}

#[derive(Debug)]
enum State{
    In,
    Out,
    None
}
pub fn get_args() -> GameBuilder {
    let mut state = State::None;
    let args = env::args();

    let mut input: Option<String> = None;
    let mut output: Option<String> = None;

    for arg in args {
        todo!()
        // match (&arg as &str, &mut state, &mut input, &mut output) {
        //     ("-i", state, None, _) => *state = State::In,
        //     ("-o", state, _, None) => *state = State::Out,

        //     (arg, State::In, Some(input), _) => input.extend(format!(" {arg}").chars()),
        //     (arg, State::In, input, _) => *input = Some(arg.into()),
            
        //     (arg, State::Out, _, Some(output)) => output.extend(format!(" {arg}").chars()),
        //     (arg, State::Out, _, output) => *output = Some(arg.into()),

        //     (_arg, State::None, None, None) => {}
            
        //     state => {
        //         panic!("{state:#?}")
        //     }
        // }
    }

    Some((input?,output?))
}