from flask import Flask, request, jsonify

app = Flask(__name__)

# In-memory storage for game data (for simplicity)
games = {}

@app.route('/start_game', methods=['POST'])
def start_game():
    data = request.get_json()
    print(data)
    game_id = data.get('game_id')
    
    if not game_id:
        print("No game id")
        return jsonify({'error': 'Game ID is required'}), 400

    # Initialize game data
    games[game_id] = {
        'status': 'active'
    }

    print(f"New game: {game_id}")

    return jsonify({'message': f'Game {game_id} started successfully', 'game_id': game_id}), 200


@app.route('/brain', methods=['POST'])
def brain():
    data = request.get_json()
    game_id = data.get('game_id')

    # Request sensor data

    if not game_id:
        return jsonify({'error': 'Game ID is required'}), 400

    if game_id not in games:
        return jsonify({'error': 'Game not found'}), 404

    print(data)

    return jsonify({'action': 'spin_left'}), 200


@app.route('/win', methods=['POST'])
def win():
    data = request.get_json()
    game_id = data.get('game_id')

    if not game_id:
        return jsonify({'error': 'Game ID is required'}), 400

    if game_id not in games:
        return jsonify({'error': 'Game not found'}), 404

    # Mark the game as ended
    del games[game_id]

    print(":)")

    return jsonify({'message': f'Game {game_id} ended successfully', 'game_id': game_id}), 200

@app.route('/loss', methods=['POST'])
def loss():
    data = request.get_json()
    game_id = data.get('game_id')

    if not game_id:
        return jsonify({'error': 'Game ID is required'}), 400

    if game_id not in games:
        return jsonify({'error': 'Game not found'}), 404

    # Mark the game as ended
    del games[game_id]

    print(":(")

    return jsonify({'message': f'Game {game_id} ended successfully', 'game_id': game_id}), 200


if __name__ == '__main__':
    app.run(debug=True)
