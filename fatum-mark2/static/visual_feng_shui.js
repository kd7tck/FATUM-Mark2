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
        txt += `\n[BAZI PROFILE]\nDay Master: ${report.bazi.day_master}\n`;
        if (report.bazi.quantum_flux) txt += `>> ${report.bazi.quantum_flux}\n`;
        if (report.bazi.alternate_pillars) txt += `>> ${report.bazi.alternate_pillars.join('\n>> ')}\n`;
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

    drawChart(report);
}

function drawChart(report) {
    const cvs = document.getElementById('fs-canvas');
    const ctx = cvs.getContext('2d');
    const w = cvs.width;
    const h = cvs.height;

    ctx.clearRect(0, 0, w, h);

    // Draw 3x3 Grid
    const cw = w / 3;
    const ch = h / 3;

    ctx.strokeStyle = '#00ff9d';
    ctx.lineWidth = 2;

    for (let i=1; i<3; i++) {
        ctx.beginPath();
        ctx.moveTo(i*cw, 0); ctx.lineTo(i*cw, h);
        ctx.stroke();
        ctx.beginPath();
        ctx.moveTo(0, i*ch); ctx.lineTo(w, i*ch);
        ctx.stroke();
    }

    // Draw Stars
    // Palaces order: Center, NW, W, NE, S, N, SW, E, SE
    // Map to grid x,y
    // N is usually bottom in Luo Shu if South is Up?
    // Standard Map: South Up (Row 0), North Down (Row 2)
    // S(0,1), SE(0,0), SW(0,2)
    // E(1,0)?? No.
    // Let's use the standard "South at Top" visual:
    // SE | S | SW
    // E  | C | W
    // NE | N | NW

    // Palace list has .sector name.
    const posMap = {
        "SE": [0,0], "S": [1,0], "SW": [2,0],
        "E": [0,1], "Center": [1,1], "W": [2,1],
        "NE": [0,2], "N": [1,2], "NW": [2,2]
    };

    report.annual_chart.palaces.forEach(p => {
        const [gx, gy] = posMap[p.sector] || [1,1];
        const x = gx * cw;
        const y = gy * ch;

        // Heatmap bg?
        // if (report.quantum.qi_heatmap) ...

        ctx.fillStyle = '#00ff9d';
        ctx.font = '20px Orbitron';
        ctx.fillText(p.sector, x + 10, y + 30);

        // Stars: M-W-B (Mountain-Water-Base) or Standard Left-Right
        // Standard: Mountain (Left), Water (Right), Base/Period (Center), Annual (Bottom Right)

        ctx.font = 'bold 24px monospace';

        ctx.fillStyle = '#ccc'; // Mountain
        ctx.fillText(p.mountain_star, x + 20, y + 60);

        ctx.fillStyle = '#00b8ff'; // Water
        ctx.fillText(p.water_star, x + cw - 40, y + 60);

        ctx.fillStyle = '#ff0055'; // Base
        ctx.font = '30px monospace';
        ctx.fillText(p.base_star, x + cw/2 - 10, y + ch/2 + 10);

        ctx.fillStyle = '#ffff00'; // Annual Visiting
        ctx.font = '18px monospace';
        ctx.fillText(p.visiting_star, x + cw - 30, y + ch - 20);
    });
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

    drawHexagram(currentHexagram);
}

function drawHexagram(hex) {
    const cvs = document.getElementById('hex-canvas');
    const ctx = cvs.getContext('2d');
    const w = cvs.width;
    const h = cvs.height;
    ctx.clearRect(0,0,w,h);

    const lineH = 20;
    const gap = 10;
    const startY = h - 30; // Draw bottom up

    // Draw lines
    hex.lines.forEach((line, i) => {
        const y = startY - (i * (lineH + gap));
        ctx.fillStyle = (hex.changing_lines.includes(i)) ? '#ff0055' : '#00ff9d';

        if (line === 1) {
            // Yang
            ctx.fillRect(20, y, 160, lineH);
        } else {
            // Yin
            ctx.fillRect(20, y, 70, lineH);
            ctx.fillRect(110, y, 70, lineH);
        }
    });
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

// Canvas Drop
const fsCanvas = document.getElementById('fs-canvas');
fsCanvas.addEventListener('dragover', e => e.preventDefault());
fsCanvas.addEventListener('drop', e => {
    e.preventDefault();
    const type = e.dataTransfer.getData("type");
    const rect = fsCanvas.getBoundingClientRect();
    const x = (e.clientX - rect.left) / (rect.width / 3);
    const y = (e.clientY - rect.top) / (rect.height / 3);

    window.virtualCures.push({ name: type, x, y });
    // Re-run to update heatmap
    runFengShui();
});
