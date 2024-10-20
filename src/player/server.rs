use std::{mem, net::IpAddr, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex, RwLock}, thread::{self, JoinHandle}, time::Duration};
use bevy::{math::Vec2, prelude::{EventWriter, GlobalTransform, Query, Res, ResMut, Resource, With, Without}, utils::hashbrown::HashMap};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::engine::tank::{gen::{Tank, Turret}, instruction::{get_rotation_z, Instruction}, vision::{VisionHit, VisionRay, NUM_OF_HULL_RAY, NUM_OF_TURRET_RAY}};

use super::PlayerID;

const REQUEST_WAIT: u64 = 10;

#[derive(Debug, Clone, Copy)]
struct PlayerData<const TURRET_RAYS: usize, const HULL_RAYS: usize> {
    pub pos: Vec2,
    pub turret_rot: f32,
    pub turret_vision: [Option<VisionHit>; TURRET_RAYS],
    pub hull_vision: [Option<VisionHit>; HULL_RAYS],
}

impl<const TURRET_RAYS: usize, const HULL_RAYS: usize> Default for PlayerData<TURRET_RAYS, HULL_RAYS> {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            turret_rot: Default::default(),
            turret_vision: [None; TURRET_RAYS],
            hull_vision: [None; HULL_RAYS]
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct PlayerDataSerialized {
    pub game_id: String,
    pub pos: Vec2,
    pub turret_rot: f32,
    pub turret_vision: Vec<Option<VisionHit>>,
    pub hull_vision: Vec<Option<VisionHit>>,
}

impl PlayerDataSerialized {
    pub fn new(game_id: &str) -> Self {
        let player_data: PlayerData<NUM_OF_TURRET_RAY, NUM_OF_HULL_RAY> = PlayerData::default();

        PlayerDataSerialized{
            game_id: game_id.to_string(),
            pos: player_data.pos,
            turret_rot: player_data.turret_rot,
            turret_vision: player_data.turret_vision.into(),
            hull_vision: player_data.hull_vision.into(),
            
        }
    }

    pub fn update(&mut self, player_data: &PlayerData<NUM_OF_TURRET_RAY, NUM_OF_HULL_RAY>) {
        self.pos = player_data.pos;
        self.turret_rot = player_data.turret_rot;
        self.turret_vision = player_data.turret_vision.into();
        self.hull_vision = player_data.hull_vision.into();
    }
}

impl<const T: usize, const H: usize> From<(String, PlayerData<T, H>)> for PlayerDataSerialized{
    fn from((game_id, player_data): (String, PlayerData<T, H>)) -> Self {
        PlayerDataSerialized{
            game_id: game_id,
            pos: player_data.pos,
            turret_rot: player_data.turret_rot,
            turret_vision: player_data.turret_vision.into(),
            hull_vision: player_data.hull_vision.into(),
        }
    }
}

#[derive(Deserialize)]
struct PlayerInstruction{
    action: String
}

#[derive(Resource)]
pub struct PlayerServer<const P_FLAG: u32>{
    request_loop: Arc<Mutex<Option<JoinHandle<()>>>>,
    kill_flag: Arc<RwLock<bool>>,
    win_cond: Arc<RwLock<bool>>,

    pub recv: Arc<Mutex<Receiver<Instruction<P_FLAG>>>>,
    send: Sender<PlayerData<NUM_OF_TURRET_RAY, NUM_OF_HULL_RAY>>
}

impl<const P_FLAG: u32> PlayerServer<P_FLAG>{
    pub fn new(ip: IpAddr, port: u16, game_id: &str) -> Self {
        
        // let mut request_body = 
        {
            let client = Client::new();

            let mut json = HashMap::new();
            json.insert("game_id", game_id);
            json.insert("server", "127.0.0.1");
            json.insert("port", "8080");

            let response = client.post(&format!("http://{ip}:{port}/start_game"))
                .json(&json)
                .timeout(Duration::from_secs(5))
                .send()
                .unwrap();

            if !response.status().is_success() {
                panic!("Failed to initialize")
            }
        }
        // todo!("Check if the server is active")
        let game_id= format!("{game_id}");
        let (send_inst, recv_inst) = mpsc::channel();
        let (
            send_player_data,
            recv_player_data
        ) = mpsc::channel::<PlayerData<NUM_OF_TURRET_RAY, NUM_OF_HULL_RAY>>();
        let kill_flag = Arc::new(RwLock::new(false));
        let win_cond = Arc::new(RwLock::new(false));
        let request_loop = {
            let kill_flag = kill_flag.clone();
            let win_cond = win_cond.clone();

            thread::spawn(move || {
                let game_id= game_id.as_str();
                let client = Client::new();

                //replace hash map with a better deserializable data
                let mut player_data: PlayerDataSerialized = PlayerDataSerialized::new(game_id);
                
                loop {
                    {
                        // let mut i1 = 0;
                        while let Ok(new_player_data) = recv_player_data.try_recv() {
                            // if 100 < i1 {
                            //     break;
                            // }
                            player_data.update(&new_player_data);
                            // i1 +=1;
                        }
                    }
                    let response = client.post(&format!("http://{ip}:{port}/brain"))
                        .json(&player_data)
                        .timeout(Duration::from_secs(5))
                        .send();
        
                    match response {
                        Ok(response) => {
                            if response.status().is_success() {
                                let response: PlayerInstruction = response.json().unwrap();

                                match response.action.as_str() {
                                    "shoot" => {
                                        let _ = send_inst.send(Instruction::Shoot);
                                    },
                                    
                                    "move_forward" => {
                                        let _ = send_inst.send(Instruction::MoveForward);
                                    },
                                    "move_backward" => {
                                        let _ = send_inst.send(Instruction::MoveBackward);
                                    },
                                    "rotate_left" => {
                                        let _ = send_inst.send(Instruction::RotateLeft);
                                    },
                                    "rotate_right" => {
                                        let _ = send_inst.send(Instruction::RotateRight);
                                    },

                                    "spin_left" => {
                                        let _ = send_inst.send(Instruction::SpinTurretLeft);
                                    },
                                    "spin_right" => {
                                        let _ = send_inst.send(Instruction::SpinTurretRight);
                                    },
                                    
                                    "wait" => {},
                                    _ => {
                                        //invalid action
                                    }
                                }
                            }
                        },
                        Err(_err) => {
                            // todo!("Error handling")
                        },
                    }
                    
                    thread::sleep(Duration::from_millis(REQUEST_WAIT));

                    {
                        let Ok(kill_flag) = kill_flag.read() else {
                            continue;
                        };

                        if *kill_flag {
                            let Ok(win_cond) = win_cond.read() else {
                                continue;
                            };
    
                            let json = {
                                let mut tmp = HashMap::new();
                                
                                tmp.insert(
                                    "game_id", 
                                    game_id
                                );
    
                                tmp
                            };
    
                            let _response = match *win_cond {
                                true => client.post(&format!("http://{ip}:{port}/win"))
                                    .json(&json)
                                    .timeout(Duration::from_secs(5))
                                    .send(),
                                false =>  client.post(&format!("http://{ip}:{port}/loss"))
                                    .json(&json)
                                    .timeout(Duration::from_secs(5))
                                    .send(),
                            };
                            return ;
                        }
            
                    }
                }
            })
        };

        PlayerServer{
            request_loop: Arc::new(Mutex::new(Some(request_loop))),
            kill_flag,
            win_cond,
            recv: Arc::new(Mutex::new(recv_inst)),
            send: send_player_data
        }
    }

