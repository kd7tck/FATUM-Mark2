
class FengShuiVisualizer {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');

        this.img = null;
        this.gridType = 'grid'; // 'grid', 'pie'
        this.gridOpacity = 0.5;
        this.heatmap = null; // 3x3 array
        this.cures = []; // Array of {name, x, y} (x,y normalized 0-3)

        this.gridState = {
            x: 0,
            y: 0,
            scale: 200,
            rotation: 0
        };

        this.isDragging = false;
        this.lastMouse = { x: 0, y: 0 };
        this.selectedCure = null; // For dragging cures

        this.resize();
        this.attachListeners();
        this.render();
    }

    resize() {
        this.canvas.width = this.canvas.parentElement.offsetWidth;
        this.canvas.height = 400;
        this.gridState.x = this.canvas.width / 2;
        this.gridState.y = this.canvas.height / 2;
        this.render();
    }

    loadImage(src) {
        this.img = new Image();
        this.img.onload = () => { this.render(); };
        this.img.src = src;
    }

    setHeatmap(heatmapData) {
        this.heatmap = heatmapData;
        this.render();
    }

    addCure(name) {
        // Add to center of grid
        this.cures.push({ name: name, x: 1.5, y: 1.5 });
        this.render();
        this.notifyUpdate();
    }

    toggleGridType() {
        this.gridType = this.gridType === 'grid' ? 'pie' : 'grid';
        this.render();
    }

    setOpacity(val) {
        this.gridOpacity = parseFloat(val);
        this.render();
    }

    resetView() {
        this.gridState.x = this.canvas.width / 2;
        this.gridState.y = this.canvas.height / 2;
        this.gridState.scale = 200;
        this.gridState.rotation = 0;
        this.render();
    }

    attachListeners() {
        const c = this.canvas;

        c.addEventListener('mousedown', (e) => {
            const m = this.getMousePos(e);
            // Check if clicking a cure
            const gridM = this.toGridCoords(m.x, m.y);
            const clickedCure = this.cures.find(cure => {
                const dx = cure.x - gridM.x;
                const dy = cure.y - gridM.y;
                return (dx*dx + dy*dy) < 0.1; // Hitbox
            });

            if (clickedCure) {
                this.selectedCure = clickedCure;
            } else {
                this.isDragging = true;
            }
            this.lastMouse = m;
        });

        window.addEventListener('mouseup', () => {
            this.isDragging = false;
            if (this.selectedCure) {
                this.selectedCure = null;
                this.notifyUpdate();
            }
        });

        c.addEventListener('mousemove', (e) => {
            const m = this.getMousePos(e);

            if (this.selectedCure) {
                const gridM = this.toGridCoords(m.x, m.y);
                this.selectedCure.x = gridM.x;
                this.selectedCure.y = gridM.y;
                this.render();
            } else if (this.isDragging) {
                const dx = m.x - this.lastMouse.x;
                const dy = m.y - this.lastMouse.y;
                this.gridState.x += dx;
                this.gridState.y += dy;
                this.render();
            }
            this.lastMouse = m;
        });

        c.addEventListener('wheel', (e) => {
            e.preventDefault();
            if (e.shiftKey) {
                this.gridState.rotation += (e.deltaY > 0 ? 0.05 : -0.05);
            } else {
                this.gridState.scale *= (e.deltaY > 0 ? 0.9 : 1.1);
            }
            this.render();
        });
    }

    getMousePos(e) {
        const rect = this.canvas.getBoundingClientRect();
        return { x: e.clientX - rect.left, y: e.clientY - rect.top };
    }

    toGridCoords(sx, sy) {
        // Inverse transform
        const tx = sx - this.gridState.x;
        const ty = sy - this.gridState.y;
        const cos = Math.cos(-this.gridState.rotation);
        const sin = Math.sin(-this.gridState.rotation);
        const rx = tx * cos - ty * sin;
        const ry = tx * sin + ty * cos;
        const cellSize = this.gridState.scale / 3;
        // 0,0 is center of grid (1.5, 1.5 in grid coords)
        // grid coord = (rx / cellSize) + 1.5
        return {
            x: (rx / cellSize) + 1.5,
            y: (ry / cellSize) + 1.5
        };
    }

    notifyUpdate() {
        if(window.onCureUpdate) window.onCureUpdate(this.cures);
    }

    render() {
        const ctx = this.ctx;
        const w = this.canvas.width;
        const h = this.canvas.height;

        ctx.fillStyle = '#0a0a0a';
        ctx.fillRect(0, 0, w, h);

        if (this.img) {
            ctx.save();
            ctx.translate(w/2, h/2);
            const scale = Math.min(w/this.img.width, h/this.img.height) * 0.9;
            ctx.scale(scale, scale);
            ctx.drawImage(this.img, -this.img.width/2, -this.img.height/2);
            ctx.restore();
        } else {
            ctx.fillStyle = '#333';
            ctx.font = '14px monospace';
            ctx.textAlign = 'center';
            ctx.fillText("Upload Floorplan", w/2, h/2);
        }

        ctx.save();
        ctx.translate(this.gridState.x, this.gridState.y);
        ctx.rotate(this.gridState.rotation);

        const size = this.gridState.scale;

        // Render Heatmap (Interpolated)
        if (this.heatmap) {
            this.drawInterpolatedHeatmap(ctx, size);
        }

        // Grid Lines
        ctx.globalAlpha = this.gridOpacity;
        const cell = size / 3;
        const half = size / 2;
        ctx.strokeStyle = '#39ff14';
        ctx.lineWidth = 1;

        if (this.gridType === 'grid') {
            ctx.beginPath();
            for(let i=0; i<=3; i++) {
                const p = -half + i*cell;
                ctx.moveTo(p, -half);
                ctx.lineTo(p, half);
                ctx.moveTo(-half, p);
                ctx.lineTo(half, p);
            }
            ctx.stroke();
        } else {
            // Pie
             ctx.beginPath();
            ctx.arc(0, 0, half, 0, Math.PI * 2);
            ctx.stroke();
            for(let i=0; i<8; i++) {
                const a = i * Math.PI/4;
                ctx.moveTo(0,0);
                ctx.lineTo(Math.cos(a)*half, Math.sin(a)*half);
            }
            ctx.stroke();
        }

        // Cures
        ctx.globalAlpha = 1.0;
        this.cures.forEach(c => {
            const cx = (c.x - 1.5) * cell;
            const cy = (c.y - 1.5) * cell;
            ctx.fillStyle = 'gold';
            ctx.beginPath();
            ctx.arc(cx, cy, 5, 0, Math.PI*2);
            ctx.fill();
            ctx.fillStyle = 'white';
            ctx.font = '10px sans-serif';
            ctx.fillText(c.name, cx+8, cy);
        });

        // Facing Arrow
        const facingInput = document.getElementById('fsFacing');
        const facingDeg = facingInput ? parseFloat(facingInput.value) : 180;

        // North (Blue) - Relative to grid, based on input
        ctx.save();
        ctx.rotate((facingDeg - 180) * Math.PI / 180);
        ctx.strokeStyle = 'cyan';
        ctx.beginPath();
        ctx.moveTo(0, -half-10);
        ctx.lineTo(0, -half-30);
        ctx.stroke();
        ctx.fillText("N", -4, -half-35);
        ctx.restore();

        // Face (Green) - Bottom of grid
        ctx.strokeStyle = '#39ff14';
        ctx.beginPath();
        ctx.moveTo(0, half);
        ctx.lineTo(0, half+20);
        ctx.stroke();

        ctx.restore();
    }

    drawInterpolatedHeatmap(ctx, size) {
        // Draw 3x3 heatmap as a 9x9 smoothed grid or use canvas gradient?
        // Canvas gradient for 3x3 is hard.
        // We will draw small rects (resolution 10x10 per cell) and interpolate color.

        const cell = size / 3;
        const res = 10; // Subdivisions per cell
        const subSize = cell / res;
        const half = size / 2;

        const facingInput = document.getElementById('fsFacing');
        const facingDeg = facingInput ? parseFloat(facingInput.value) : 180;

        ctx.save();
        ctx.rotate(facingDeg * Math.PI / 180);

        for(let r=0; r<3; r++) {
            for(let c=0; c<3; c++) {
                // Neighbors for interpolation... simplified: just draw raw blocks with blur?
                // Or true bilinear.
                // Let's just draw the cells with high blur context setting.
                // Actually, just drawing the rects with opacity is "blocky".
                // User wants "Smooth".
                // Let's use a radial gradient at center of each cell?

                const val = this.heatmap[r][c];
                const x = (c - 1.5) * cell; // Center of cell is +0.5 cell
                const y = (r - 1.5) * cell;

                // Draw a radial gradient for this cell's influence
                const grad = ctx.createRadialGradient(x + cell/2, y + cell/2, 0, x + cell/2, y + cell/2, cell);
                grad.addColorStop(0, `rgba(57, 255, 20, ${val * 0.8})`);
                grad.addColorStop(1, `rgba(57, 255, 20, 0)`);

                ctx.fillStyle = grad;
                ctx.fillRect(x - cell/2, y - cell/2, cell*2, cell*2); // Overlap
            }
        }
        ctx.restore();
    }
}
