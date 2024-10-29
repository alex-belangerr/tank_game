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
    }
    
    print(f"New game: {game_id}")

    return {'message': f'Game {game_id} started successfully', 'game_id': game_id}

@app.post('/brain')
async def brain(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    # Check if game_id exists
    if not game_id:
        return {'error': 'Game ID is required'}, 400
    if game_id not in games:
        return {'error': 'Game not found'}, 404
    
    turret_vision = data["turret_vision"]

    left_vision = any(
        map(
            lambda x: "Enemy" in x if x is not None else False,
            turret_vision[0:2]
        )
    )
    center_vision = "Enemy" in turret_vision[2] if turret_vision[2] is not None else False
    right_vision = any(
        map(
            lambda x: "Enemy" in x if x is not None else False,
            turret_vision[3:]
        )
    )
    if center_vision:
        return {"action": "shoot"}
    elif left_vision:
        return {"action": "spin_left"}
    elif right_vision:
        return {"action": "spin_right"}
    
    return {"action": "spin_left"}

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