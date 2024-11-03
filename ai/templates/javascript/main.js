const express = require('express');
const bodyParser = require('body-parser');
const { program } = require('commander');

const app = express();
const PORT = process.env.PORT || 5000;

// Middleware to parse JSON bodies
app.use(bodyParser.json());

// TODO - Initialize any game data

app.post('/start_game', (req, res) => {
    const data = req.body;

    // TODO - YOUR CODE GOES HERE

    res.json({ message: '' });
});

app.post('/brain', (req, res) => {
    const data = req.body;

    // TODO - YOUR CODE GOES HERE

    res.json({ action: '' });
});

app.post('/win', (req, res) => {
    const data = req.body;

    // TODO - Clean up any variables

    res.json({ message: '' });
});

app.post('/loss', (req, res) => {
    const data = req.body;

    // TODO - Clean up any variables

    res.json({ message: '' });
});

// Command-line argument parser
program
    .option('--port <number>', 'Port to run the server on', 5000)
    .parse(process.argv);

const port = program.opts().port;

// Start the server
app.listen(port, () => {
    console.log(`Server is running on http://0.0.0.0:${port}`);
});
