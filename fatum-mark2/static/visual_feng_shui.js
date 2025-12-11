// GLOBAL STATE
let currentReport = null;
let currentHexagram = null;

// TABS
function showTab(tabId) {
    document.querySelectorAll('.tab-content').forEach(el => el.style.display = 'none');
    document.querySelectorAll('.nav-btn').forEach(el => el.classList.remove('active'));
    document.getElementById(`tab-${tabId}`).style.display = 'flex'; // or block based on layout
    document.querySelector(`button[onclick="showTab('${tabId}')"]`).classList.add('active');

    if (tabId === 'profiles') loadProfiles();
    if (tabId === 'history') loadHistory();
}

// === PROFILES ===
async function createProfile() {
    const data = {
        name: document.getElementById('p-name').value,
        birth_year: parseInt(document.getElementById('p-year').value),
        birth_month: parseInt(document.getElementById('p-month').value),
        birth_day: parseInt(document.getElementById('p-day').value),
        birth_hour: parseInt(document.getElementById('p-hour').value),
        gender: document.getElementById('p-gender').value
    };

    const res = await fetch('/api/profiles', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data)
    });
    const json = await res.json();
    if (json.id) {
        alert('Entity Registered: ID ' + json.id);
        loadProfiles();
    } else {
        alert('Error: ' + json.error);
    }
}

async function loadProfiles() {
    const res = await fetch('/api/profiles');
    const profiles = await res.json();
    const list = document.getElementById('profile-list');
    const select = document.getElementById('fs-profile-select');

    list.innerHTML = '';
    select.innerHTML = '<option value="">-- Manual Input --</option>';

    profiles.forEach(p => {
        // Add to list
        const card = document.createElement('div');
        card.className = 'card';
        card.innerHTML = `<h4>${p.name}</h4><p>${p.gender} | ${p.birth_year}-${p.birth_month}-${p.birth_day}</p>`;
        list.appendChild(card);

        // Add to select
        const opt = document.createElement('option');
        opt.value = JSON.stringify(p);
        opt.textContent = p.name;
        select.appendChild(opt);
    });
}

function loadProfileIntoForm() {
    const val = document.getElementById('fs-profile-select').value;
    if (!val) return;
    const p = JSON.parse(val);
    // Auto-fill logic could go here if we had fields for birth info in FS tab
    // Currently FS tab only has construction params, assumes birth info is passed in req
    // We will store the selected profile to merge into request
}

// === SVG HELPER FUNCTIONS ===
const NS = "http://www.w3.org/2000/svg";

function createSVG(w, h) {
    const svg = document.createElementNS(NS, "svg");
    svg.setAttribute("width", "100%");
    svg.setAttribute("height", "100%");
    svg.setAttribute("viewBox", `0 0 ${w} ${h}`);
    return svg;
}

function createRect(x, y, w, h, fill, stroke) {
    const rect = document.createElementNS(NS, "rect");
    rect.setAttribute("x", x);
    rect.setAttribute("y", y);
    rect.setAttribute("width", w);
    rect.setAttribute("height", h);
    if (fill) rect.setAttribute("fill", fill);
    if (stroke) {
        rect.setAttribute("stroke", stroke);
        rect.setAttribute("stroke-width", "2");
    }
    return rect;
}

function createText(x, y, text, fontSize, fill, className) {
    const txt = document.createElementNS(NS, "text");
    txt.setAttribute("x", x);
    txt.setAttribute("y", y);
    txt.textContent = text;
    txt.setAttribute("fill", fill || "#fff");
    txt.setAttribute("font-size", fontSize);
    if (className) txt.setAttribute("class", className);
    return txt;
}

function createLine(x1, y1, x2, y2, stroke) {
    const line = document.createElementNS(NS, "line");
    line.setAttribute("x1", x1);
    line.setAttribute("y1", y1);
    line.setAttribute("x2", x2);
    line.setAttribute("y2", y2);
    line.setAttribute("stroke", stroke);
    line.setAttribute("stroke-width", "2");
    return line;
}

