import { Chip8 } from "chip8";
import { memory } from "chip8/chip8_bg";

const BLACK = "#FFFFFF";
const WHITE = "#000000";
const PIXEL_SIZE = 10;

const canvas = document.getElementById("chip8-canvas");
const fileInput = document.getElementById("file-input");

const chip8 = Chip8.new();
const width = chip8.width();
const height = chip8.height();

const ctx = canvas.getContext("2d");
ctx.canvas.width = width * PIXEL_SIZE;
ctx.canvas.height = height * PIXEL_SIZE;

var audioContext = new AudioContext();
var o = audioContext.createOscillator();
var audioPlaying = false
o.type = "square";
o.start()

let started = false;

setInterval(() => {
    if (started) {
        chip8.step(); 
    }
}, 2);

setInterval(() => {
    if (started) {
        if (chip8.handle_timers()) {
            if (!audioPlaying) {
                audioPlaying = true;
                o.connect(audioContext.destination);
            }
        } else {
            if (audioPlaying) {
                audioPlaying = false;
                o.disconnect(audioContext.destination);
            }
        }
    }
}, 16);

const renderLoop = () => {
    if (started) {
        drawScreen();
    }
    requestAnimationFrame(renderLoop);
}

const getIndex = (row, col) => {
    return row * width + col;
}

const drawScreen = () => {
    const pixelsPtr = chip8.display();
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            let idx = getIndex(row, col);

            ctx.fillStyle = pixels[idx] ? BLACK : WHITE;

            ctx.fillRect(
                col * PIXEL_SIZE,
                row * PIXEL_SIZE,
                PIXEL_SIZE,
                PIXEL_SIZE
            );
        }
    }

    ctx.stroke();
}

const handleFile = (e) => {
    const file = e.currentTarget.files[0];
    const reader = new FileReader();
    reader.onload = (e) => {
        var contents = e.target.result;
        chip8.load_rom(contents);
        started = true;
    }
    reader.readAsBinaryString(file);
}

const getKey = (key) => {
    switch (key) {
        case '1': return 0;
        case '2': return 1;
        case '3': return 2;
        case '4': return 3;
        case 'q': return 4;
        case 'w': return 5;
        case 'e': return 6;
        case 'r': return 7;
        case 'a': return 8;
        case 's': return 9;
        case 'd': return 10;
        case 'f': return 11;
        case 'z': return 12;
        case 'x': return 13;
        case 'c': return 14;
        case 'v': return 15;
        default: return -1;
    }
}

document.addEventListener('keydown', (e) => {
    const key = getKey(e.key);
    if (key != -1) {
        chip8.key_pressed(key, true);
    }
})
document.addEventListener('keyup', (e) => {
    const key = getKey(e.key);
    if (key != -1) {
        chip8.key_pressed(key, false);
    }
})

fileInput.addEventListener('change', handleFile);

requestAnimationFrame(renderLoop);