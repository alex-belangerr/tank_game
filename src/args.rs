use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::env;

use bevy::prelude::KeyCode;

#[derive(Debug, PartialEq, Eq)]
pub struct GameBuilder {
    pub render: bool,
    pub player_1: PlayerController,
    pub player_2: PlayerController,
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


#[derive(Debug, PartialEq, Eq)]
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
        PlayerController::Control{
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            rotate_left: KeyCode::KeyA,
            rotate_right: KeyCode::KeyD,
            spin_turret_left: KeyCode::KeyQ,
            spin_turret_right: KeyCode::KeyE,
            shoot: KeyCode::Space,
        }
    }
    pub fn arrow() -> PlayerController{
        PlayerController::Control{
            move_forward: KeyCode::ArrowUp,
            move_backward: KeyCode::ArrowDown,
            rotate_left: KeyCode::ArrowLeft,
            rotate_right: KeyCode::ArrowRight,
            spin_turret_left: KeyCode::KeyI,
            spin_turret_right: KeyCode::KeyO,
            shoot: KeyCode::KeyP,
        }
    }
}

#[derive(Debug)]
enum ReaderState{
    Render,
    Player1,
    Player2,
    None
}
pub fn get_args() -> GameBuilder {
    read_args(env::args().into_iter())
}

fn read_args<'a, I: Iterator<Item = String>>(args: I) -> GameBuilder {
    let mut state: ReaderState = ReaderState::None;

    let mut builder: GameBuilder = GameBuilder::default();

    for arg in args{
        let arg = arg.as_str();
        match (arg, &mut state) {
            ("-render" | "-r", ReaderState::None) => {
                state = ReaderState::Render
            },
            ("-p1" | "-player_1", ReaderState::None) => state = ReaderState::Player1,
            ("-p2" | "-player_2", ReaderState::None) => state = ReaderState::Player2,

            ("t" | "true", ReaderState::Render) => builder.render = true,
            ("f" | "false", ReaderState::Render) => builder.render = false,

            ("wasd", ReaderState::Player1) => builder.player_1 = PlayerController::wasd(),
            ("arrow", ReaderState::Player1) => builder.player_1 = PlayerController::arrow(),
            (ip, ReaderState::Player1) => {
                let ip = ip.split(":").collect::<Vec<&str>>();

                let tmp = match ip[..] {
                    [ip, port] => PlayerController::Server{
                        ip: {
                            let ip = ip.split(".").collect::<Vec<&str>>();

                            match ip[..]{
                                [a, b, c, d] => {
                                    let a = a.parse().unwrap();
                                    let b = b.parse().unwrap();
                                    let c = c.parse().unwrap();
                                    let d = d.parse().unwrap();

                                    Ipv4Addr::new(a, b, c, d).into()
                                },
                                [a, b, c, d, e, f, g, h] => {
                                    let a = a.parse().unwrap();
                                    let b = b.parse().unwrap();
                                    let c = c.parse().unwrap();
                                    let d = d.parse().unwrap();
                                    let e = e.parse().unwrap();
                                    let f = f.parse().unwrap();
                                    let g = g.parse().unwrap();
                                    let h = h.parse().unwrap();

                                    Ipv6Addr::new(a, b, c, d, e, f, g, h).into()
                                }
                                _ => panic!("Invalid IP")
                            }
                        },
                        port: port.parse::<u16>().unwrap()
                    },
                    _ => panic!("Invalid format")
                };

                builder.player_1 = tmp;
            },

            ("wasd", ReaderState::Player2) => builder.player_2 = PlayerController::wasd(),
            ("arrow", ReaderState::Player2) => builder.player_2 = PlayerController::arrow(),
            (ip, ReaderState::Player2) => {
                let ip = ip.split(":").collect::<Vec<&str>>();

                let tmp = match ip[..] {
                    [ip, port] => PlayerController::Server{
                        ip: {
                            let ip = ip.split(".").collect::<Vec<&str>>();

                            match ip[..]{
                                [a, b, c, d] => {
                                    let a = a.parse().unwrap();
                                    let b = b.parse().unwrap();
                                    let c = c.parse().unwrap();
                                    let d = d.parse().unwrap();

                                    Ipv4Addr::new(a, b, c, d).into()
                                },
                                [a, b, c, d, e, f, g, h] => {
                                    let a = a.parse().unwrap();
                                    let b = b.parse().unwrap();
                                    let c = c.parse().unwrap();
                                    let d = d.parse().unwrap();
                                    let e = e.parse().unwrap();
                                    let f = f.parse().unwrap();
                                    let g = g.parse().unwrap();
                                    let h = h.parse().unwrap();

                                    Ipv6Addr::new(a, b, c, d, e, f, g, h).into()
                                }
                                _ => panic!("Invalid IP")
                            }
                        },
                        port: port.parse::<u16>().unwrap()
                    },
                    _ => panic!("Invalid format")
                };

                builder.player_2 = tmp;
            },
            
            state => {
                panic!("{state:#?}")
            }
        }
    }

    builder
}

mod tests{

}