function getElementColor(char) {
    // Basic mapping of Heavenly Stems / Earthly Branches to Elements
    // This is a simplified lookup. Ideally backend provides element data.
    // Wood: Jia, Yi, Yin, Mao
    // Fire: Bing, Ding, Si, Wu
    // Earth: Wu, Ji, Chen, Xu, Chou, Wei
    // Metal: Geng, Xin, Shen, You
    // Water: Ren, Gui, Hai, Zi

    const wood = ["Jia", "Yi", "Yin", "Mao"];
    const fire = ["Bing", "Ding", "Si", "Wu"];
    const earth = ["Wu", "Ji", "Chen", "Xu", "Chou", "Wei"];
    const metal = ["Geng", "Xin", "Shen", "You"];
    const water = ["Ren", "Gui", "Hai", "Zi"];

    if (wood.includes(char)) return "var(--wood)";
    if (fire.includes(char)) return "var(--fire)";
    if (earth.includes(char)) return "var(--earth)";
    if (metal.includes(char)) return "var(--metal)";
    if (water.includes(char)) return "var(--water)";
    return "#fff";
}

// === FENG SHUI ===

async function runFengShui() {
    const profileVal = document.getElementById('fs-profile-select').value;
    const profile = profileVal ? JSON.parse(profileVal) : null;

    const req = {
        construction_year: parseInt(document.getElementById('fs-year').value),
        facing_degrees: parseFloat(document.getElementById('fs-facing').value),
        intention: document.getElementById('fs-intention').value,
        quantum_mode: document.getElementById('fs-quantum').checked,
        virtual_cures: window.virtualCures || []
    };

    if (profile) {
        req.birth_year = profile.birth_year;
        req.birth_month = profile.birth_month;
        req.birth_day = profile.birth_day;
        req.birth_hour = profile.birth_hour;
        req.gender = profile.gender;
    }

    const res = await fetch('/api/tools/fengshui', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(req)
    });

    currentReport = await res.json();
    renderFengShuiOutput(currentReport);
}

function renderFengShuiOutput(report) {
    const out = document.getElementById('fs-output');

    let txt = `=== QUANTUM FENG SHUI ANALYSIS ===\n`;
    txt += `Period: ${report.annual_chart.period}\n`;
    txt += `Formation: ${report.formations.join(', ') || "None"}\n`;

    if (report.bazi) {
        // Render SVG BaZi
        renderBaZiSVG(report.bazi);
        if (report.bazi.quantum_flux) txt += `\n[BAZI QUANTUM FLUX]\n>> ${report.bazi.quantum_flux}\n`;
        if (report.bazi.alternate_pillars) txt += `>> ${report.bazi.alternate_pillars.join('\n>> ')}\n`;
    } else {
        document.getElementById('bazi-svg-container').innerHTML = '';
    }

    if (report.quantum) {
        txt += `\n[QUANTUM FIELD]\nFocus: ${report.quantum.focus_sector}\nVolatility: ${report.quantum.volatility_index}\n`;
        if (report.quantum.cure_efficacy) txt += `Cure Efficacy: ${(report.quantum.cure_efficacy * 100).toFixed(1)}%\n`;
    }

    if (report.qimen) {
        const qm = report.qimen;
        txt += `\n[QI MEN DUN JIA]\nTerm: ${qm.solar_term} (${qm.dun_type} Ju ${qm.ju_number})\n`;
        txt += `Structure: ${qm.palaces[0].structure}\n`; // Just grab first for summary
    }

    out.innerText = txt;

    renderFengShuiSVG(report);
}

