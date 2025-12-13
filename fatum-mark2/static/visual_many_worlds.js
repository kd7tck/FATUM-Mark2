// Visualizer for Many-Worlds Simulation (D3.js / SVG)

async function runManyWorldsSimulation() {
    const profileSelect = document.getElementById('mw-profile');
    if (!profileSelect.value) {
        alert("Please select a profile first.");
        return;
    }

    // We need birth year from the profile.
    // Since the select value is just ID, we need to fetch the profile data or store it in dataset.
    // Assuming the select options have dataset attributes or we fetch it.
    // For now, let's just assume we send the ID and the backend handles lookup?
    // The backend `handle_many_worlds` expects `birth_year`.
    // Let's grab it from the option text if it's there, or we fetch profiles.
    // A better way is to make `loadProfiles` attach data to the options.

    const selectedOption = profileSelect.options[profileSelect.selectedIndex];
    // Assuming format "Name (Year)"
    let birthYear = 1990;
    const match = selectedOption.text.match(/\((\d{4})\)/);
    if (match) {
        birthYear = parseInt(match[1]);
    }

    const duration = parseInt(document.getElementById('mw-duration').value) || 10;
    const numWorlds = parseInt(document.getElementById('mw-worlds').value) || 50;

    document.getElementById('mw-status').innerText = "Harvesting Quantum Entropy & Simulating...";

    try {
        const response = await fetch('/api/tools/many_worlds', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                birth_year: birthYear,
                duration: duration,
                num_worlds: numWorlds
            })
        });

        const result = await response.json();

        if (result.error) {
            document.getElementById('mw-status').innerText = "Error: " + result.error;
        } else {
            document.getElementById('mw-status').innerText = "Simulation Complete.";
            renderManyWorldsChart(result);
        }

    } catch (e) {
        console.error(e);
        document.getElementById('mw-status').innerText = "Network Error.";
    }
}

function renderManyWorldsChart(data) {
    const container = document.getElementById('mw-chart-container');
    container.innerHTML = ''; // Clear previous

    // Setup SVG
    const width = container.clientWidth || 800;
    const height = 500;
    const margin = { top: 20, right: 30, bottom: 30, left: 40 };

    // Create SVG element
    const ns = "http://www.w3.org/2000/svg";
    const svg = document.createElementNS(ns, "svg");
    svg.setAttribute("width", width);
    svg.setAttribute("height", height);
    svg.setAttribute("viewBox", `0 0 ${width} ${height}`);
    svg.style.background = "#1a1a1a";
    container.appendChild(svg);

    // Scales
    const numSteps = data.aggregate_stats.length;
    // Assume scores are roughly around 100? or sum of elements (~100-200)
    // Let's find min/max
    let minScore = 10000;
    let maxScore = -10000;

    data.paths.forEach(p => {
        p.steps.forEach(s => {
            if (s.score < minScore) minScore = s.score;
            if (s.score > maxScore) maxScore = s.score;
        });
    });

    // Add padding to Y scale
    const yPadding = (maxScore - minScore) * 0.1;
    minScore -= yPadding;
    maxScore += yPadding;

    const xScale = (step) => margin.left + (step / (numSteps - 1)) * (width - margin.left - margin.right);
    const yScale = (score) => height - margin.bottom - ((score - minScore) / (maxScore - minScore)) * (height - margin.top - margin.bottom);

    // Draw Axes (Simple)
    const xAxisLine = document.createElementNS(ns, "line");
    xAxisLine.setAttribute("x1", margin.left);
    xAxisLine.setAttribute("y1", height - margin.bottom);
    xAxisLine.setAttribute("x2", width - margin.right);
    xAxisLine.setAttribute("y2", height - margin.bottom);
    xAxisLine.setAttribute("stroke", "#555");
    svg.appendChild(xAxisLine);

    const yAxisLine = document.createElementNS(ns, "line");
    yAxisLine.setAttribute("x1", margin.left);
    yAxisLine.setAttribute("y1", margin.top);
    yAxisLine.setAttribute("x2", margin.left);
    yAxisLine.setAttribute("y2", height - margin.bottom);
    yAxisLine.setAttribute("stroke", "#555");
    svg.appendChild(yAxisLine);

    // Color map for elements
    const colors = {
        "Wood": "#4caf50",
        "Fire": "#f44336",
        "Earth": "#ffeb3b",
        "Metal": "#bdc3c7",
        "Water": "#2196f3",
        "Unknown": "#fff"
    };

    // Draw Paths
    data.paths.forEach(path => {
        // We draw line segments because color might change each step
        for (let i = 0; i < path.steps.length - 1; i++) {
            const curr = path.steps[i];
            const next = path.steps[i+1];

            const line = document.createElementNS(ns, "line");
            line.setAttribute("x1", xScale(curr.step_index));
            line.setAttribute("y1", yScale(curr.score));
            line.setAttribute("x2", xScale(next.step_index));
            line.setAttribute("y2", yScale(next.score));

            // Color based on dominant element of the NEXT step (the result of flux)
            line.setAttribute("stroke", colors[next.dominant_element] || "#fff");
            line.setAttribute("stroke-width", "1.5");
            line.setAttribute("opacity", "0.4"); // Semi-transparent to show density

            svg.appendChild(line);
        }
    });

    // Draw Aggregate Average Line
    let avgPathD = "";
    data.aggregate_stats.forEach((stat, i) => {
        const x = xScale(stat.step_index);
        const y = yScale(stat.avg_score);
        if (i === 0) avgPathD += `M ${x} ${y}`;
        else avgPathD += ` L ${x} ${y}`;
    });

    const avgPath = document.createElementNS(ns, "path");
    avgPath.setAttribute("d", avgPathD);
    avgPath.setAttribute("fill", "none");
    avgPath.setAttribute("stroke", "#fff");
    avgPath.setAttribute("stroke-width", "3");
    avgPath.setAttribute("stroke-dasharray", "5,5"); // Dashed
    svg.appendChild(avgPath);

    // Labels
    const title = document.createElementNS(ns, "text");
    title.setAttribute("x", width / 2);
    title.setAttribute("y", margin.top);
    title.setAttribute("text-anchor", "middle");
    title.setAttribute("fill", "#0ff");
    title.textContent = `Simulated ${data.paths.length} Timelines`;
    svg.appendChild(title);
}

// Expose to window
window.runManyWorldsSimulation = runManyWorldsSimulation;
