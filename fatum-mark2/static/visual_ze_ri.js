// GLOBAL STATE for Ze Ri
// (Assuming visual_feng_shui.js is also loaded for shared utilities, but if not we should duplicate essential ones)
// We rely on 'showTab' and profile loading from visual_feng_shui.js if they are global.
// Since they are in global scope in visual_feng_shui.js, we can access them.

// === DATE SELECTION (ZE RI) ===

async function runZeRi() {
    const start = document.getElementById('zr-start').value;
    const end = document.getElementById('zr-end').value;
    const intention = document.getElementById('zr-intention').value;
    const usePersonal = document.getElementById('zr-personal').checked;

    if (!start || !end) {
        alert("Please specify date range.");
        return;
    }

    let userYear = null;
    if (usePersonal) {
        // Try to get from FS Profile Select (shared component)
        const pVal = document.getElementById('fs-profile-select').value;
        if (pVal) {
            const p = JSON.parse(pVal);
            userYear = p.birth_year;
        } else {
            // Or manual input? For now, warn if no profile selected
            if (!confirm("No profile selected in 'Feng Shui' tab. Proceed with Generic Mode?")) {
                // Do nothing
            }
        }
    }

    // New: Activities
    // We can parse intention string for activities or add a checklist.
    // For now, let's treat intention as comma-separated activities or keyword.
    // Or we could add checkboxes in HTML. Let's stick to parsing intention for now.
    // Actually, let's allow intention to be the "Activities" list.
    let activities = [];
    if (intention) {
        activities = intention.split(',').map(s => s.trim()).filter(s => s.length > 0);
    }

    const req = {
        start_date: start,
        end_date: end,
        intention: intention || null,
        activities: activities.length > 0 ? activities : null,
        user_birth_year: userYear
    };

    const res = await fetch('/api/tools/zeri', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(req)
    });

    const dates = await res.json();
    renderZeRiOutput(dates);
}

function renderZeRiOutput(dates) {
    const out = document.getElementById('zr-output');
    out.innerHTML = '';

    if (dates.error) {
        out.innerHTML = `<p class="error">${dates.error}</p>`;
        return;
    }

    if (dates.length === 0) {
        out.innerHTML = `<p>No auspicious dates found in this range.</p>`;
        return;
    }

    // Grid layout is defined in CSS (.data-grid)
    dates.forEach(d => {
        const card = document.createElement('div');
        card.className = 'card';
        // Color code by score
        let color = '#fff'; // Default text color
        let borderColor = 'var(--primary)';

        if (d.score > 60) {
             borderColor = 'var(--accent)'; // High score
        } else if (d.score < 40) {
             borderColor = 'var(--fire)'; // Low score
        }

        card.style.border = `1px solid ${borderColor}`;

        // Officer formatting
        const officerHtml = `<div style="margin: 5px 0; font-size: 0.9em; color: var(--secondary)">OFFICER: ${d.officer}</div>`;

        // Suitable Activities
        let actsHtml = '';
        if (d.suitable_activities && d.suitable_activities.length > 0) {
            actsHtml = `<div style="font-size: 0.8em; color: #aaa">Suitable: ${d.suitable_activities.join(', ')}</div>`;
        }

        card.innerHTML = `
            <h4 style="color:${d.score > 50 ? 'var(--accent)' : 'var(--fire)'}">${d.date} (Score: ${d.score})</h4>
            ${officerHtml}
            <p>${d.summary}</p>
            ${actsHtml}
            ${d.collision ? `<p style="color:var(--fire)">⚠️ ${d.collision}</p>` : ''}
        `;
        out.appendChild(card);
    });
}
