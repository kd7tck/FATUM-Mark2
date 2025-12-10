// editor.js

class TreeEditor {
    constructor(containerId, jsonOutputId) {
        this.container = document.getElementById(containerId);
        this.jsonOutput = document.getElementById(jsonOutputId);
        this.tree = {
            root_node_id: "start",
            nodes: {
                "start": {
                    id: "start",
                    question: "Start Question?",
                    options: []
                }
            }
        };
        this.render();
    }

    loadTree(treeData) {
        if (!treeData.root_node_id || !treeData.nodes) {
            alert("Invalid Tree Data");
            return;
        }
        this.tree = treeData;
        this.render();
        this.syncJson();
    }

    syncJson() {
        this.jsonOutput.value = JSON.stringify(this.tree, null, 2);
    }

    addNode() {
        const id = "node_" + Math.random().toString(36).substr(2, 5);
        this.tree.nodes[id] = {
            id: id,
            question: "New Question",
            options: []
        };
        this.render();
        this.syncJson();
    }

    deleteNode(id) {
        if (id === this.tree.root_node_id) {
            alert("Cannot delete Root Node");
            return;
        }
        if (confirm("Delete node " + id + "?")) {
            delete this.tree.nodes[id];
            this.render();
            this.syncJson();
        }
    }

    updateNodeId(oldId, newId) {
        if (oldId === newId) return;
        if (this.tree.nodes[newId]) {
            alert("ID already exists!");
            this.render(); // Revert
            return;
        }

        // Move data
        this.tree.nodes[newId] = this.tree.nodes[oldId];
        this.tree.nodes[newId].id = newId;
        delete this.tree.nodes[oldId];

        // Update Root pointer
        if (this.tree.root_node_id === oldId) {
            this.tree.root_node_id = newId;
        }

        // Update all references in options
        for (const key in this.tree.nodes) {
            const node = this.tree.nodes[key];
            node.options.forEach(opt => {
                if (opt.next_node_id === oldId) {
                    opt.next_node_id = newId;
                }
            });
        }

        this.render();
        this.syncJson();
    }

    updateQuestion(id, text) {
        if (this.tree.nodes[id]) {
            this.tree.nodes[id].question = text;
            this.syncJson();
        }
    }

    addOption(nodeId) {
        if (this.tree.nodes[nodeId]) {
            this.tree.nodes[nodeId].options.push({
                text: "Option",
                weight: 1.0,
                next_node_id: null
            });
            this.render();
            this.syncJson();
        }
    }

    deleteOption(nodeId, index) {
        if (this.tree.nodes[nodeId]) {
            this.tree.nodes[nodeId].options.splice(index, 1);
            this.render();
            this.syncJson();
        }
    }

    updateOption(nodeId, index, field, value) {
        if (this.tree.nodes[nodeId] && this.tree.nodes[nodeId].options[index]) {
            const opt = this.tree.nodes[nodeId].options[index];
            if (field === 'weight') value = parseFloat(value) || 1.0;
            opt[field] = value;
            this.syncJson();
        }
    }

