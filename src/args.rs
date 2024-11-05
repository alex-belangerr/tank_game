//! This module handles command-line arguments for configuring the game settings.
//! It defines the `GameBuilder` struct for creating game instances and includes
//! functionality to parse command-line arguments to customize player controls and
//! rendering settings.

use std::net::{Ipv4Addr, Ipv6Addr};
use std::env;

use uuid::Uuid;

use crate::player::PlayerController;

/// A builder for configuring game settings.
///
/// This struct allows customization of the game by specifying player controls
/// and rendering options.
#[derive(Debug, PartialEq, Eq)]
pub struct GameBuilder {
    pub render: bool,
    pub player_1: PlayerController<0>,
    pub player_2: PlayerController<1>,
    pub map: Option<String>,
    pub delta_time: Option<u32>

}
impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            render: true,
            player_1: PlayerController::wasd(),
            player_2: PlayerController::arrow(),
            map: None,
            delta_time: None
        }
    }
}

#[derive(Debug)]
enum ReaderState{
    Render,
    Player1,
    Player2,
    Map,
    DeltaTime,
    None
}

/// Parses command-line arguments and constructs a `GameBuilder` instance.
///
/// # Returns
/// A `GameBuilder` instance configured based on the provided command-line
/// arguments.
pub fn get_args() -> GameBuilder {
    let args = env::args().into_iter().skip(1).collect::<Vec<String>>();
    read_args(args.into_iter())
}

