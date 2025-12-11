
class FengShuiVisualizer {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');

        // State
        this.img = null;
        this.gridType = 'grid'; // 'grid' or 'pie'
        this.gridOpacity = 0.5;
        this.heatmap = null; // 3x3 array if loaded

        // Transform State (Pan/Zoom of Grid relative to Canvas Center?)
        // Or Pan/Zoom of Image?
        // Let's implement: Canvas shows Viewport.
        // Image is at (imgX, imgY, imgScale, imgRot).
        // Grid is drawn on top.
        // WAIT: Requirement is to align 3x3 grid OVER the layout.
        // Usually, the layout image is the base.
        // The user moves the GRID to fit the HOUSE on the image.
        // So Image is static (centered initially), Grid is transformable.

        this.gridState = {
            x: 0,
            y: 0,
            scale: 200, // Size in pixels
            rotation: 0 // Radians
        };

        // Interaction State
        this.isDragging = false;
        this.lastMouse = { x: 0, y: 0 };
        this.mode = 'move'; // 'move'

        this.resize();
        this.attachListeners();
        this.render();
    }

    resize() {
        this.canvas.width = this.canvas.parentElement.offsetWidth;
        this.canvas.height = 400; // Fixed height from CSS
        // Center grid initially
        this.gridState.x = this.canvas.width / 2;
        this.gridState.y = this.canvas.height / 2;
        this.render();
    }

    loadImage(src) {
        this.img = new Image();
        this.img.onload = () => {
            // Auto fit grid size to image?
            // Maybe just center image.
            this.render();
        };
        this.img.src = src;
    }

    setHeatmap(heatmapData, facingDeg) {
        this.heatmap = heatmapData;
        // facingDeg is used to confirm orientation, but our visual grid defines orientation.
        // The heatmap data is strictly formatted [Row0=S, Row1=C, Row2=N].
        // We will draw it onto the grid cells.
        this.render();
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
            this.isDragging = true;
            this.lastMouse = { x: e.offsetX, y: e.offsetY };
        });

        window.addEventListener('mouseup', () => {
            this.isDragging = false;
        });

        c.addEventListener('mousemove', (e) => {
            if (!this.isDragging) return;
            const dx = e.offsetX - this.lastMouse.x;
            const dy = e.offsetY - this.lastMouse.y;

            this.gridState.x += dx;
            this.gridState.y += dy;

            this.lastMouse = { x: e.offsetX, y: e.offsetY };
            this.render();
        });

        c.addEventListener('wheel', (e) => {
            e.preventDefault();
            if (e.shiftKey) {
                // Rotate
                const delta = e.deltaY > 0 ? 0.05 : -0.05;
                this.gridState.rotation += delta;
            } else {
                // Scale
                const zoom = e.deltaY > 0 ? 0.9 : 1.1;
                this.gridState.scale *= zoom;
            }
            this.render();
        });
    }

    render() {
        const ctx = this.ctx;
        const w = this.canvas.width;
        const h = this.canvas.height;

        // Clear
        ctx.fillStyle = '#0a0a0a';
        ctx.fillRect(0, 0, w, h);

        // Draw Image (Centered, Scaled to fit?)
        // Let's draw image centered at canvas center, scaled to fit height
        if (this.img) {
            ctx.save();
            ctx.translate(w/2, h/2);
            // Optional: User could transform image instead of grid.
            // Requirement says "align a 3x3 grid over it".
            // So Image is static background.
            const scale = Math.min(w / this.img.width, h / this.img.height) * 0.9;
            ctx.scale(scale, scale);
            ctx.drawImage(this.img, -this.img.width/2, -this.img.height/2);
            ctx.restore();
        } else {
            ctx.fillStyle = '#222';
            ctx.font = '14px Courier New';
            ctx.textAlign = 'center';
            ctx.fillText("Upload Floorplan to Begin", w/2, h/2);
        }

        // Draw Grid
        ctx.save();
        ctx.translate(this.gridState.x, this.gridState.y);
        ctx.rotate(this.gridState.rotation);

        const size = this.gridState.scale;
        const half = size / 2;

        ctx.globalAlpha = this.gridOpacity;

        if (this.gridType === 'grid') {
            this.drawSquareGrid(ctx, size);
        } else {
            this.drawPieGrid(ctx, size);
        }

        // Draw Facing Indicator (Arrow pointing DOWN relative to grid)
        // Standard convention: "Facing" is usually bottom of page or specific side.
        // We will treat the BOTTOM side of the grid (Row 2 end?) as "Facing"?
        // Wait, earlier logic: "South Up".
        // In Luo Shu, South (Top) is usually Facing if House Faces South.
        // Let's assume the user aligns the grid such that the "Facing" side of the house
        // corresponds to the "Facing" side of the Grid.
        // We will define the "Facing" side of the Grid as the BOTTOM for intuitive "Entry".
        // But backend data says Row 0 is South.

        // Let's draw a Green Arrow pointing out from the BOTTOM of the Grid to indicate "Face".
        // And a Blue Arrow pointing out from the TOP of the Grid to indicate "Sitting".
        // This helps user align: "Point Green Arrow to Front Door".

        ctx.globalAlpha = 1.0;

        // Facing Arrow (Bottom)
        ctx.beginPath();
        ctx.strokeStyle = '#39ff14'; // Neon Green
        ctx.lineWidth = 3;
        ctx.moveTo(0, half);
        ctx.lineTo(0, half + 30);
        ctx.lineTo(-5, half + 20);
        ctx.moveTo(0, half + 30);
        ctx.lineTo(5, half + 20);
        ctx.stroke();

        // North Arrow indicator (Blue)
        // Usually North is opposite Facing (South).
        // But the house might face East.
        // The Grid itself is just a template.
        // If the user aligns the grid, they are assigning sectors to space.
        // If the House Faces South, then Green Arrow (Face) = South.
        // If House Faces North, Green Arrow = North.
        // The USER input "Facing Degrees" tells us what the Green Arrow points to.

        // So, we just label the Green Arrow "Facing".
        // We calculate where "North" is based on the input degrees.

        // Ex: Facing = 180 (S). Green Arrow = S. North is Opposite (Top).
        // Ex: Facing = 90 (E). Green Arrow = E. North is Left (-90 deg).

        // We need to fetch the current Facing Input from the DOM to draw the North Arrow correctly.
        const facingInput = document.getElementById('fsFacing');
        const facingDeg = facingInput ? parseFloat(facingInput.value) : 180;

        // Calculate North relative to Grid orientation
        // Green Arrow is at 90 deg (Down) visually relative to Grid Center?
        // Let's say Grid Down is Angle 0 relative to "Face".
        // North is (Facing - 0) ? No.
        // North is (Facing + 180) degrees away from Face.
        // Visually: If Face (Bottom) is South (180), then North (0) is Top.
        // Angle difference = 180.
        // If Face (Bottom) is East (90), then North (0) is Left.

        // Rotation of North relative to Facing Vector:
        // NorthAngle = FaceAngle - FacingDeg?
        // Let's visualize:
        // Face is Vector (0, 1) [Down].
        // If Facing=180 (S), North should be Vector (0, -1) [Up].
        // If Facing=90 (E), North should be Vector (-1, 0) [Left].

        // Canvas rotation for North Arrow:
        // We are already rotated by gridState.rotation.
        // We want to draw North Arrow relative to Grid.
        // The Angle of "North" relative to "Face" (Bottom/Down) is:
        // -FacingDeg.
        // e.g. If Facing 180, North is -180 (Up). Correct.
        // If Facing 90, North is -90 (Right? No, Left).
        // Wait, standard compass: N=0, E=90, S=180, W=270.
        // If Face=90 (E), then N=0 is -90 degrees relative to E.
        // So yes, Rotate -FacingDeg relative to the Face Vector (Down).

        ctx.save();
        ctx.rotate((facingDeg - 180) * Math.PI / 180); // Adjust so 180 is Up?
        // Let's try: Facing 180. (180-180)=0. No rotation.
        // If we draw arrow pointing UP, it stays UP.
        // But Face is DOWN. So North is UP. Correct.

        // Facing 90 (E). (90-180) = -90. Rotate -90 (CCW). Arrow points Left.
        // If Face is Down (E), Left is North. Correct (N -> E is 90 deg CW).

        // So drawing an arrow pointing UP (relative to unrotated context, which is grid space)
        // works if we rotate by (Facing - 180).

        // Draw North Arrow (Blue)
        ctx.beginPath();
        ctx.strokeStyle = '#00ffff';
        ctx.lineWidth = 2;
        ctx.moveTo(0, -half - 10); // Start above grid
        ctx.lineTo(0, -half - 40);
        ctx.lineTo(-5, -half - 30);
        ctx.moveTo(0, -half - 40);
        ctx.lineTo(5, -half - 30);
        ctx.stroke();
        ctx.fillStyle = '#00ffff';
        ctx.fillText("N", -5, -half - 45);

        ctx.restore(); // Pop North rotation

        ctx.restore(); // Pop Grid Transform
    }

    drawSquareGrid(ctx, size) {
        const cell = size / 3;
        const half = size / 2;

        ctx.strokeStyle = '#39ff14';
        ctx.lineWidth = 1;
        ctx.beginPath();

        // Grid Lines
        for (let i = 0; i <= 3; i++) {
            const p = -half + i * cell;
            ctx.moveTo(p, -half);
            ctx.lineTo(p, half);
            ctx.moveTo(-half, p);
            ctx.lineTo(half, p);
        }
        ctx.stroke();

        // Draw Heatmap if exists
        if (this.heatmap) {
             // Heatmap mapping:
             // Row 0: Top (-half to -half+cell)
             // Row 2: Bottom

             // Wait.
             // If Facing (Bottom) is South (180).
             // Then Top is North.
             // Backend Heatmap: Row 2 is North.
             // So Row 2 should be at Top?
             // NO.
             // My previous logic: "South Up".
             // If South is Up (Top), then Row 0 (S) is Top.
             // Facing (Bottom) is North?
             // That contradicts "Green Arrow (Face) = South".

             // Let's Resolve:
             // Backend Heatmap is "South Up" (Row 0 = S, Row 2 = N).
             // If User Inputs Facing = 180 (S).
             // Green Arrow (Bottom) = S.
             // So Grid Bottom = S.
             // Grid Top = N.
             // So we must draw Row 2 (N) at Top?
             // No, Grid Top is visually Top.
             // If Grid Bottom is S, Grid Top is N.
             // Backend Row 2 is N. So Backend Row 2 goes to Grid Top.
             // Backend Row 0 is S. So Backend Row 0 goes to Grid Bottom.

             // BUT, what if Facing = 0 (N)?
             // Green Arrow (Bottom) = N.
             // Grid Bottom = N.
             // Grid Top = S.
             // Backend Row 2 (N) goes to Grid Bottom.
             // Backend Row 0 (S) goes to Grid Top.

             // CONCLUSION:
             // The Heatmap orientation depends on the Facing Degree!
             // Standard Luo Shu is South-Up (S at Top).
             // If the House Faces South (180), then the "Front" (Bottom of Grid) is South.
             // That means South is Down.
             // So the Chart is inverted relative to standard Luo Shu?

             // Let's simply rotate the Heatmap based on the Facing Degree.
             // The Heatmap is inherently: [0]=S, [1]=C, [2]=N.
             // Index 0 (S) must point to "South" direction.
             // Where is South?
             // We calculated North Vector earlier. South is Opposite.

             // Let's draw the Heatmap cells in a local coordinate system where Up is South.
             // Then rotate that whole system to align "South" with the actual South vector.

             ctx.save();
             // Standard Heatmap: Up is South.
             // We want "Up" (South) to point to... South.
             // North Arrow rotation was (Facing - 180).
             // So South Arrow rotation is (Facing).
             // Actually: North Arrow points to North.
             // If we rotate context so +Y is South...

             // Let's simpler approach:
             // Calculate rotation offset for the heatmap.
             // Base Heatmap: Row 0 is South.
             // If Facing=180 (S), Bottom is S. Top is N.
             // So Row 0 (S) should be at Bottom.
             // Row 2 (N) should be at Top.
             // Visually, Row 0 is normally drawn at Top (-y).
             // So we need to rotate the Heatmap drawing 180 deg?
             // Yes.

             // If Facing=0 (N). Bottom is N. Top is S.
             // Row 0 (S) should be at Top.
             // Row 0 is normally drawn at Top.
             // So 0 deg rotation.

             // Formula: Rotation = (Facing) deg?
             // Test: Facing 180 -> Rotate 180. Correct.
             // Test: Facing 0 -> Rotate 0. Correct.
             // Test: Facing 90 (E). Bottom is E. Left is N. Right is S.
             // Row 0 (S) should be at Right.
             // Standard Row 0 is Top.
             // Rotate Top to Right = +90 deg.
             // Formula: 90. Correct.

             const facingInput = document.getElementById('fsFacing');
             const facingDeg = facingInput ? parseFloat(facingInput.value) : 180;

             ctx.rotate(facingDeg * Math.PI / 180);

             // Draw 3x3 Heatmap (Assumed South-Up in data)
             for(let r=0; r<3; r++) {
                 for(let c=0; c<3; c++) {
                     const val = this.heatmap[r][c]; // Density
                     // Draw rect
                     // Grid Coords:
                     // r=0 -> Top (-1.5 to -0.5)
                     // r=1 -> Mid (-0.5 to 0.5)
                     // r=2 -> Bot (0.5 to 1.5)

                     // We need to center it.
                     const x = (c - 1.5) * cell; // c=0 -> -1.5, c=1 -> -0.5
                     // wait c=0 is left. c=2 is right.
                     // x = (c - 1) * cell - cell/2 ?
                     // c=0 -> -cell. -cell/2 = -1.5 cell. Correct.
                     const y = (r - 1.5) * cell;

                     const alpha = Math.min(0.8, val * 0.8); // Scale density
                     ctx.fillStyle = `rgba(57, 255, 20, ${alpha})`; // Neon Green
                     ctx.fillRect(x + cell * 0.5, y + cell * 0.5, cell, cell);
                     // Correction: x/y calculation above matches left/top of cell relative to center?
                     // (c-1.5)*cell is left edge.
                     // (c-1)*cell is center?
                     // center is (c-1)*cell.
                     // left is (c-1.5)*cell.
                     // fillRect takes x,y,w,h.
                     // Correct.
                 }
             }
             ctx.restore();
        }
    }

    drawPieGrid(ctx, size) {
        const half = size / 2;
        ctx.strokeStyle = '#ff00ff';
        ctx.lineWidth = 1;

        ctx.beginPath();
        ctx.arc(0, 0, half, 0, Math.PI * 2);
        ctx.stroke();

        // 8 Sectors
        for(let i=0; i<8; i++) {
            const angle = i * (Math.PI / 4);
            ctx.beginPath();
            ctx.moveTo(0, 0);
            ctx.lineTo(Math.cos(angle)*half, Math.sin(angle)*half);
            ctx.stroke();
        }

        // If heatmap, draw sectors?
        // Pie chart heatmap is trickier to map 3x3 to 8 slices.
        // Simplified: Center is separate. 8 directions map to 8 outer cells.
        // We will skip detailed heatmap for Pie mode in this iteration or approx it.
        if (this.heatmap) {
             const facingInput = document.getElementById('fsFacing');
             const facingDeg = facingInput ? parseFloat(facingInput.value) : 180;
             ctx.save();
             ctx.rotate(facingDeg * Math.PI / 180);

             // Map:
             // [0][0] SE -> Angle?
             // South is Up (-y) in this rotated view.
             // SE is Top-Left? No, East is Left (in South-Up).
             // Standard Compass (North Up): E is Right. SE is Bot-Right.
             // South Up: E is Left. SE is Top-Left.
             // Angles in Canvas (0 is Right, CW):
             // Top (-PI/2). Left (PI).
             // South (Top) -> -PI/2.
             // SE -> -3*PI/4.
             // SW -> -PI/4.
             // E -> PI (or -PI).
             // W -> 0.
             // N -> PI/2.
             // NE -> 3*PI/4.
             // NW -> PI/4.

             // Mapping indices to pie slices
             const map = [
                 { r:0, c:0, start: -Math.PI, end: -Math.PI/2 }, // SE (Approx)
                 // This is getting complex. Let's just fallback to grid render or skip.
                 // "yes to all" included Pie chart option.
                 // I will draw simple colored arcs.
             ];
             // Let's implement simpler: Just draw the 8 direction cells.
             // S (0,1), N (2,1), E (1,0), W (1,2)...
             // S is Top (-PI/2).
             this.drawPieSector(ctx, this.heatmap[0][1], -Math.PI*0.625, -Math.PI*0.375, half); // S
             this.drawPieSector(ctx, this.heatmap[0][2], -Math.PI*0.375, -Math.PI*0.125, half); // SW
             this.drawPieSector(ctx, this.heatmap[1][2], -Math.PI*0.125, Math.PI*0.125, half);  // W
             this.drawPieSector(ctx, this.heatmap[2][2], Math.PI*0.125, Math.PI*0.375, half);   // NW
             this.drawPieSector(ctx, this.heatmap[2][1], Math.PI*0.375, Math.PI*0.625, half);   // N
             this.drawPieSector(ctx, this.heatmap[2][0], Math.PI*0.625, Math.PI*0.875, half);   // NE
             this.drawPieSector(ctx, this.heatmap[1][0], Math.PI*0.875, Math.PI*1.125, half);   // E
             this.drawPieSector(ctx, this.heatmap[0][0], Math.PI*1.125, Math.PI*1.375, half);   // SE

             // Center
             ctx.beginPath();
             ctx.arc(0, 0, half/3, 0, Math.PI*2);
             ctx.fillStyle = `rgba(57, 255, 20, ${this.heatmap[1][1]})`;
             ctx.fill();

             ctx.restore();
        }
    }

    drawPieSector(ctx, val, start, end, radius) {
        ctx.beginPath();
        ctx.moveTo(0,0);
        ctx.arc(0, 0, radius, start, end);
        ctx.fillStyle = `rgba(57, 255, 20, ${Math.min(0.8, val*0.8)})`;
        ctx.fill();
    }
}
