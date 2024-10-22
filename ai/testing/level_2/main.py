import uvicorn
import argparse
# from flask import Flask, request, json.dumps
from fastapi import FastAPI, Request
import json
import math

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
        'last_pos': [0,0],  # Initialize last position for stuck detection,
        'turning': False,
        'last_move': 'move_forward',
        'i1': 0,
    }
    
    print(f"New game: {game_id}")

    return {'message': f'Game {game_id} started successfully', 'game_id': game_id}

import timeit

@app.post('/brain')
async def brain(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    # Check if game_id exists
    if not game_id:
        return {'error': 'Game ID is required'}, 400
    if game_id not in games:
        return {'error': 'Game not found'}, 404
    
    game_data = games[game_id]
    game_data["i1"]+=1

    if game_data["i1"] % 10 == 0:
        return {"action": "shoot"}

    hull_vision = data.get('hull_vision', [])
    turret_vision = data.get('turret_vision', [])
    current_pos = data.get('pos')  # Current position of the tank
    current_rot = data.get('rot')

    # Threshold distance for detecting a wall
    wall_threshold = 40.0

    # Function to process each ray
    def get_wall_distance(ray):
        if ray is None:
            return None
        if 'Wall' in ray:
            return ray['Wall']
        return None

    # Hull-based wall detection
    front_wall = get_wall_distance(hull_vision[0])  # Directly forward
    back_wall = get_wall_distance(hull_vision[4])  # Directly forward
    left_wall = get_wall_distance(hull_vision[7])   # Front-left
    right_wall = get_wall_distance(hull_vision[1])  # Front-right

    print(front_wall)
    print(left_wall)
    print(right_wall)
    print(back_wall)
    print(game_data["last_move"])

    # print(int(current_rot * 10) % int(math.pi / 4. * 10))
    if not game_data["turning"]:
        if left_wall is not None:
            if left_wall < wall_threshold:
                game_data["turning"] = True
                game_data["last_move"] = "rotate_right"
                return {"action": "rotate_right"}
        if right_wall is not None:
            if right_wall < wall_threshold:
                game_data["turning"] = True
                game_data["last_move"] = "rotate_left"
                return {"action": "rotate_left"}
        if front_wall is None:
            game_data["last_move"] = "move_forward"
            return {"action": "move_forward"}

        if front_wall < wall_threshold:
            game_data["turning"] = True
            game_data["last_move"] = "rotate_left"
            return {"action": "rotate_left"}
        
        game_data["last_move"] = "move_forward"
        return {"action": "move_forward"}
    elif int(current_rot * 10) % int(math.pi / 4. * 10) < 3:
        
        game_data["last_move"] = "move_forward"
        game_data["turning"] = False
        return {"action": "move_forward"}
    
    else:
        if left_wall is not None and right_wall is not None:
            print(f"dist_between left and right - {abs(left_wall - right_wall)}")
            if abs(left_wall - right_wall) < 15:
                game_data["turning"] = False
                if front_wall is not None and back_wall is not None:
                    if front_wall < back_wall:
                        game_data["last_move"] = "move_backward"
                        return {"action": "move_backward"}
                    game_data["last_move"] = "move_forward"
                    return {"action": "move_forward"}
                    
                elif front_wall is not None:
                    game_data["last_move"] = "move_backward"
                    return {"action": "move_backward"}
                
                game_data["last_move"] = "move_forward"
                return {"action": "move_forward"}
            elif left_wall < right_wall:
                game_data["last_move"] = "rotate_right"
                return {"action": "rotate_right"}
            else:
                game_data["last_move"] = "rotate_left"
                return {"action": "rotate_left"}
        elif left_wall is not None:
            game_data["last_move"] = "rotate_right"
            return {"action": "rotate_right"}
        elif right_wall is not None:
            game_data["last_move"] = "rotate_left"
            return {"action": "rotate_left"}
        return {"action": game_data["last_move"]}

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