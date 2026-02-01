use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Reverse SSH Interface</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            background: #0f172a;
            color: #e2e8f0;
            min-height: 100vh;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        header {
            background: #1e293b;
            border-bottom: 1px solid #334155;
            padding: 16px 0;
            margin-bottom: 24px;
        }
        header .container {
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        h1 {
            font-size: 1.5rem;
            font-weight: 600;
            color: #f1f5f9;
        }
        h1 span {
            color: #38bdf8;
        }
        .api-link {
            color: #94a3b8;
            text-decoration: none;
            font-size: 0.875rem;
        }
        .api-link:hover {
            color: #38bdf8;
        }
        .grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 24px;
        }
        @media (max-width: 768px) {
            .grid {
                grid-template-columns: 1fr;
            }
        }
        .card {
            background: #1e293b;
            border-radius: 12px;
            border: 1px solid #334155;
            overflow: hidden;
        }
        .card-header {
            background: #334155;
            padding: 16px 20px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .card-header h2 {
            font-size: 1rem;
            font-weight: 600;
        }
        .card-body {
            padding: 20px;
        }
        .btn {
            background: #3b82f6;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 6px;
            cursor: pointer;
            font-size: 0.875rem;
            font-weight: 500;
            transition: background 0.2s;
        }
        .btn:hover {
            background: #2563eb;
        }
        .btn-success {
            background: #22c55e;
        }
        .btn-success:hover {
            background: #16a34a;
        }
        .btn-danger {
            background: #ef4444;
        }
        .btn-danger:hover {
            background: #dc2626;
        }
        .btn-sm {
            padding: 4px 12px;
            font-size: 0.75rem;
        }
        .list {
            list-style: none;
        }
        .list-item {
            padding: 16px;
            border-bottom: 1px solid #334155;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .list-item:last-child {
            border-bottom: none;
        }
        .list-item:hover {
            background: #334155;
        }
        .profile-info h3 {
            font-size: 1rem;
            font-weight: 600;
            margin-bottom: 4px;
        }
        .profile-info p {
            font-size: 0.875rem;
            color: #94a3b8;
        }
        .session-info {
            flex: 1;
        }
        .session-info h3 {
            font-size: 0.875rem;
            font-weight: 600;
            margin-bottom: 4px;
        }
        .session-info p {
            font-size: 0.75rem;
            color: #94a3b8;
        }
        .status-badge {
            display: inline-block;
            padding: 4px 10px;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 500;
            text-transform: uppercase;
        }
        .status-connected {
            background: #166534;
            color: #bbf7d0;
        }
        .status-starting, .status-reconnecting {
            background: #854d0e;
            color: #fef08a;
        }
        .status-stopped, .status-failed {
            background: #991b1b;
            color: #fecaca;
        }
        .empty-state {
            text-align: center;
            padding: 40px 20px;
            color: #64748b;
        }
        .empty-state p {
            margin-bottom: 8px;
        }
        .modal-overlay {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.7);
            z-index: 100;
            justify-content: center;
            align-items: center;
        }
        .modal-overlay.active {
            display: flex;
        }
        .modal {
            background: #1e293b;
            border-radius: 12px;
            border: 1px solid #334155;
            width: 100%;
            max-width: 500px;
            margin: 20px;
        }
        .modal-header {
            padding: 16px 20px;
            border-bottom: 1px solid #334155;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .modal-header h3 {
            font-size: 1.125rem;
            font-weight: 600;
        }
        .modal-close {
            background: none;
            border: none;
            color: #94a3b8;
            font-size: 1.5rem;
            cursor: pointer;
            line-height: 1;
        }
        .modal-close:hover {
            color: #f1f5f9;
        }
        .modal-body {
            padding: 20px;
        }
        .form-group {
            margin-bottom: 16px;
        }
        .form-group label {
            display: block;
            font-size: 0.875rem;
            font-weight: 500;
            margin-bottom: 6px;
            color: #cbd5e1;
        }
        .form-group input {
            width: 100%;
            padding: 10px 12px;
            background: #0f172a;
            border: 1px solid #334155;
            border-radius: 6px;
            color: #f1f5f9;
            font-size: 0.875rem;
        }
        .form-group input:focus {
            outline: none;
            border-color: #3b82f6;
        }
        .form-group small {
            display: block;
            margin-top: 4px;
            font-size: 0.75rem;
            color: #64748b;
        }
        .modal-footer {
            padding: 16px 20px;
            border-top: 1px solid #334155;
            display: flex;
            justify-content: flex-end;
            gap: 12px;
        }
        .btn-secondary {
            background: #475569;
        }
        .btn-secondary:hover {
            background: #64748b;
        }
        .toast {
            position: fixed;
            bottom: 20px;
            right: 20px;
            background: #1e293b;
            border: 1px solid #334155;
            border-radius: 8px;
            padding: 12px 20px;
            z-index: 200;
            display: none;
        }
        .toast.show {
            display: block;
            animation: slideIn 0.3s ease;
        }
        .toast.success {
            border-color: #22c55e;
        }
        .toast.error {
            border-color: #ef4444;
        }
        @keyframes slideIn {
            from {
                transform: translateX(100%);
                opacity: 0;
            }
            to {
                transform: translateX(0);
                opacity: 1;
            }
        }
        .ws-status {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 0.75rem;
            color: #64748b;
        }
        .ws-dot {
            width: 8px;
            height: 8px;
            border-radius: 50%;
            background: #ef4444;
        }
        .ws-dot.connected {
            background: #22c55e;
        }
    </style>