function renderFengShuiSVG(report) {
    const container = document.getElementById('fs-svg-container');
    container.innerHTML = '';

    // Width/Height logic: container is 600x600 via CSS
    const w = 600;
    const h = 600;
    const svg = createSVG(w, h);

    // Draw 3x3 Grid
    const cw = w / 3;
    const ch = h / 3;
    const gridColor = "var(--primary)";

    // Outer Border
    svg.appendChild(createRect(0, 0, w, h, "none", gridColor));

    // Inner Lines
    for (let i = 1; i < 3; i++) {
        svg.appendChild(createLine(i * cw, 0, i * cw, h, gridColor));
        svg.appendChild(createLine(0, i * ch, w, i * ch, gridColor));
    }

    // Stars Mapping
    // SE | S | SW  -> (0,0) (1,0) (2,0)
    // E  | C | W   -> (0,1) (1,1) (2,1)
    // NE | N | NW  -> (0,2) (1,2) (2,2)
    const posMap = {
        "SE": [0,0], "S": [1,0], "SW": [2,0],
        "E": [0,1], "Center": [1,1], "W": [2,1],
        "NE": [0,2], "N": [1,2], "NW": [2,2]
    };

    report.annual_chart.palaces.forEach((p, idx) => {
        const [gx, gy] = posMap[p.sector] || [1,1];
        const x = gx * cw;
        const y = gy * ch;
        const cx = x + cw/2;
        const cy = y + ch/2;

        const group = document.createElementNS(NS, "g");
        group.setAttribute("class", "anim-fade-in");
        group.style.animationDelay = `${idx * 0.1}s`;

        // Sector Name
        const secText = createText(x + 10, y + 25, p.sector, "16", "var(--primary)");
        secText.setAttribute("class", "anim-pulse");
        group.appendChild(secText);

        // Mountain Star (Top Left)
        group.appendChild(createText(x + 20, y + 60, p.mountain_star, "24", "#ccc", "star-text"));

        // Water Star (Top Right)
        group.appendChild(createText(x + cw - 40, y + 60, p.water_star, "24", "var(--secondary)", "star-text"));

        // Base Star (Center)
        group.appendChild(createText(cx - 10, cy + 10, p.base_star, "32", "var(--accent)", "star-text"));

        // Visiting Star (Bottom Right)
        group.appendChild(createText(x + cw - 30, y + ch - 20, p.visiting_star, "18", "#ffff00", "star-text"));

        // Virtual Cures visualization
        // (Handled by redrawing if needed, logic below in drag handler)
        window.virtualCures.forEach(cure => {
            // Check if cure is in this sector
            // Cure x,y are grid coords (0-3)
            if (cure.x >= gx && cure.x < gx+1 && cure.y >= gy && cure.y < gy+1) {
               // Render cure symbol
               const cureGroup = document.createElementNS(NS, "g");
               const cxOffset = (cure.x - gx) * cw;
               const cyOffset = (cure.y - gy) * ch;
               // Actually cure x/y from drag are likely exact coordinates normalized to 0-3
               // Let's assume they are.
               const absX = cure.x * cw; // 0-3 * 200 = 0-600
               const absY = cure.y * ch;

               // But we are inside a loop iterating sectors.
               // Easier to draw all cures AFTER the loop in the main SVG space.
            }
        });

        svg.appendChild(group);
    });

    // Draw Cures Overlay
    window.virtualCures.forEach(cure => {
        const cx = cure.x * cw;
        const cy = cure.y * ch;
        // Simple circle for now
        const circle = document.createElementNS(NS, "circle");
        circle.setAttribute("cx", cx);
        circle.setAttribute("cy", cy);
        circle.setAttribute("r", 15);

        let color = "#fff";
        if (cure.name === "Fire") color = "var(--fire)";
        if (cure.name === "Water") color = "var(--water)";
        if (cure.name === "Wood") color = "var(--wood)";
        if (cure.name === "Metal") color = "var(--metal)";
        if (cure.name === "Earth") color = "var(--earth)";

        circle.setAttribute("fill", color);
        circle.setAttribute("stroke", "#fff");
        circle.setAttribute("class", "anim-pulse");
        svg.appendChild(circle);
    });

    container.appendChild(svg);
}

