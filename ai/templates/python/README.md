# Tank Game AI Template

A Python template for building an AI agent for `tank_game`. This setup provides endpoints to initialize, control, and clean up game logic.

## Prerequisites

- Python 3.11 or newer
- pip

## Setup

Install dependencies:
```
pip install -r requirements.txt
```

## Running the Server

Start the server on a specified port (e.g., 5000):

python main.py --port 5000

## API Endpoints

| Method | Endpoint      | Description                  |
|--------|---------------|------------------------------|
| POST   | `/start_game` | Initialize a game session    |
| POST   | `/brain`      | Main game logic for your AI  |
| POST   | `/win`        | Clean up on game win         |
| POST   | `/loss`       | Clean up on game loss        |