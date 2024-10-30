import uvicorn
import argparse
# from flask import Flask, request, json.dumps
from fastapi import FastAPI, Request
import json
import math
from math import pi

from typing import Tuple

app = FastAPI()

# In-memory storage for game data (for simplicity)
games = {}

@app.post('/start_game')
async def start_game(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    if not game_id:
        print("No game id")
        return json.dumps({'error': 'Game ID is required'}), 400

    # Initialize game data
    games[game_id] = {
        'status': 'active',
        'counting': 0,
        'turning': False,
        'old_pos': (0, 0),
        'old_rot': 0
    }


    print(f"New game: {game_id}")

    return {'message': f'Game {game_id} started successfully', 'game_id': game_id}

def dist(p1, p2):
    return (p1[0]-p2[0])**2 + (p1[1]-p2[1])**2

def normalize_angle(angle):
    return angle % (2 * math.pi)

def angle_distance(angle_1, angle_2):
    angle_1 = normalize_angle(angle_1)
    angle_2 = normalize_angle(angle_2)
    
    diff = abs(angle_2 - angle_1)
    return min(diff, 2 * math.pi - diff)

def is_blocked(sensor, min_dist = 30.) -> Tuple[bool, float]:
    if sensor is None:
        return False, -1.
    
    value = next(iter(sensor.values()))

    return value < min_dist, value

@app.post('/brain')
async def brain(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    # Check if game_id exists
    if not game_id:
        return {'error': 'Game ID is required'}, 400
    if game_id not in games:
        return {'error': 'Game not found'}, 404
    
    counting = games[game_id]["counting"]
    turning = games[game_id]["turning"]
    old_pos = games[game_id]["old_pos"]
    old_rot = games[game_id]["old_rot"]

    current_heading = data["rot"]
    current_pos = data["pos"]
    
    sensors = {
        "n": data["hull_vision"][0],
        "ne": data["hull_vision"][1],
        "e": data["hull_vision"][2],
        "se": data["hull_vision"][3],
        "s": data["hull_vision"][4],
        "sw": data["hull_vision"][5],
        "w": data["hull_vision"][6],
        "nw": data["hull_vision"][7]
    }
    
    if turning is None:
        games[game_id]["counting"] = 0
        forward_blocked = is_blocked(sensors['n'], min_dist=45.)[0] or\
            is_blocked(sensors['ne'], min_dist=40.)[0] or\
            is_blocked(sensors['nw'], min_dist=40.)[0]
        if forward_blocked:
            print("time to turn")
            left_dist = max(
                is_blocked(sensors['w'])[1],
                is_blocked(sensors['nw'])[1],
            )
            right_dist = max(
                is_blocked(sensors['e'])[1],
                is_blocked(sensors['ne'])[1],
            )
            if left_dist < right_dist:
                print("rotate right")
                games[game_id]["turning"] = 'right'
            else:
                print("rotate left")
                games[game_id]["turning"] = 'left'
            games[game_id]["old_pos"] = current_pos
            return {"action": "move_backward"}
        else:
            print("Forward")
            games[game_id]["old_pos"] = current_pos
            return {"action": "move_forward"}
    else:
        print("Turning")
        forward_blocked = is_blocked(sensors['n'], min_dist=70.)[0] or\
            is_blocked(sensors['ne'], min_dist=40.)[0] or\
            is_blocked(sensors['nw'], min_dist=40.)[0]
        
        print(is_blocked(sensors['n'], min_dist=70.))
        print(is_blocked(sensors['ne'], min_dist=40.))
        print(is_blocked(sensors['nw'], min_dist=40.))
        good_rot = [float(x) * pi/4 for x in range(8)]

        facing_good_dir = any(
            map(
                lambda x: angle_distance(x, current_heading) < 0.1,
                good_rot
            )
        )

        if angle_distance(current_heading, old_rot) < 0.00001:
            print("We might be stuck")
            counting=1+(counting % 30)
            counting = games[game_id]["counting"]

            if counting == 29:
                print("Move back")
                games[game_id]["old_pos"] = current_pos
                return {"action": "move_backward"}
        if facing_good_dir and not forward_blocked:
            print("No longer blocked")
            games[game_id]["turning"] = None
            games[game_id]["old_pos"] = current_pos
            return {"action": "move_forward"}
        
        print(f"rotate_{games[game_id]['turning']}")
        games[game_id]["old_rot"] = current_heading
        return {"action": f"rotate_{games[game_id]['turning']}"}
        
        # if turning == "left":
        #     my_tank.turn_left()
        # elif turning == "right":
        #     my_tank.turn_right()
        # if my_tank.my_heading()%90 == 0:
        #     turning = False
        #     counting = 0
        #     oldpos = mypos
    
    # return {"action": "spin_left"}

@app.post('/win')
async def win(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    if not game_id:
        return json.dumps({'error': 'Game ID is required'}), 400

    if game_id not in games:
        return json.dumps({'error': 'Game not found'}), 404

    # Mark the game as ended
    del games[game_id]

    print(":)")

    return {'message': f'Game {game_id} ended successfully', 'game_id': game_id}

@app.post('/loss')
async def loss(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    if not game_id:
        return json.dumps({'error': 'Game ID is required'}), 400

    if game_id not in games:
        return json.dumps({'error': 'Game not found'}), 404

    # Mark the game as ended
    del games[game_id]

    print(":(")

    return json.dumps({'message': f'Game {game_id} ended successfully', 'game_id': game_id})

if __name__ == '__main__':
    # Argument parser for handling the port input
    parser = argparse.ArgumentParser(description="Flask Tank Game AI")
    parser.add_argument('--port', type=int, default=5000, help='Port to run the server on')
    
    # Parse the arguments
    args = parser.parse_args()
    
    # Run the app on the specified port
    uvicorn.run(app, host="0.0.0.0", port=args.port)