    render() {
        this.container.innerHTML = '';
        const nodeIds = Object.keys(this.tree.nodes).sort();

        nodeIds.forEach(id => {
            const node = this.tree.nodes[id];
            const isRoot = (id === this.tree.root_node_id);

            const card = document.createElement('div');
            card.className = `node-card ${isRoot ? 'is-root' : ''}`;

            // Header
            const header = document.createElement('div');
            header.className = 'node-header';

            const idInput = document.createElement('input');
            idInput.className = 'node-id-input';
            idInput.value = id;
            idInput.onchange = (e) => this.updateNodeId(id, e.target.value);

            const actions = document.createElement('div');
            actions.className = 'node-actions';

            if (!isRoot) {
                const setRootBtn = document.createElement('button');
                setRootBtn.className = 'btn-mini';
                setRootBtn.innerText = 'Set Root';
                setRootBtn.onclick = () => {
                    this.tree.root_node_id = id;
                    this.render();
                    this.syncJson();
                };
                actions.appendChild(setRootBtn);

                const delBtn = document.createElement('button');
                delBtn.className = 'btn-mini btn-delete';
                delBtn.innerText = 'X';
                delBtn.onclick = () => this.deleteNode(id);
                actions.appendChild(delBtn);
            } else {
                 const rootLabel = document.createElement('span');
                 rootLabel.innerText = "[ROOT]";
                 rootLabel.style.color = "var(--neon-green)";
                 rootLabel.style.fontSize = "0.8em";
                 actions.appendChild(rootLabel);
            }

            header.appendChild(idInput);
            header.appendChild(actions);
            card.appendChild(header);

            // Question
            const qInput = document.createElement('textarea');
            qInput.className = 'node-question';
            qInput.value = node.question;
            qInput.placeholder = "Enter Question...";
            qInput.onchange = (e) => this.updateQuestion(id, e.target.value);
            card.appendChild(qInput);

            // Options
            const optList = document.createElement('ul');
            optList.className = 'options-list';

            node.options.forEach((opt, idx) => {
                const item = document.createElement('li');
                item.className = 'option-item';

                // Row 1: Text and Weight
                const row1 = document.createElement('div');
                row1.className = 'option-row';

                const txt = document.createElement('input');
                txt.className = 'option-text';
                txt.value = opt.text;
                txt.placeholder = "Choice Text";
                txt.onchange = (e) => this.updateOption(id, idx, 'text', e.target.value);

                const w = document.createElement('input');
                w.className = 'option-weight';
                w.type = 'number';
                w.step = '0.1';
                w.value = opt.weight || 1.0;
                w.title = "Weight";
                w.onchange = (e) => this.updateOption(id, idx, 'weight', e.target.value);

                const delOpt = document.createElement('button');
                delOpt.className = 'btn-mini btn-delete';
                delOpt.innerText = 'x';
                delOpt.style.width = '20px';
                delOpt.onclick = () => this.deleteOption(id, idx);

                row1.appendChild(txt);
                row1.appendChild(w);
                row1.appendChild(delOpt);
                item.appendChild(row1);

                // Row 2: Next Node Select
                const nextSelect = document.createElement('select');
                nextSelect.className = 'option-next';

                // Populate Dropdown
                const nullOpt = document.createElement('option');
                nullOpt.value = "";
                nullOpt.innerText = "-- End / Leaf --";
                nextSelect.appendChild(nullOpt);

                nodeIds.forEach(targetId => {
                    // Prevent self-loop if desired? Allowed for cyclic.
                    const o = document.createElement('option');
                    o.value = targetId;
                    o.innerText = `-> ${targetId}`;
                    if (opt.next_node_id === targetId) o.selected = true;
                    nextSelect.appendChild(o);
                });

                nextSelect.onchange = (e) => {
                    const val = e.target.value === "" ? null : e.target.value;
                    this.updateOption(id, idx, 'next_node_id', val);
                };

                item.appendChild(nextSelect);
                optList.appendChild(item);
            });

            card.appendChild(optList);

            // Add Option Btn
            const addBtn = document.createElement('button');
            addBtn.className = 'add-option-btn';
            addBtn.innerText = '+ Add Option';
            addBtn.onclick = () => this.addOption(id);
            card.appendChild(addBtn);

            this.container.appendChild(card);
        });
    }
}

// Initialization and Global Hooks
window.editorInstance = null;

function initEditor() {
    if (!window.editorInstance) {
        window.editorInstance = new TreeEditor('nodeEditorContainer', 'decisionTreeJson');
    }
}

function handleFileUpload(input) {
    const file = input.files[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = function(e) {
        try {
            const json = JSON.parse(e.target.result);
            if (window.editorInstance) {
                window.editorInstance.loadTree(json);
            } else {
                // If not init, init then load
                initEditor();
                window.editorInstance.loadTree(json);
                // Also switch mode UI
                document.getElementById('radioVisual').checked = true;
                toggleDecisionMode();
            }
        } catch (err) {
            alert("Error parsing JSON: " + err.message);
        }
    };
    reader.readAsText(file);
}
