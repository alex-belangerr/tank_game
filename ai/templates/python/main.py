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

    games[game_id] = { #initialize the game id to help with brain
        'status': 'active',
        'counting': 0,
        'turning': False,
        'old_pos': (0, 0),
        'old_rot': 0,
        'turret_state': False
    }

    print(f"New game: {game_id}")

    return {'message': f'Game {game_id} started successfully', 'game_id': game_id}

#ADDING HELPER FUNCTIONS
def dist(p1, p2): #distance between two points returns a float
    return (p1[0]-p2[0])**2 + (p1[1]-p2[1])**2

def is_blocked(sensor, min_dist=30.0):
    #check if a sensor is blocked within a specified distance.
    if sensor is None:
        return False, -1.0  # No data
    value = next(iter(sensor.values()))  # Sensor value
    return value < min_dist, value


def decide_turn_direction(sensors):
    #determine whether to turn left or right based on sensor data.
    left_dist = max(is_blocked(sensors["nw"])[1], is_blocked(sensors["w"])[1], is_blocked(sensors["sw"])[1])
    right_dist = max(is_blocked(sensors["ne"])[1], is_blocked(sensors["e"])[1], is_blocked(sensors["se"])[1])
    
    if left_dist < right_dist:
        return "rotate_right"
    return "rotate_left"

def aim_turret(turret_vision): #aim the turret towards the enemy
    for idx, vision in enumerate(turret_vision):
        if vision and "Enemy" in vision:
            if idx < len(turret_vision) // 2:
                return "spin_left"
            return "spin_right"
    return None



@app.post('/brain')
async def brain(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    # Check if game_id exists
    games[game_id]['turret_state'] = True

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

        if is_blocked(sensors["n"])[0]:
            return {"action":"rotate_right"}
        return {"action":"move_forward"}


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