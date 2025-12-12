
async function runDaLiuRen() {
    const stem = parseInt(document.getElementById('dlr-stem').value);
    const branch = parseInt(document.getElementById('dlr-branch').value);
    const hour = parseInt(document.getElementById('dlr-hour').value);
    const term = parseInt(document.getElementById('dlr-term').value);

    const req = {
        day_stem_idx: stem,
        day_branch_idx: branch,
        hour_branch_idx: hour,
        solar_term_idx: term
    };

    const res = await fetch('/api/tools/daliuren', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(req)
    });

    const chart = await res.json();
    renderDaLiuRen(chart);
}

function renderDaLiuRen(chart) {
    const out = document.getElementById('dlr-output');

    if (chart.error) {
        out.innerHTML = `<span style="color:var(--fire)">Error: ${chart.error}</span>`;
        return;
    }

    let html = `<h3>${chart.description}</h3>`;

    // Three Transmissions
    html += `<div class="dlr-section">
        <h4>Three Transmissions (San Chuan)</h4>
        <div class="dlr-transmissions">
            <div>1. Chu Chuan: ${chart.three_transmissions[0]}</div>
            <div>2. Zhong Chuan: ${chart.three_transmissions[1]}</div>
            <div>3. Mo Chuan: ${chart.three_transmissions[2]}</div>
        </div>
    </div>`;

    // Four Lessons
    html += `<div class="dlr-section">
        <h4>Four Lessons (Si Ke)</h4>
        <div class="dlr-grid-4">
            ${chart.four_lessons.map((l, i) => `
                <div class="dlr-lesson">
                    <strong>Lesson ${i+1}</strong><br>
                    Top: ${l.top}<br>
                    Bottom: ${l.bottom}
                </div>
            `).join('')}
        </div>
    </div>`;

    // Plates (Simplified View)
    html += `<div class="dlr-section">
        <h4>Heaven Plate (On top of Earth 12)</h4>
        <p>Earth (Fixed): Rat, Ox, Tiger, Rabbit, Dragon, Snake, Horse, Goat, Monkey, Rooster, Dog, Pig</p>
        <p>Heaven (Rotated): ${chart.heaven_plate.join(', ')}</p>
    </div>`;

    out.innerHTML = html;
}
