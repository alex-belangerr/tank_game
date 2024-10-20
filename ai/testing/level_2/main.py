import argparse
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
        'status': 'active',
        'last_pos': None  # Initialize last position for stuck detection
    }

    print(f"New game: {game_id}")

    return jsonify({'message': f'Game {game_id} started successfully', 'game_id': game_id}), 200

@app.route('/brain', methods=['POST'])
def brain():
    data = request.get_json()
    game_id = data.get('game_id')

    # Check if game_id exists
    if not game_id:
        return jsonify({'error': 'Game ID is required'}), 400
    if game_id not in games:
        return jsonify({'error': 'Game not found'}), 404

    # Extract hull vision data (8 rays), turret vision (5 rays), and position
    hull_vision = data.get('hull_vision', [])
    turret_vision = data.get('turret_vision', [])
    current_pos = data.get('pos')  # Current position of the tank

    # Initialize anti-stuck detection and movement tracking
    if 'stuck_counter' not in games[game_id]:
        games[game_id]['stuck_counter'] = 0
    if 'anti_stuck_active' not in games[game_id]:
        games[game_id]['anti_stuck_active'] = False
    if 'anti_stuck_iterations' not in games[game_id]:
        games[game_id]['anti_stuck_iterations'] = 0

    # Check if the tank has moved
    if games[game_id]['last_pos'] is not None:
        distance_moved = ((current_pos[0] - games[game_id]['last_pos'][0]) ** 2 +
                           (current_pos[1] - games[game_id]['last_pos'][1]) ** 2) ** 0.5
        if distance_moved < 5.0:  # Adjust threshold as needed
            games[game_id]['stuck_counter'] += 1
        else:
            games[game_id]['stuck_counter'] = 0
    games[game_id]['last_pos'] = current_pos  # Update last position

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
    left_wall = get_wall_distance(hull_vision[7])   # Front-left
    right_wall = get_wall_distance(hull_vision[1])  # Front-right

    # If stuck for more than a threshold (e.g., 5 iterations), activate anti-stuck mode
    if games[game_id]['stuck_counter'] > 5:
        games[game_id]['anti_stuck_active'] = True
        games[game_id]['stuck_counter'] = 0
        games[game_id]['anti_stuck_iterations'] = 3  # Set the number of iterations to perform anti-stuck actions

    # Handle anti-stuck behavior
    if games[game_id]['anti_stuck_active']:
        if games[game_id]['anti_stuck_iterations'] > 0:
            # Move backward
            action=""
            if games[game_id]['anti_stuck_iterations'] % 2 == 0:
                action = 'move_backward'
            else:
                if left_wall is None or left_wall >= wall_threshold:
                    action = 'rotate_left'
                elif right_wall is None or right_wall >= wall_threshold:
                    action = 'rotate_right'

            games[game_id]['anti_stuck_iterations'] -= 1  # Decrease iteration count

            print(f"Game {game_id} action: {action} (anti-stuck, iterations left: {games[game_id]['anti_stuck_iterations']})")
            return jsonify({'action': action}), 200

    # Counter to alternate between shooting and not shooting
    if 'shoot_counter' not in games[game_id]:
        games[game_id]['shoot_counter'] = 0

    # AI Logic for Movement (based on hull vision)
    if front_wall and front_wall < wall_threshold:
        # Wall detected directly in front
        if left_wall and left_wall < wall_threshold and right_wall and right_wall < wall_threshold:
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

    # AI Logic for Shooting (based on turret vision)
    turret_wall = False
    for ray in turret_vision:
        wall_distance = get_wall_distance(ray)
        if wall_distance is not None and wall_distance < 100.0:  # Wall within shooting range (100 units)
            turret_wall = True
            break

    # Shoot every second brain call and only if a wall is detected within turret vision
    games[game_id]['shoot_counter'] += 1
    if turret_wall and games[game_id]['shoot_counter'] % 2 == 0:
        action = 'shoot'

    print(f"Game {game_id} action: {action}")

    return jsonify({'action': action}), 200

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
    # Argument parser for handling the port input
    parser = argparse.ArgumentParser(description="Flask Tank Game AI")
    parser.add_argument('--port', type=int, default=5000, help='Port to run the server on')
    
    # Parse the arguments
    args = parser.parse_args()
    
    # Run the app on the specified port
    app.run(debug=True, port=args.port)