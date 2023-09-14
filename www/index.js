import * as sim from "lib-sim-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();
const animals = simulation.animals_present();
console.log(world);
console.log(animals)

// Initialize the arean
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

        this.stroke();
    };


for (const animal of animals) {
    ctx.drawTriangle(
        animal.x * viewWidth,
        animal.y * viewHeight,
        0.01 * viewWidth,
        animal.rotation
    );
    console.log(animal);
}