function renderBaZiSVG(bazi) {
    const container = document.getElementById('bazi-svg-container');
    container.innerHTML = '';
    const w = 600;
    const h = 200;
    const svg = createSVG(w, h);

    // 4 Pillars: Year, Month, Day, Hour
    // Each takes 1/4 width
    const pw = w / 4;
    const pillars = [
        { name: "YEAR", data: bazi.year_pillar },
        { name: "MONTH", data: bazi.month_pillar },
        { name: "DAY", data: bazi.day_pillar },
        { name: "HOUR", data: bazi.hour_pillar }
    ];

    pillars.forEach((p, i) => {
        const x = i * pw;
        const cx = x + pw / 2;

        const group = document.createElementNS(NS, "g");
        group.setAttribute("class", "anim-slide-up");
        group.style.animationDelay = `${i * 0.15}s`;

        // Box border
        group.appendChild(createRect(x + 5, 5, pw - 10, h - 10, "rgba(255,255,255,0.05)", "var(--grid-line)"));

        // Header
        const title = createText(cx, 30, p.name, "18", "var(--secondary)");
        title.setAttribute("text-anchor", "middle");
        group.appendChild(title);

        if (p.data) {
            // Split "Stem Branch" string. Assuming format "Stem Branch" or "StemBranch"
            // The backend usually returns "Wood Rat" (Element Animal) or "Jia Zi" (Pinyin).
            // Let's assume the string contains two parts.
            // If the backend returns "Yang Wood Rat", it might be longer.
            // Let's try to split by space.
            const parts = p.data.split(' ');
            const stem = parts[0] || "?";
            const branch = parts.slice(1).join(' ') || "?";

            // Stem (Top)
            const stemColor = getElementColor(stem);
            const stemTxt = createText(cx, 80, stem, "24", stemColor);
            stemTxt.setAttribute("text-anchor", "middle");
            stemTxt.setAttribute("font-weight", "bold");
            group.appendChild(stemTxt);

            // Branch (Bottom)
            const branchColor = getElementColor(branch); // This might need mapping animal -> element
            const branchTxt = createText(cx, 130, branch, "24", branchColor);
            branchTxt.setAttribute("text-anchor", "middle");
            branchTxt.setAttribute("font-weight", "bold");
            group.appendChild(branchTxt);

            // Element Labels (optional, small)
        }

        svg.appendChild(group);
    });

    container.appendChild(svg);
}

// === DIVINATION ===

async function castHexagram() {
    const res = await fetch('/api/tools/divination', { method: 'POST' });
    currentHexagram = await res.json();

    const out = document.getElementById('divination-text');
    out.innerHTML = `<h3>Hexagram ${currentHexagram.number}: ${currentHexagram.name}</h3>
    <p><strong>Judgment:</strong> ${currentHexagram.judgment}</p>
    <p><strong>Image:</strong> ${currentHexagram.image}</p>`;

    if (currentHexagram.transformed_hexagram) {
        out.innerHTML += `<hr><h3>Transformed to: Hexagram ${currentHexagram.transformed_hexagram.number}: ${currentHexagram.transformed_hexagram.name}</h3>
        <p><strong>Judgment:</strong> ${currentHexagram.transformed_hexagram.judgment}</p>`;
    }

    renderHexagramSVG(currentHexagram);
}

function renderHexagramSVG(hex) {
    const container = document.getElementById('hex-svg-container');
    container.innerHTML = '';
    const w = 200;
    const h = 200;
    const svg = createSVG(w, h);

    const lineH = 15;
    const gap = 15;
    const totalH = (6 * lineH) + (5 * gap);
    const startY = (h - totalH) / 2 + totalH - lineH; // Centered vertically, drawing bottom up

    hex.lines.forEach((line, i) => {
        const y = startY - (i * (lineH + gap));
        const color = (hex.changing_lines.includes(i)) ? "var(--accent)" : "var(--primary)";

        const group = document.createElementNS(NS, "g");
        group.setAttribute("class", "anim-fade-in");
        group.style.animationDelay = `${i * 0.1}s`;

        if (line === 1) {
            // Yang (Solid)
            group.appendChild(createRect(20, y, 160, lineH, color));
        } else {
            // Yin (Broken)
            group.appendChild(createRect(20, y, 70, lineH, color));
            group.appendChild(createRect(110, y, 70, lineH, color));
        }
        svg.appendChild(group);
    });

    container.appendChild(svg);
}

