# Tank Game AI Template

A C# template for building an AI agent for `tank_game`. This setup provides endpoints to initialize, control, and clean up game logic.

## Prerequisites

- .NET 6.0 or newer

## Setup

Restore dependencies:
```
dotnet restore
```

## Running the Server

Start the server:
```
dotnet run
```

## API Endpoints

| Method | Endpoint      | Description                  |
|--------|---------------|------------------------------|
| POST   | `/start_game` | Initialize a game session    |
| POST   | `/brain`      | Main game logic for your AI  |
| POST   | `/win`        | Clean up on game win         |
| POST   | `/loss`       | Clean up on game loss        |
