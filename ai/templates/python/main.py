import uvicorn
import argparse
from fastapi import FastAPI, Request

app = FastAPI()

# TODO - Initialize any game data

@app.post('/start_game')
async def start_game(request: Request):
    data = await request.json()
    
    # TODO - YOUR CODE GOES HERE

    return {'message': f''}

@app.post('/brain')
async def brain(request: Request):
    data = await request.json()
    
    # TODO - YOUR CODE GOES HERE

    return {'message': f''}

@app.post('/win')
async def win(request: Request):
    data = await request.json()
    
    # TODO - Clean up any variables

    return {'message': f''}

@app.post('/loss')
async def loss(request: Request):
    data = await request.json()
    
    # TODO - Clean up any variables

    return {'message': f''}

if __name__ == '__main__':
    # Argument parser for handling the port input
    parser = argparse.ArgumentParser(description="Tank Game AI")
    parser.add_argument('--port', type=int, default=5000, help='Port to run the server on')
    
    # Parse the arguments
    args = parser.parse_args()
    
    # Run the app on the specified port
    uvicorn.run(app, host="0.0.0.0", port=args.port)