    pub fn win(&mut self) -> bool {
        {
            let Ok(mut win_cond) =  self.win_cond.write() else {
                return false;
            };
            let Ok(mut kill_flag) = self.kill_flag.write() else {
                return false;
            };

            *kill_flag = true;
            *win_cond = true;
        }

        let Ok(mut server) = self.request_loop.lock() else {
            return false;
        };
        let mut tmp_server: Option<JoinHandle<()>> = None;

        match tmp_server{
            Some(join_handle) => {
                let _result = join_handle.join();
                true
            },
            None => {
                mem::swap(&mut *server, &mut tmp_server);
                true
            },
        }
    }
    pub fn lose(&mut self) -> bool {
        let Ok(mut kill_flag) = self.kill_flag.write() else {
            return false;
        };

        *kill_flag = true;
        
        let Ok(mut server) = self.request_loop.lock() else {
            return false;
        };
        let mut tmp_server: Option<JoinHandle<()>> = None;

        match tmp_server{
            Some(join_handle) => {
                let _result = join_handle.join();
                true
            },
            None => {
                mem::swap(&mut *server, &mut tmp_server);
                true
            },
        }
    }
}

pub fn update_player_data<const P_FLAG: u32>(
    player_server: Res<PlayerServer<P_FLAG>>,
    
    tank_query: Query<(&GlobalTransform, &Tank, &VisionRay<NUM_OF_HULL_RAY, Tank>, &VisionRay<NUM_OF_TURRET_RAY, Turret>), With<PlayerID<P_FLAG>>>,
    turret_query: Query<&GlobalTransform, With<Turret>>,
) {
    let Ok((transform, tank, tank_vision, turret_vision)) = tank_query.get_single() else {
        return;
    };
    let turret_transform = turret_query.get(tank.turret).expect("Tank lost ref to turret entity");

    let _ = player_server.send.send(
        PlayerData{
            pos: {
                let pos = transform.translation();

                Vec2::new(pos.x, pos.y)
            },
            turret_rot: get_rotation_z({
                let dir = turret_transform.compute_transform().up().as_vec3();

                Vec2::new(dir.x, dir.y)
            }),
            turret_vision: turret_vision.rays,
            hull_vision: tank_vision.rays,
        }
    );
}

pub fn server_input<const P_FLAG: u32>(
    mut player_server: ResMut<PlayerServer<P_FLAG>>,
    mut event_writer: EventWriter<Instruction<P_FLAG>>
) {
    let player_server = player_server.as_mut();

    // let Ok(kill_flag) = player_server.kill_flag.read() else {
    //     return;
    // };

    // if *kill_flag {
    //     return;
    // }

    match player_server.recv.try_lock() {
        Ok(recv) => {
            while let Ok(val) = recv.try_recv(){
                event_writer.send(val);
            };
        },
        Err(_) => {
            //skip
        },
    }
}

pub fn end_game_msg<const P_FLAG: u32>(
    mut player_server: ResMut<PlayerServer<P_FLAG>>,
    player_tanks: Query<(), (With<Tank>, With<PlayerID<P_FLAG>>)>,
    other_tanks: Query<(), (With<Tank>, Without<PlayerID<P_FLAG>>)>
) {
    let other_tanks_count = other_tanks.iter().count();
    let player_tanks_count = player_tanks.iter().count();

    #[cfg(feature="debug")]
    println!("other tanks:{other_tanks_count}\t{P_FLAG} tanks: {player_tanks_count}");

    let player_server = player_server.as_mut();

    match (player_tanks_count, other_tanks_count) {
        (0, _) => {
            player_server.lose();
        },
        (_, 0) => {
            player_server.win();
        },
         _=> {
            //do nothing
         }
    }
}