# Tank Game AI Level 2

This ai just sits in place and will shoot at the first enemy it sees.

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

```
python main.py --port 5000
```

## API Endpoints

| Method | Endpoint      | Description                  |
|--------|---------------|------------------------------|
| POST   | `/start_game` | Initialize a game session    |
| POST   | `/brain`      | Main game logic for your AI  |
| POST   | `/win`        | Clean up on game win         |
| POST   | `/loss`       | Clean up on game loss        |