# Tank Game AI Template

A Node.js template for building an AI agent for `tank_game`. This setup provides endpoints to initialize, control, and clean up game logic.

## Prerequisites

- Node.js 14.x or newer
- npm (Node package manager)

## Setup

Install dependencies:

```
npm install
```

## Running the Server

Start the server on a specified port (e.g., 5000):

```
node main.js --port 5000
```

## API Endpoints

| Method | Endpoint      | Description                  |
|--------|---------------|------------------------------|
| POST   | `/start_game` | Initialize a game session    |
| POST   | `/brain`      | Main game logic for your AI  |
| POST   | `/win`        | Clean up on game win         |
| POST   | `/loss`       | Clean up on game loss        |

# Instructions
1. Set up your environment:
- Make sure you have Node.js installed.
- Create a new directory for your project and navigate into it.
2. Copy the provided files:
- Create main.js, package.json, and README.md files with the corresponding content provided above.
3. Install dependencies:
- Run npm install to install the necessary packages.
4. Run the server:
- Start your server with the command `node main.js --port 5000`.
