using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.DependencyInjection;
using System.Text.Json;
using System.Threading.Tasks;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddEndpointsApiExplorer();

var app = builder.Build();

// Initialize any game data as needed
app.MapPost("/start_game", async (HttpContext context) =>
{
    var data = await JsonSerializer.DeserializeAsync<StartGameRequest>(context.Request.Body);

    // TODO - Your game initialization code here

    return Results.Ok(new { message = "" });
});

app.MapPost("/brain", async (HttpContext context) =>
{
    var data = await JsonSerializer.DeserializeAsync<BrainRequest>(context.Request.Body);

    // TODO - Implement AI decision-making logic based on the BrainRequest data

    return Results.Ok(new { action = "" });  // Example action
});

app.MapPost("/win", async (HttpContext context) =>
{
    var data = await JsonSerializer.DeserializeAsync<GameStatusRequest>(context.Request.Body);

    // TODO - Handle win condition cleanup

    return Results.Ok(new { message = "" });
});

app.MapPost("/loss", async (HttpContext context) =>
{
    var data = await JsonSerializer.DeserializeAsync<GameStatusRequest>(context.Request.Body);

    // TODO - Handle loss condition cleanup

    return Results.Ok(new { message = "" });
});

app.Run();

// Request data models for each endpoint
public class StartGameRequest
{
    public string GameId { get; set; }  // Unique identifier for the game instance
}

public class BrainRequest
{
    public string GameId { get; set; }
    public (float X, float Y) Pos { get; set; }
    public float Rot { get; set; }
    public float TurretRot { get; set; }
    public VisionData[] TurretVision { get; set; }
    public VisionData[] HullVision { get; set; }
}

public class VisionData
{
    public float? Wall { get; set; }
    public float? Enemy { get; set; }
}

public class GameStatusRequest
{
    public string GameId { get; set; }  // Game ID is optional here for win/loss tracking purposes
}
