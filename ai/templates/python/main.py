import json
import uvicorn
import argparse
from fastapi import FastAPI, Request
# Alex Belangers implementation
app = FastAPI()

games = {}

@app.post('/start_game')
async def start_game(request: Request):
    data = await request.json()
    game_id = data.get('game_id')    

    if not game_id:
        print("No game id")
        return json.dumps({'error': 'Game ID is required'}), 400

    games[game_id] = {
        'status': 'active',
        'counting': 0,
        'turning': False,
        'old_pos': (0, 0),
        'old_rot': 0,
        'turret_state': False
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
    
    counting = games[game_id]["counting"]
    turning = games[game_id]["turning"]
    old_pos = games[game_id]["old_pos"]
    old_rot = games[game_id]["old_rot"]

    current_heading = data["rot"]
    current_pos = data["pos"]
    if games[game_id]['turret_state']:
        games[game_id]['turret_state'] = False
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

    print("Winner Winner Chicken Dinner!")

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

    print("Lock in we lost")

    return json.dumps({'message': f'Game {game_id} ended successfully', 'game_id': game_id})

if __name__ == '__main__':
    # Argument parser for handling the port input
    parser = argparse.ArgumentParser(description="Tank Game AI")
    parser.add_argument('--port', type=int, default=5000, help='Port to run the server on')
    
    # Parse the arguments
    args = parser.parse_args()
    
    # Run the app on the specified port
    uvicorn.run(app, host="0.0.0.0", port=args.port)