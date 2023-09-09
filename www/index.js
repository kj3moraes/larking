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

ctx.fillStyle = "rgb(0,0,0)";

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size) {
    this.beginPath();
    this.moveTo(x, y);
    this.lineTo(x + size, y + size);
    this.lineTo(x - size, y + size);
    this.lineTo(x, y);

    this.fillStyle = 'rgb(0, 0, 0)';
    this.fill();
};


for (const animal of animals) {
    ctx.drawTriangle(
        animal.x * viewWidth,
        animal.y * viewHeight,
        0.01 * viewWidth
    );
}


ctxt.drawTriangle(50, 0, 50);

