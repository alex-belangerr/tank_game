import uvicorn
import argparse
# from flask import Flask, request, json.dumps
from fastapi import FastAPI, Request
import json

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
        'last_pos': None,  # Initialize last position for stuck detection,
        'last_rot': None,
        'stuck_counter': 0,
        'stuck_check': False,
        'shoot_counter': 0
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

    # Extract hull vision data (8 rays), turret vision (5 rays), and position
    hull_vision = data.get('hull_vision', [])
    turret_vision = data.get('turret_vision', [])
    current_pos = data.get('pos')  # Current position of the tank
    current_rot = data.get('rot')

    # Check if the tank has moved
    if games[game_id]['last_pos'] is not None and games[game_id]['last_rot'] is not None and not games[game_id]['stuck_check']:
        distance_moved = (
            (current_pos[0] - games[game_id]['last_pos'][0]) ** 2 +
            (current_pos[1] - games[game_id]['last_pos'][1]) ** 2
        )** 0.5

        rot = max(games[game_id]['last_rot'], current_rot) - min(games[game_id]['last_rot'], current_rot)

        if distance_moved < 0.000001 and rot < 0.00001:  # Adjust threshold as needed
            games[game_id]['stuck_check'] = True
            games[game_id]['stuck_counter'] = 0
        else: 
            games[game_id]['stuck_check'] = False
            games[game_id]['stuck_counter'] = 0
    games[game_id]['last_rot'] = current_rot
    games[game_id]['last_pos'] = current_pos  # Update last position

    # Threshold distance for detecting a wall
    wall_threshold = 50.0

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

    # Handle anti-stuck behavior
    if games[game_id]['stuck_check']:
        if games[game_id]['stuck_counter'] < 10:
            action="move_backward"
            if games[game_id]['stuck_counter'] % 2 == 0:
                action = 'move_backward'
            else:
                action = 'rotate_left'
                    
            print(games[game_id]['stuck_counter'])
            games[game_id]['stuck_counter'] = 1 + games[game_id]['stuck_counter']
            
            print(games[game_id]['stuck_counter'])

            print(f"Game {game_id} action: {action} (anti-stuck, iterations left: {games[game_id]['stuck_counter']})")
            return {'action': action}
        else:
            games[game_id]['stuck_check'] = False
            games[game_id]['stuck_counter'] = 0

    # AI Logic for Movement (based on hull vision)
    if front_wall and front_wall > wall_threshold:
        # Wall detected directly in front
        if left_wall and left_wall > wall_threshold and right_wall and right_wall < wall_threshold:
            # If there's a wall on both sides, choose to rotate away from the closer wall
            if left_wall < right_wall:
                action = 'rotate_right'  # Rotate right if the left wall is closer
            else:
                action = 'rotate_left'   # Rotate left if the right wall is closer
        elif left_wall and left_wall < wall_threshold:
            # If there's a wall to the left, turn right
            action = 'rotate_right'
        elif right_wall and right_wall < wall_threshold:
            # If there's a wall to the right, turn left
            action = 'rotate_left'
        else:
            # If no walls are close on either side, you can choose a default action
            action = 'move_forward'  # Or any other suitable action
    else:
        # No walls in front or sides, move forward
        action = 'move_forward'


    # Shoot every second brain call and only if a wall is detected within turret vision
    games[game_id]['shoot_counter'] += 1
    if games[game_id]['shoot_counter'] % 5 == 0:
        action = 'shoot'

    return {'action': action}

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