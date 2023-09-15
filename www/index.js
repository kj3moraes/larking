import * as sim from "lib-sim-wasm";

const simulation = new sim.Simulation();

// Initialize the arena
const viewport = document.getElementById("arena");
const viewportScale = window.devicePixelRatio || 1;
const viewWidth = viewport.width * viewportScale;
const viewHeight = viewport.height * viewportScale;
viewport.width = viewWidth;
viewport.height = viewHeight;
viewport.style.width = viewWidth + 'px';
viewport.style.height = viewHeight + 'px';

const ctx = viewport.getContext("2d");
ctx.scale(viewportScale, viewportScale);
ctx.lineWidth = 0.001 * viewWidth;
ctx.fillStyle = "rgb(0,0,0)";

// Draw all the birds as triangles.
CanvasRenderingContext2D.prototype.drawTriangle = 
    function (x, y, size, rotation) {
        this.beginPath();
        this.moveTo(
            x - Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5
        );
        this.lineTo(
            x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        );
        this.lineTo(
            x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        );
        this.lineTo(
            x - Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5
        );

        this.fillStyle = 'rgb(255, 128, 128)';
        this.fill();
    };

CanvasRenderingContext2D.prototype.drawCircle =
    function(x, y, radius) {
        this.beginPath();

        this.arc(x, y, radius, 0, 2.0 * Math.PI);
        this.fillStyle = 'rgb(0, 255, 0)';
        this.fill();
    };

function redraw() {

    ctx.clearRect(0, 0, viewWidth, viewHeight);
    simulation.step();

    const world = simulation.world();
    console.log(world);

    // Draw the food
    for (const food of world.food) {
        ctx.drawCircle(
            food.x * viewWidth,
            food.y * viewHeight,
            0.003 * viewWidth);
    }

    // Draw the animals
    for (const animal of world.animals) {

        ctx.drawTriangle(
            animal.x * viewWidth,
            animal.y * viewHeight,
            0.01 * viewWidth,
            animal.rotation
        );
        console.log(animal);
    }

    requestAnimationFrame(redraw);
}
redraw();