</head>
<body>
    <header>
        <div class="container">
            <h1>Reverse <span>SSH</span> Interface</h1>
            <div style="display: flex; align-items: center; gap: 20px;">
                <div class="ws-status">
                    <div class="ws-dot" id="wsStatus"></div>
                    <span id="wsStatusText">Disconnected</span>
                </div>
                <a href="/swagger-ui/" class="api-link">API Docs </a>
            </div>
        </div>
    </header>

    <div class="container">
        <div class="grid">
            <div class="card">
                <div class="card-header">
                    <h2> Profiles</h2>
                    <button class="btn btn-sm" onclick="showAddProfileModal()">+ Add Profile</button>
                </div>
                <div class="card-body" style="padding: 0;">
                    <ul class="list" id="profilesList">
                        <li class="empty-state">
                            <p>Loading profiles...</p>
                        </li>
                    </ul>
                </div>
            </div>

            <div class="card">
                <div class="card-header">
                    <h2> Active Sessions</h2>
                </div>
                <div class="card-body" style="padding: 0;">
                    <ul class="list" id="sessionsList">
                        <li class="empty-state">
                            <p>No active sessions</p>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    </div>

    <!-- Add Profile Modal -->
    <div class="modal-overlay" id="addProfileModal">
        <div class="modal">
            <div class="modal-header">
                <h3>Add New Profile</h3>
                <button class="modal-close" onclick="closeModal()">&times;</button>
            </div>
            <form id="addProfileForm" onsubmit="handleAddProfile(event)">
                <div class="modal-body">
                    <div class="form-group">
                        <label for="profileName">Profile Name</label>
                        <input type="text" id="profileName" required placeholder="my-server">
                    </div>
                    <div class="form-group">
                        <label for="profileHost">SSH Host</label>
                        <input type="text" id="profileHost" required placeholder="example.com">
                    </div>
                    <div class="form-group">
                        <label for="profileUser">SSH User</label>
                        <input type="text" id="profileUser" required placeholder="admin">
                    </div>
                    <div class="form-group">
                        <label for="profilePort">SSH Port</label>
                        <input type="number" id="profilePort" value="22" placeholder="22">
                    </div>
                    <div class="form-group">
                        <label for="profileAuth">Authentication</label>
                        <select id="profileAuth" onchange="toggleAuthFields('profileAuth', 'profileKeyPathGroup', 'profilePasswordGroup', 'profileSshpassPathGroup')">
                            <option value="agent">SSH Agent (Recommended)</option>
                            <option value="key_file">Key File</option>
                            <option value="password">Password (via sshpass + SSHPASS env var)</option>
                        </select>
                        <small>For password auth, enter a password below (stored in this browser) or leave it empty to use SSHPASS from the server environment.</small>
                    </div>
                    <div class="form-group" id="profileKeyPathGroup" style="display:none;">
                        <label for="profileKeyPath">Key File Path</label>
                        <input type="text" id="profileKeyPath" placeholder="/home/user/.ssh/id_ed25519">
                    </div>
                    <div class="form-group" id="profilePasswordGroup" style="display:none;">
                        <label for="profilePassword">Password</label>
                        <input type="password" id="profilePassword" placeholder="Password">
                        <small>Stored in this browser's local storage. Not written to profile files.</small>
                    </div>
                    <div class="form-group" id="profileSshpassPathGroup" style="display:none;">
                        <label for="profileSshpassPath">sshpass/plink Path (server)</label>
                        <input type="text" id="profileSshpassPath" placeholder="/usr/bin/sshpass or C:\\Program Files\\PuTTY\\plink.exe">
                        <small>Windows: Use PuTTY's plink.exe (accepts password directly). Linux/Mac: Use sshpass. Leave empty if already in PATH.</small>
                    </div>
                    <div class="form-group">
                        <label>Tunnels</label>
                        <div id="tunnelsEditor">
                            <div class="tunnel-row" style="display:flex; gap:8px; align-items:center; margin-bottom:8px;">
                                <input type="text" class="tunnel-remote-bind" placeholder="localhost" value="localhost" style="width:110px;">
                                <input type="number" class="tunnel-remote-port" placeholder="Remote" min="1" max="65535" style="width:110px;">
                                <span>→</span>
                                <input type="text" class="tunnel-local-host" placeholder="localhost" value="localhost" style="flex:1; min-width:120px;">
                                <span>:</span>
                                <input type="number" class="tunnel-local-port" placeholder="Local" min="1" max="65535" style="width:110px;">
                                <button type="button" class="btn btn-sm btn-danger" onclick="removeTunnelRow(this, 'tunnelsEditor')">×</button>
                            </div>
                        </div>
                        <button type="button" class="btn btn-sm" onclick="addTunnelRow('tunnelsEditor')">+ Add Tunnel</button>
                    </div>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" onclick="closeModal()">Cancel</button>
                    <button type="submit" class="btn btn-success">Create Profile</button>
                </div>
            </form>
        </div>
    </div>

    <!-- Edit Profile Modal -->
    <div class="modal-overlay" id="editProfileModal">
        <div class="modal">
            <div class="modal-header">
                <h3>Edit Profile</h3>
                <button class="modal-close" onclick="closeEditModal()">&times;</button>
            </div>
            <form id="editProfileForm" onsubmit="handleEditProfile(event)">
                <div class="modal-body">
                    <input type="hidden" id="editExistingName" />

                    <div class="form-group">
                        <label for="editProfileName">Profile Name</label>
                        <input type="text" id="editProfileName" required placeholder="my-server">
                    </div>
                    <div class="form-group">
                        <label for="editProfileHost">SSH Host</label>
                        <input type="text" id="editProfileHost" required placeholder="example.com">
                    </div>
                    <div class="form-group">
                        <label for="editProfileUser">SSH User</label>
                        <input type="text" id="editProfileUser" required placeholder="admin">
                    </div>
                    <div class="form-group">
                        <label for="editProfilePort">SSH Port</label>
                        <input type="number" id="editProfilePort" value="22" placeholder="22">
                    </div>
                    <div class="form-group">
                        <label for="editProfileAuth">Authentication</label>
                        <select id="editProfileAuth" onchange="toggleAuthFields('editProfileAuth', 'editKeyPathGroup', 'editPasswordGroup', 'editSshpassPathGroup')">
                            <option value="agent">SSH Agent (Recommended)</option>
                            <option value="key_file">Key File</option>
                            <option value="password">Password (via sshpass + SSHPASS env var)</option>
                        </select>
                        <small>For password auth, enter a password below (stored in this browser) or leave it empty to use SSHPASS from the server environment.</small>
                    </div>
                    <div class="form-group" id="editKeyPathGroup" style="display:none;">
                        <label for="editProfileKeyPath">Key File Path</label>
                        <input type="text" id="editProfileKeyPath" placeholder="/home/user/.ssh/id_ed25519">
                    </div>
                    <div class="form-group" id="editPasswordGroup" style="display:none;">
                        <label for="editProfilePassword">Password</label>
                        <input type="password" id="editProfilePassword" placeholder="Password">
                        <small>Stored in this browser's local storage. Not written to profile files.</small>
                    </div>
                    <div class="form-group" id="editSshpassPathGroup" style="display:none;">
                        <label for="editProfileSshpassPath">sshpass/plink Path (server)</label>
                        <input type="text" id="editProfileSshpassPath" placeholder="/usr/bin/sshpass or C:\\Program Files\\PuTTY\\plink.exe">
                        <small>Windows: Use PuTTY's plink.exe (accepts password directly). Linux/Mac: Use sshpass. Leave empty if already in PATH.</small>
                    </div>
                    <div class="form-group">
                        <label>Tunnels</label>
                        <div id="editTunnelsEditor"></div>
                        <button type="button" class="btn btn-sm" onclick="addTunnelRow('editTunnelsEditor')">+ Add Tunnel</button>
                    </div>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" onclick="closeEditModal()">Cancel</button>
                    <button type="submit" class="btn btn-success">Save Changes</button>
                </div>
            </form>
        </div>
    </div>

    <div class="toast" id="toast"></div>

    <script>
        const API_BASE = '';
        let ws = null;
        let profilesCache = [];

        // Initialize
        document.addEventListener('DOMContentLoaded', () => {
            loadProfiles();
            loadSessions();
            connectWebSocket();
        });

        // WebSocket connection
        function connectWebSocket() {
            const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
            ws = new WebSocket(`${protocol}//${window.location.host}/ws`);
            
            ws.onopen = () => {
                document.getElementById('wsStatus').classList.add('connected');
                document.getElementById('wsStatusText').textContent = 'Connected';
            };
            
            ws.onclose = () => {
                document.getElementById('wsStatus').classList.remove('connected');
                document.getElementById('wsStatusText').textContent = 'Disconnected';
                // Reconnect after 3 seconds
                setTimeout(connectWebSocket, 3000);
            };
            
            ws.onmessage = (event) => {
                const data = JSON.parse(event.data);
                if (data.type === 'sessions_update') {
                    renderSessions(data.data);
                }
            };
        }

        // Load profiles
        async function loadProfiles() {
            try {
                const response = await fetch(`${API_BASE}/api/profiles`);
                const profiles = await response.json();
                profilesCache = Array.isArray(profiles) ? profiles : [];
                renderProfiles(profiles);
            } catch (error) {
                console.error('Failed to load profiles:', error);
                document.getElementById('profilesList').innerHTML = `
                    <li class="empty-state">
                        <p>Failed to load profiles</p>
                    </li>
                `;
            }
        }

        // Load sessions
        async function loadSessions() {
            try {
                const response = await fetch(`${API_BASE}/api/sessions`);
                const sessions = await response.json();
                renderSessions(sessions);
            } catch (error) {
                console.error('Failed to load sessions:', error);
            }
        }

        // Render profiles list
        function renderProfiles(profiles) {
            const list = document.getElementById('profilesList');
            
            if (!profiles || profiles.length === 0) {
                list.innerHTML = `
                    <li class="empty-state">
                        <p>No profiles configured</p>
                        <p><small>Click "Add Profile" to create one</small></p>
                    </li>
                `;
                return;
            }
            
            list.innerHTML = profiles.map(profile => `
                <li class="list-item">
                    <div class="profile-info">
                        <h3>${escapeHtml(profile.name)}</h3>
                        <p>${escapeHtml(profile.user)}@${escapeHtml(profile.host)}:${profile.port}</p>
                        <p>${profile.tunnels.length} tunnel(s)</p>
                    </div>
                    <div style="display: flex; gap: 8px;">
                        <button class="btn btn-sm" onclick='showEditProfileModal(${JSON.stringify(profile.name)})'>Edit</button>
                        <button class="btn btn-success btn-sm" onclick='startSession(${JSON.stringify(profile.name)})'>Start</button>
                        <button class="btn btn-danger btn-sm" onclick='deleteProfile(${JSON.stringify(profile.name)})'>Delete</button>
                    </div>
                </li>
            `).join('');
        }

        function toggleAuthFields(selectId, keyGroupId, passwordGroupId, sshpassGroupId) {
            const value = document.getElementById(selectId).value;

            const keyGroup = document.getElementById(keyGroupId);
            if (keyGroup) {
                keyGroup.style.display = value === 'key_file' ? 'block' : 'none';
            }

            const passwordGroup = document.getElementById(passwordGroupId);
            if (passwordGroup) {
                passwordGroup.style.display = value === 'password' ? 'block' : 'none';
            }

            const sshpassGroup = document.getElementById(sshpassGroupId);
            if (sshpassGroup) {
                sshpassGroup.style.display = value === 'password' ? 'block' : 'none';
            }
        }

        function passwordStorageKey(profileName) {
            return `rssh.password.${profileName}`;
        }

        function sshpassPathStorageKey(profileName) {
            return `rssh.sshpass_path.${profileName}`;
        }

        function loadStoredPassword(profileName) {
            try {
                return localStorage.getItem(passwordStorageKey(profileName)) || '';
            } catch {
                return '';
            }
        }

        function storePassword(profileName, password) {
            try {
                localStorage.setItem(passwordStorageKey(profileName), password);
            } catch {
                // ignore
            }
        }

        function loadStoredSshpassPath(profileName) {
            try {
                return localStorage.getItem(sshpassPathStorageKey(profileName)) || '';
            } catch {
                return '';
            }
        }

        function storeSshpassPath(profileName, sshpassPath) {
            try {
                localStorage.setItem(sshpassPathStorageKey(profileName), sshpassPath);
            } catch {
                // ignore
            }
        }

        function deleteStoredPassword(profileName) {
            try {
                localStorage.removeItem(passwordStorageKey(profileName));
            } catch {
                // ignore
            }
        }

        function addTunnelRow(editorId, preset) {
            const editor = document.getElementById(editorId);
            const row = document.createElement('div');
            row.className = 'tunnel-row';
            row.style.cssText = 'display:flex; gap:8px; align-items:center; margin-bottom:8px;';

            const remoteBind = preset?.remote_bind ?? 'localhost';
            const remotePort = preset?.remote_port ?? '';
            const localHost = preset?.local_host ?? 'localhost';
            const localPort = preset?.local_port ?? '';

            row.innerHTML = `
                <input type="text" class="tunnel-remote-bind" placeholder="localhost" value="${escapeAttribute(String(remoteBind))}" style="width:110px;">
                <input type="number" class="tunnel-remote-port" placeholder="Remote" min="1" max="65535" value="${escapeAttribute(String(remotePort))}" style="width:110px;">
                <span>→</span>
                <input type="text" class="tunnel-local-host" placeholder="localhost" value="${escapeAttribute(String(localHost))}" style="flex:1; min-width:120px;">
                <span>:</span>
                <input type="number" class="tunnel-local-port" placeholder="Local" min="1" max="65535" value="${escapeAttribute(String(localPort))}" style="width:110px;">
                <button type="button" class="btn btn-sm btn-danger" onclick="removeTunnelRow(this, '${editorId}')">×</button>
            `;
            editor.appendChild(row);
        }

        function resetTunnelsEditor(editorId) {
            const editor = document.getElementById(editorId);
            editor.innerHTML = '';
            addTunnelRow(editorId);
        }

        function removeTunnelRow(btn, editorId) {
            const editor = document.getElementById(editorId);
            if (editor.children.length > 1) {
                btn.parentElement.remove();
            }
        }

        function readTunnels(editorId) {
            const rows = document.querySelectorAll(`#${editorId} .tunnel-row`);
            const tunnels = [];
            for (const row of rows) {
                const remoteBind = row.querySelector('.tunnel-remote-bind').value || 'localhost';
                const remotePort = row.querySelector('.tunnel-remote-port').value;
                const localHost = row.querySelector('.tunnel-local-host').value || 'localhost';
                const localPort = row.querySelector('.tunnel-local-port').value;
                if (remotePort && localPort) {
                    tunnels.push({
                        remote_bind: remoteBind,
                        remote_port: parseInt(remotePort),
                        local_host: localHost,
                        local_port: parseInt(localPort),
                    });
                }
            }
            return tunnels;
        }

        function buildAuth(selectId, keyPathId) {
            const authType = document.getElementById(selectId).value;
            if (authType === 'agent') return { type: 'agent' };
            if (authType === 'password') return { type: 'password' };
            const keyPath = document.getElementById(keyPathId).value;
            return { type: 'key_file', path: keyPath };
        }

        // Render sessions list
        function renderSessions(sessions) {
            const list = document.getElementById('sessionsList');
            
            if (!sessions || sessions.length === 0) {
                list.innerHTML = `
                    <li class="empty-state">
                        <p>No active sessions</p>
                        <p><small>Start a profile to create a session</small></p>
                    </li>
                `;
                return;
            }
            
            list.innerHTML = sessions.map(session => `
                <li class="list-item">
                    <div class="session-info">
                        <h3>${escapeHtml(session.profile_name)}</h3>
                        <p>ID: ${session.id.substring(0, 8)}...</p>
                        <p>Started: ${new Date(session.started_at).toLocaleString()}</p>
                        ${session.pid ? `<p>PID: ${session.pid}</p>` : ''}
                        ${session.last_error ? `<p style="color: #f87171;">Error: ${escapeHtml(session.last_error)}</p>` : ''}
                    </div>
                    <div style="display: flex; flex-direction: column; align-items: flex-end; gap: 8px;">
                        <span class="status-badge status-${session.status}">${session.status}</span>
                        <button class="btn btn-danger btn-sm" onclick="stopSession('${session.id}')">Stop</button>
                    </div>
                </li>
            `).join('');
        }

        // Start session
        async function startSession(profileName) {
            try {
                const profile = profilesCache.find(p => p && p.name === profileName);
                const isPasswordAuth = profile?.auth?.type === 'password';
                const body = {};

                if (isPasswordAuth) {
                    let pw = loadStoredPassword(profileName);
                    if (!pw) {
                        pw = prompt(`Enter SSH password for "${profileName}" (will be stored in this browser):`) || '';
                        if (pw) {
                            storePassword(profileName, pw);
                        }
                    }

                    if (pw) {
                        body.password = pw;
                    }

                    const sshpassPath = loadStoredSshpassPath(profileName);
                    if (sshpassPath) {
                        body.sshpass_path = sshpassPath;
                    }
                }

                const response = await fetch(`${API_BASE}/api/sessions/${encodeURIComponent(profileName)}/start`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(body)
                });
                const result = await response.json();
                
                if (response.ok) {
                    showToast('Session started successfully', 'success');
                    loadSessions();
                } else {
                    showToast(result.error || 'Failed to start session', 'error');
                }
            } catch (error) {
                showToast('Failed to start session', 'error');
            }
        }

        // Stop session
        async function stopSession(sessionId) {
            try {
                const response = await fetch(`${API_BASE}/api/sessions/${sessionId}/stop`, {
                    method: 'POST'
                });
                const result = await response.json();
                
                if (response.ok) {
                    showToast('Session stopped', 'success');
                    loadSessions();
                } else {
                    showToast(result.error || 'Failed to stop session', 'error');
                }
            } catch (error) {
                showToast('Failed to stop session', 'error');
            }
        }

        // Delete profile
        async function deleteProfile(profileName) {
            if (!confirm(`Are you sure you want to delete profile "${profileName}"?`)) {
                return;
            }
            
            try {
                const response = await fetch(`${API_BASE}/api/profiles/${encodeURIComponent(profileName)}`, {
                    method: 'DELETE'
                });
                
                if (response.ok) {
                    showToast('Profile deleted', 'success');
                    loadProfiles();
                } else {
                    const result = await response.json();
                    showToast(result.error || 'Failed to delete profile', 'error');
                }
            } catch (error) {
                showToast('Failed to delete profile', 'error');
            }
        }

        // Add profile
        async function handleAddProfile(event) {
            event.preventDefault();
            const tunnels = readTunnels('tunnelsEditor');
            if (tunnels.length === 0) {
                showToast('Please add at least one valid tunnel', 'error');
                return;
            }

            const addAuthType = document.getElementById('profileAuth').value;
            if (addAuthType === 'key_file' && !document.getElementById('profileKeyPath').value.trim()) {
                showToast('Key file path is required for key_file auth', 'error');
                return;
            }

            if (addAuthType === 'password') {
                // Password is stored locally (not sent to server on profile creation).
                // It will be used when starting the session.
                // Note: do not block profile creation if empty; server can still use SSHPASS env.
                const pw = document.getElementById('profilePassword').value || '';
                if (pw) {
                    storePassword(document.getElementById('profileName').value, pw);
                }

                const sp = document.getElementById('profileSshpassPath').value || '';
                if (sp) {
                    storeSshpassPath(document.getElementById('profileName').value, sp);
                }
            }

            const profile = {
                name: document.getElementById('profileName').value,
                host: document.getElementById('profileHost').value,
                user: document.getElementById('profileUser').value,
                port: parseInt(document.getElementById('profilePort').value) || 22,
                auth: buildAuth('profileAuth', 'profileKeyPath'),
                tunnels,
            };
            
            try {
                const response = await fetch(`${API_BASE}/api/profiles`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(profile)
                });
                
                if (response.ok) {
                    showToast('Profile created successfully', 'success');
                    closeModal();
                    loadProfiles();
                    document.getElementById('addProfileForm').reset();
                    document.getElementById('profileKeyPathGroup').style.display = 'none';
                    document.getElementById('profilePasswordGroup').style.display = 'none';
                } else {
                    const result = await response.json();
                    showToast(result.error || 'Failed to create profile', 'error');
                }
            } catch (error) {
                showToast('Failed to create profile', 'error');
            }
        }

        async function showEditProfileModal(profileName) {
            try {
                const response = await fetch(`${API_BASE}/api/profiles/${encodeURIComponent(profileName)}`);
                const profile = await response.json();
                if (!response.ok) {
                    showToast(profile.error || 'Failed to load profile', 'error');
                    return;
                }

                document.getElementById('editExistingName').value = profileName;
                document.getElementById('editProfileName').value = profile.name;
                document.getElementById('editProfileHost').value = profile.host;
                document.getElementById('editProfileUser').value = profile.user;
                document.getElementById('editProfilePort').value = profile.port;

                // auth
                const authType = profile.auth?.type || 'agent';
                document.getElementById('editProfileAuth').value = authType;
                toggleAuthFields('editProfileAuth', 'editKeyPathGroup', 'editPasswordGroup', 'editSshpassPathGroup');
                document.getElementById('editProfileKeyPath').value = authType === 'key_file' ? (profile.auth.path || '') : '';
                document.getElementById('editProfilePassword').value = authType === 'password' ? loadStoredPassword(profileName) : '';
                document.getElementById('editProfileSshpassPath').value = authType === 'password' ? loadStoredSshpassPath(profileName) : '';

                // tunnels
                const editor = document.getElementById('editTunnelsEditor');
                editor.innerHTML = '';
                for (const t of profile.tunnels || []) {
                    addTunnelRow('editTunnelsEditor', t);
                }
                if (editor.children.length === 0) {
                    addTunnelRow('editTunnelsEditor');
                }

                document.getElementById('editProfileModal').classList.add('active');
            } catch (error) {
                showToast('Failed to load profile', 'error');
            }
        }

        function closeEditModal() {
            document.getElementById('editProfileModal').classList.remove('active');
        }

        async function handleEditProfile(event) {
            event.preventDefault();
            const existingName = document.getElementById('editExistingName').value;

            const tunnels = readTunnels('editTunnelsEditor');
            if (tunnels.length === 0) {
                showToast('Please add at least one valid tunnel', 'error');
                return;
            }

            const editAuthType = document.getElementById('editProfileAuth').value;
            if (editAuthType === 'key_file' && !document.getElementById('editProfileKeyPath').value.trim()) {
                showToast('Key file path is required for key_file auth', 'error');
                return;
            }

            // Keep password locally in the browser for password auth.
            const newName = document.getElementById('editProfileName').value;
            if (editAuthType === 'password') {
                const pw = document.getElementById('editProfilePassword').value || '';
                if (pw) {
                    storePassword(newName, pw);
                }

                const sp = document.getElementById('editProfileSshpassPath').value || '';
                if (sp) {
                    storeSshpassPath(newName, sp);
                }
            } else {
                deleteStoredPassword(existingName);
            }

            // If renamed, also move stored password key.
            if (existingName && newName && existingName !== newName) {
                const oldPw = loadStoredPassword(existingName);
                if (oldPw) {
                    storePassword(newName, oldPw);
                    deleteStoredPassword(existingName);
                }

                const oldSp = loadStoredSshpassPath(existingName);
                if (oldSp) {
                    storeSshpassPath(newName, oldSp);
                    try { localStorage.removeItem(sshpassPathStorageKey(existingName)); } catch {}
                }
            }

            const payload = {
                name: newName,
                host: document.getElementById('editProfileHost').value,
                user: document.getElementById('editProfileUser').value,
                port: parseInt(document.getElementById('editProfilePort').value) || 22,
                auth: buildAuth('editProfileAuth', 'editProfileKeyPath'),
                tunnels,
            };

            try {
                const response = await fetch(`${API_BASE}/api/profiles/${encodeURIComponent(existingName)}`, {
                    method: 'PUT',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(payload),
                });

                const result = await response.json();
                if (response.ok) {
                    showToast('Profile updated successfully', 'success');
                    closeEditModal();
                    loadProfiles();
                } else {
                    showToast(result.error || 'Failed to update profile', 'error');
                }
            } catch (error) {
                showToast('Failed to update profile', 'error');
            }
        }

        // Modal functions
        function showAddProfileModal() {
            resetTunnelsEditor('tunnelsEditor');
            document.getElementById('profileAuth').value = 'agent';
            document.getElementById('profileKeyPath').value = '';
            document.getElementById('profilePassword').value = '';
            document.getElementById('profileSshpassPath').value = '';
            toggleAuthFields('profileAuth', 'profileKeyPathGroup', 'profilePasswordGroup', 'profileSshpassPathGroup');
            document.getElementById('addProfileModal').classList.add('active');
        }

        function closeModal() {
            document.getElementById('addProfileModal').classList.remove('active');
        }

        // Toast notification
        function showToast(message, type = 'success') {
            const toast = document.getElementById('toast');
            toast.textContent = message;
            toast.className = `toast show ${type}`;
            
            setTimeout(() => {
                toast.classList.remove('show');
            }, 3000);
        }

        // Escape HTML to prevent XSS
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }

        // Escape for HTML attributes (e.g. value="...")
        function escapeAttribute(text) {
            return String(text)
                .replaceAll('&', '&amp;')
                .replaceAll('<', '&lt;')
                .replaceAll('>', '&gt;')
                .replaceAll('"', '&quot;')
                .replaceAll("'", '&#39;');
        }

        // Close modal on escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                closeModal();
                closeEditModal();
            }
        });

        // Close modal on overlay click
        document.getElementById('addProfileModal').addEventListener('click', (e) => {
            if (e.target === e.currentTarget) {
                closeModal();
            }
        });

        document.getElementById('editProfileModal').addEventListener('click', (e) => {
            if (e.target === e.currentTarget) {
                closeEditModal();
            }
        });
    </script>
</body>
</html>
"##;
