import uvicorn
import argparse
from fastapi import FastAPI, HTTPException, Request

app = FastAPI()

games = {}

@app.post('/start_game')
async def start_game(request: Request):
    data = await request.json()
    game_id = data.get('game_id')
    if not game_id:
        print("No game id")
        raise HTTPException(status_code=400, detail="No game id")

    # Initialize game data
    games[game_id] = {
        'status': 'active'
    }

    print(f"New game: {game_id}")

    return {'message': f'Game {game_id} started successfully', 'game_id': game_id}

@app.post('/brain')
async def brain(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    if not game_id:
        raise HTTPException(status_code=404, detail="Game not found")

    if game_id not in games:
        raise HTTPException(status_code=404, detail="Game not found")

    print(data)

    return {'action': 'spin_left'}

@app.post('/win')
async def win(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    if not game_id:
        raise HTTPException(status_code=404, detail="Game not found")

    if game_id not in games:
        raise HTTPException(status_code=404, detail="Game not found")

    # Mark the game as ended
    del games[game_id]

    print(":)")

    return {'message': f'Game {game_id} ended successfully', 'game_id': game_id}


@app.post('/loss')
async def loss(request: Request):
    data = await request.json()
    game_id = data.get('game_id')

    if not game_id:
        raise HTTPException(status_code=404, detail="Game not found")

    if game_id not in games:
        raise HTTPException(status_code=404, detail="Game not found")

    # Mark the game as ended
    del games[game_id]

    print(":(")

    return {'message': f'Game {game_id} ended successfully', 'game_id': game_id}

if __name__ == '__main__':
    # Argument parser for handling the port input
    parser = argparse.ArgumentParser(description="Tank Game AI")
    parser.add_argument('--port', type=int, default=5000, help='Port to run the server on')
    
    # Parse the arguments
    args = parser.parse_args()
    
    # Run the app on the specified port
    uvicorn.run(app, host="0.0.0.0", port=args.port)