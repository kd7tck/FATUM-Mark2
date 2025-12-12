// GLOBAL STATE for Zi Wei

async function runZiWei() {
    // Try to get active profile from Feng Shui tab selector
    const pVal = document.getElementById('fs-profile-select').value;
    if (!pVal) {
        alert("Please select a profile in the 'Feng Shui' tab first (Zi Wei requires birth data).");
        return;
    }
    const profile = JSON.parse(pVal);

    const req = {
        birth_year: profile.birth_year,
        birth_month: profile.birth_month,
        birth_day: profile.birth_day,
        birth_hour: profile.birth_hour,
        gender: profile.gender
    };

    const res = await fetch('/api/tools/ziwei', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(req)
    });

    const chart = await res.json();
    renderZiWeiChart(chart);
}

function renderZiWeiChart(chart) {
    if (chart.error) {
        alert(chart.error);
        return;
    }

    const container = document.getElementById('zw-chart-container');
    container.innerHTML = '';

    // Standard Zi Wei Layout is a 4x4 Grid.
    // Center 2x2 is usually empty or general info.
    // Palaces wrap around:
    // Si(5)  Wu(6)  Wei(7)  Shen(8)
    // Chen(4)               You(9)
    // Mao(3)                Xu(10)
    // Yin(2) Chou(1) Zi(0)  Hai(11)

    // The backend returns a list of palaces sorted by index 0..11.
    // We need to map Index -> Grid Position (row, col). 0-based, top-left is 0,0.
    // Grid:
    // 0,0 (Si-5) 0,1 (Wu-6) 0,2 (Wei-7) 0,3 (Shen-8)
    // 1,0 (Chen-4)                      1,3 (You-9)
    // 2,0 (Mao-3)                       2,3 (Xu-10)
    // 3,0 (Yin-2) 3,1 (Chou-1) 3,2 (Zi-0) 3,3 (Hai-11)

    const posMap = {
        5: [0,0], 6: [0,1], 7: [0,2], 8: [0,3],
        4: [1,0],                     9: [1,3],
        3: [2,0],                     10: [2,3],
        2: [3,0], 1: [3,1], 0: [3,2], 11: [3,3]
    };

    container.style.display = 'grid';
    container.style.gridTemplateColumns = 'repeat(4, 1fr)';
    container.style.gridTemplateRows = 'repeat(4, 1fr)';
    container.style.gap = '5px';
    container.style.aspectRatio = '1/1';
    container.style.maxWidth = '800px';
    container.style.margin = '0 auto';

    for (let r = 0; r < 4; r++) {
        for (let c = 0; c < 4; c++) {
            const cell = document.createElement('div');
            cell.className = 'ziwei-cell';
            cell.style.border = '1px solid var(--grid-line)';
            cell.style.padding = '5px';
            cell.style.minHeight = '100px';
            cell.style.position = 'relative';
            cell.style.backgroundColor = 'rgba(0,0,0,0.5)';
            cell.style.overflow = 'hidden';

            // Find if this r,c maps to a palace
            let pIndex = -1;
            for (const [idx, pos] of Object.entries(posMap)) {
                if (pos[0] === r && pos[1] === c) {
                    pIndex = parseInt(idx);
                    break;
                }
            }

            if (pIndex !== -1) {
                const palace = chart.palaces.find(p => p.index === pIndex);
                if (palace) {
                    renderPalaceContent(cell, palace, chart);
                }
            } else {
                if (r === 1 && c === 1) {
                    // Center Info
                    cell.style.gridColumn = "2 / 4";
                    cell.style.gridRow = "2 / 4";
                    cell.style.display = "flex";
                    cell.style.flexDirection = "column";
                    cell.style.alignItems = "center";
                    cell.style.justifyContent = "center";
                    cell.innerHTML = `<h3>ZI WEI DOU SHU</h3><p>Element Phase: ${chart.element_phase}</p>`;
                } else if ((r === 1 && c === 2) || (r === 2 && c === 1) || (r === 2 && c === 2)) {
                    continue;
                }
            }
            container.appendChild(cell);
        }
    }
}

function renderPalaceContent(el, p, chart) {
    // Palace Name (Top Left)
    const nameDiv = document.createElement('div');
    nameDiv.className = 'zw-palace-name';
    nameDiv.textContent = p.name;
    nameDiv.style.fontWeight = 'bold';
    nameDiv.style.color = 'var(--accent)';
    nameDiv.style.marginBottom = '5px';
    nameDiv.style.fontSize = '1.0em';

    if (p.index === chart.life_palace_idx) {
        nameDiv.style.textDecoration = "underline";
        nameDiv.style.color = "#fff";
        nameDiv.style.backgroundColor = "var(--primary)";
        nameDiv.style.padding = "2px 4px";
        nameDiv.textContent = "LIFE: " + p.name;
    }
    if (p.index === chart.body_palace_idx) nameDiv.textContent += " (Body)";

    // Branch Name (Bottom Right)
    const branchDiv = document.createElement('div');
    branchDiv.className = 'zw-branch-label';
    branchDiv.textContent = p.branch_name;
    branchDiv.style.position = 'absolute';
    branchDiv.style.bottom = '2px';
    branchDiv.style.right = '5px';
    branchDiv.style.fontSize = '0.9em';
    branchDiv.style.color = '#777';
    branchDiv.style.fontWeight = 'bold';

    // Stars Container
    const starsDiv = document.createElement('div');
    starsDiv.style.display = 'flex';
    starsDiv.style.flexDirection = 'column';
    starsDiv.style.gap = '2px';

    // Major Stars
    p.major_stars.forEach(s => {
        const d = document.createElement('div');
        d.textContent = s;
        d.style.color = '#ff4081'; // Pink/Red for major stars
        d.style.fontSize = '0.85em';
        d.style.fontWeight = 'bold';

        // Highlight Transformations
        if (s.includes("(Hua Lu)")) d.style.color = "#4caf50"; // Green
        if (s.includes("(Hua Quan)")) d.style.color = "#2196f3"; // Blue
        if (s.includes("(Hua Ke)")) d.style.color = "#ffeb3b"; // Yellow
        if (s.includes("(Hua Ji)")) d.style.color = "#f44336"; // Red

        starsDiv.appendChild(d);
    });

    // Minor Stars
    p.minor_stars.forEach(s => {
        const d = document.createElement('div');
        d.textContent = s;
        d.style.color = '#b0bec5'; // Greyish Blue
        d.style.fontSize = '0.75em';

        // Highlight Transformations (if any applied to minors)
        if (s.includes("(Hua Lu)")) d.style.color = "#4caf50";
        if (s.includes("(Hua Quan)")) d.style.color = "#2196f3";
        if (s.includes("(Hua Ke)")) d.style.color = "#ffeb3b";
        if (s.includes("(Hua Ji)")) d.style.color = "#f44336";

        starsDiv.appendChild(d);
    });

    el.appendChild(nameDiv);
    el.appendChild(starsDiv);
    el.appendChild(branchDiv);
}
