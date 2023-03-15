import * as wasm from "langtons-ant";
import { Cell, Universe, Direction } from "langtons-ant";
import { memory } from "langtons-ant/langtons_ant_bg";

const CELL_SIZE = 25; // px
const GRID_COLOR = "#000000";
const WHITE = "#FFFFFF";
const BLACK = "#505050";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("langtons-ant-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    universe.move_ant();

    drawGrid();
    drawCells();
    drawAnt();
    setTimeout(renderLoop, 1000 / 2); // 2 frames per second
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col);

            ctx.fillStyle = cells[index] === Cell.White ? WHITE : BLACK;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

const drawAnt = () => {
    var x = universe.get_x();
    var y = universe.get_y();
    var direction = universe.get_direction();
    var img = new Image();

    if (direction == Direction.East) {
        img.src = "./images/ant_east.png";
    } else if (direction == Direction.West) {
        img.src = "./images/ant_west.png";
    } else if (direction == Direction.South) {
        img.src = "./images/ant_south.png";
    } else {
        img.src = "./images/ant_north.png";
    }
    img.onload = () => {
        ctx.drawImage(img, x * (CELL_SIZE + 1) + 1, y * (CELL_SIZE + 1) + 1, 25, 25);
    }
}

drawGrid();
drawCells();
drawAnt();
setTimeout(renderLoop, 1000 / 2); // 2 frames per second
