use std::{process::{Child, Command}, sync::{Arc, RwLock}, thread, time::Duration};


fn cmd(inst: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", inst])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(inst)
            .output()
            .expect("failed to execute process")
    };

    return String::from_utf8(output.stdout).unwrap().trim().into();
}

fn check_and_kill_listeners(port: u16) {
    if cfg!(target_os = "windows") {
        let netstat_command = format!("netstat -ano | findstr :{}", port);
        let output = cmd(&netstat_command);
        
        if !output.is_empty() {
            for line in output.lines() {
                if let Some(pid) = line.split_whitespace().last() {
                    if let Ok(pid) = pid.parse::<u32>() {
                        let kill_command = format!("taskkill /PID {} /F", pid);
                        cmd(&kill_command);  // Kill the process
                        println!("Killed process with PID: {}", pid);
                    }
                }
            }
        }
    }
}

fn has_python() {
    let has_python = cmd("python --version");

    assert!(has_python.starts_with("Python "), "Python not found or unexpected output: {}", has_python);
}

fn start_server(command: &str) -> Child {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .spawn()
            .expect("failed to start server")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("failed to start server")
    }
}

#[test]
fn render_test() {
    has_python();

    let kill_flag = Arc::new(RwLock::new(false));
    
    let server_1 = {
        let kill_flag = kill_flag.clone();

        thread::spawn(move || {
            check_and_kill_listeners(5001);
            let mut server = start_server("python ai\\testing\\level_1\\main.py --port 5001");

            while !*kill_flag.read().unwrap() {
                thread::sleep(Duration::from_millis(500));
            };

            server.kill().expect("failed to terminate server 1");
            check_and_kill_listeners(5001);
        })
    };
    let server_2 = {

        let kill_flag = kill_flag.clone();

        thread::spawn(move || {
            check_and_kill_listeners(5000);
            let mut server = start_server("python ai\\testing\\level_4\\main.py --port 5000");

            while !*kill_flag.read().unwrap() {
                thread::sleep(Duration::from_millis(500));
            };

            server.kill().expect("failed to terminate server 1");
            check_and_kill_listeners(5000);
        })
    };


    let game_output = cmd("cargo run -- -p2 127.0.0.1:5000 -p1 127.0.0.1:5001 -map test_1.ron");

    *kill_flag.write().unwrap() = true;
    let _ = server_1.join();
    let _ = server_2.join();

    println!("{}", game_output);

    let last_line = game_output.lines().rev().find(|line| !line.is_empty()).unwrap_or("");

    assert_eq!(last_line, "1", "Expected last line to be '1' but got '{}'", last_line);
}
#[test]
fn headless_test() {
    has_python();

    let kill_flag = Arc::new(RwLock::new(false));
    
    let server_1 = {
        let kill_flag = kill_flag.clone();

        thread::spawn(move || {
            check_and_kill_listeners(5002);
            let mut server = start_server("python ai\\testing\\level_1\\main.py --port 5002");

            while !*kill_flag.read().unwrap() {
                thread::sleep(Duration::from_millis(500));
            };

            server.kill().expect("failed to terminate server 1");
            check_and_kill_listeners(5002);
        })
    };
    let server_2 = {

        let kill_flag = kill_flag.clone();

        thread::spawn(move || {
            check_and_kill_listeners(5003);
            let mut server = start_server("python ai\\testing\\level_4\\main.py --port 5003");

            while !*kill_flag.read().unwrap() {
                thread::sleep(Duration::from_millis(500));
            };
            
            server.kill().expect("failed to terminate server 1");
            check_and_kill_listeners(5003);
        })
    };


    let game_output = cmd("cargo run -- -p2 127.0.0.1:5003 -p1 127.0.0.1:5002 -map test_1.ron -r false -time 10");

    *kill_flag.write().unwrap() = true;
    let _ = server_1.join();
    let _ = server_2.join();

    println!("{}", game_output);

    let last_line = game_output.lines().rev().find(|line| !line.is_empty()).unwrap_or("");

    assert_eq!(last_line, "1", "Expected last line to be '1' but got '{}'", last_line);
}