// === HISTORY ===

async function saveReport(type) {
    let summary = "";
    let data = null;
    let pid = null;

    if (type === 'fengshui' && currentReport) {
        summary = `Feng Shui Period ${currentReport.annual_chart.period}`;
        data = currentReport;
        // Try to get profile ID if selected
        const pVal = document.getElementById('fs-profile-select').value;
        if (pVal) pid = JSON.parse(pVal).id;
    } else if (type === 'divination' && currentHexagram) {
        summary = `Hexagram ${currentHexagram.number}`;
        data = currentHexagram;
    } else {
        alert("No active report to save.");
        return;
    }

    const req = {
        profile_id: pid,
        tool_type: type,
        summary: summary,
        full_report: data
    };

    const res = await fetch('/api/history', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(req)
    });

    if (res.ok) alert("Archived successfully.");
}

async function loadHistory() {
    const res = await fetch('/api/history');
    const items = await res.json();
    const list = document.getElementById('history-list');
    list.innerHTML = '';

    items.forEach(h => {
        const card = document.createElement('div');
        card.className = 'card';
        card.innerHTML = `<h4>${h.tool_type.toUpperCase()}</h4>
        <p>${h.summary}</p>
        <small>${h.created_at} | ${h.profile_name || 'Anonymous'}</small>`;
        list.appendChild(card);
    });
}

async function downloadPdf() {
    // Re-send request to PDF endpoint
    // Reuse current params
    const req = {
        construction_year: parseInt(document.getElementById('fs-year').value),
        facing_degrees: parseFloat(document.getElementById('fs-facing').value),
        intention: document.getElementById('fs-intention').value,
        quantum_mode: document.getElementById('fs-quantum').checked,
        virtual_cures: window.virtualCures || []
    };
    const profileVal = document.getElementById('fs-profile-select').value;
    if (profileVal) {
        const p = JSON.parse(profileVal);
        req.birth_year = p.birth_year;
        req.birth_month = p.birth_month;
        req.birth_day = p.birth_day;
        req.birth_hour = p.birth_hour;
        req.gender = p.gender;
    }

    const res = await fetch('/api/tools/fengshui/pdf', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(req)
    });

    if (res.ok) {
        const blob = await res.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = "FATUM_REPORT.pdf";
        document.body.appendChild(a);
        a.click();
        a.remove();
    } else {
        alert("PDF Generation Failed");
    }
}

// Init
window.virtualCures = [];
function drag(ev) {
    ev.dataTransfer.setData("type", ev.target.dataset.type);
}

// SVG Drop Handling
const fsContainer = document.getElementById('fs-svg-container');
fsContainer.addEventListener('dragover', e => e.preventDefault());
fsContainer.addEventListener('drop', e => {
    e.preventDefault();
    const type = e.dataTransfer.getData("type");
    const rect = fsContainer.getBoundingClientRect();

    // Normalize coordinates to 0-3 grid space
    const x = (e.clientX - rect.left) / (rect.width / 3);
    const y = (e.clientY - rect.top) / (rect.height / 3);

    window.virtualCures.push({ name: type, x, y });

    // Redraw SVG if report exists
    if (currentReport) {
        // Rerunning might not be needed if we just want to update visual.
        // But the previous code called runFengShui() to re-calculate heatmaps if they depend on cures.
        // For now, let's just redraw the SVG locally for instant feedback, then maybe trigger run?
        // The original code re-ran runFengShui(). Let's stick to that to ensure backend logic applies.
        runFengShui();
    }
});