/// Reads command-line arguments from the provided iterator and constructs
/// a `GameBuilder` instance.
///
/// # Parameters
/// - `args`: An iterator over command-line arguments.
///
/// # Returns
/// A `GameBuilder` instance configured based on the parsed arguments.
fn read_args<'a, I: Iterator<Item = String>>(args: I) -> GameBuilder {
    let mut state: ReaderState = ReaderState::None;

    let id = Uuid::new_v4().to_string();

    let mut builder: GameBuilder = GameBuilder::default();

    for arg in args{
        let arg = arg.as_str();
        match (arg, &mut state) {
            ("-render" | "-r", ReaderState::None) => {
                state = ReaderState::Render
            },
            ("-p1" | "-player_1", ReaderState::None) => state = ReaderState::Player1,
            ("-p2" | "-player_2", ReaderState::None) => state = ReaderState::Player2,

            ("-map", ReaderState::None) => state = ReaderState::Map,
            
            ("-dt" | "-delta_time" | "-t" | "-time", ReaderState::None) => state = ReaderState::DeltaTime,

            ("t" | "true", ReaderState::Render) => {
                builder.render = true;
                state = ReaderState::None;
            },
            ("f" | "false", ReaderState::Render) => {
                builder.render = false;
                state = ReaderState::None;
            },

            ("wasd", ReaderState::Player1) => {
                builder.player_1 = PlayerController::wasd();
                state = ReaderState::None;
            },
            ("arrow", ReaderState::Player1) => {
                builder.player_1 = PlayerController::arrow();
                state = ReaderState::None;
            },
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
                        port: port.parse::<u16>().unwrap(),
                        game_id: id.clone()
                    },
                    _ => panic!("Invalid format")
                };

                builder.player_1 = tmp;
                state = ReaderState::None;
            },

            ("wasd", ReaderState::Player2) => {
                builder.player_2 = PlayerController::wasd();
                state = ReaderState::None;
            },
            ("arrow", ReaderState::Player2) => {
                builder.player_2 = PlayerController::arrow();
                state = ReaderState::None;
            },
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
                        port: port.parse::<u16>().unwrap(),
                        game_id: id.clone()
                    },
                    _ => panic!("Invalid format")
                };

                builder.player_2 = tmp;
                state = ReaderState::None;
            },

            
            ("random" | "None", ReaderState::Map) => {
                builder.map = None;
                state = ReaderState::None;
            },
            (asset_path, ReaderState::Map) => {
                builder.map = Some(asset_path.to_string());
                state = ReaderState::None;
            },

            ("default", ReaderState::DeltaTime) => {
                builder.delta_time = None;
                state = ReaderState::None;
            },
            (time, ReaderState::DeltaTime) => {
                builder.delta_time = Some(time.parse().unwrap());
                state = ReaderState::None;
            },
            
            state => {
                panic!("{state:#?}")
            }
        }
    }

    builder
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_empty_vec(){
        assert_eq!(
            GameBuilder::default(),
            read_args(Vec::new().into_iter())
        )
    }

    #[test]
    fn test_render(){
        let render_true = {
            let mut tmp = GameBuilder::default();

            tmp.render = true;

            tmp
        };
        let render_false = {
            let mut tmp = GameBuilder::default();

            tmp.render = false;

            tmp
        };

        assert_eq!(
            render_true,
            read_args([format!("-r"), format!("t")].into_iter())
        );
        assert_eq!(
            render_true,
            read_args([format!("-r"), format!("true")].into_iter())
        );

        assert_eq!(
            render_false,
            read_args([format!("-r"), format!("f")].into_iter())
        );

        assert_eq!(
            render_false,
            read_args([format!("-r"), format!("false")].into_iter())
        );
    }
    
    // #[test]
    // fn test_player_control(){
    //     let game_builder_1 = {
    //         let mut tmp = GameBuilder::default();

    //         tmp.player_1 = PlayerController::arrow();
    //         tmp.player_2 = PlayerController::wasd();

    //         tmp
    //     };
    //     let game_builder_2 = {
    //         let mut tmp = GameBuilder::default();

    //         tmp.player_1 = PlayerController::Server { ip: Ipv4Addr::new(0, 0, 0, 0).into(), port: 244 };
    //         tmp.player_2 = PlayerController::Server { ip: Ipv4Addr::new(1, 2, 3, 4).into(), port: 12 };

    //         tmp
    //     };

    //     assert_eq!(
    //         game_builder_1,
    //         read_args([format!("-p1"), format!("arrow"), format!("-p2"), format!("wasd")].into_iter())
    //     );
    //     assert_eq!(
    //         game_builder_1,
    //         read_args([format!("-player_1"), format!("arrow"), format!("-player_2"), format!("wasd")].into_iter())
    //     );
    //     assert_eq!(
    //         game_builder_1,
    //         read_args([format!("-p1"), format!("arrow"), format!("-player_2"), format!("wasd")].into_iter())
    //     );
    //     assert_eq!(
    //         game_builder_1,
    //         read_args([format!("-player_1"), format!("arrow"), format!("-p2"), format!("wasd")].into_iter())
    //     );
    //     assert_eq!(
    //         game_builder_2,
    //         read_args([format!("-player_1"), format!("0.0.0.0:244"), format!("-p2"), format!("1.2.3.4:12")].into_iter())
    //     );
    // }

    // #[test]
    // fn test_order_args(){
    //     let game_builder = {
    //         let mut tmp = GameBuilder::default();

    //         tmp.player_1 = PlayerController::arrow();
    //         tmp.player_2 = PlayerController::Server { ip: Ipv4Addr::new(0, 0, 0, 0).into(), port: 244 };

    //         tmp.render = false;

    //         tmp
    //     };
        
    //     assert_eq!(
    //         game_builder,
    //         read_args([
    //             format!("-p1"), format!("arrow"),
    //             format!("-r"), format!("false"),
    //             format!("-p2"), format!("0.0.0.0:244")
    //         ].into_iter())
    //     );
    // }

    #[test]
    #[should_panic]
    fn test_bad_args_1(){
        read_args([
            format!("-p1"),
            format!("-r"),
            format!("-p2")
        ].into_iter());
    }

    #[test]
    #[should_panic]
    fn test_bad_args_2(){
        read_args([
            format!("-p2"), format!("asdf.sdf.df.df:34dgfdf")
        ].into_iter());
    }
}