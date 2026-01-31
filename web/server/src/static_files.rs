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
                        <label for="profileTunnel">Tunnel Specification</label>
                        <input type="text" id="profileTunnel" required placeholder="8080:localhost:3000">
                        <small>Format: remote_port:local_host:local_port</small>
                    </div>
                </div>
                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" onclick="closeModal()">Cancel</button>
                    <button type="submit" class="btn btn-success">Create Profile</button>
                </div>
            </form>
        </div>
    </div>

    <div class="toast" id="toast"></div>

    <script>
        const API_BASE = '';
        let ws = null;

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
                        <button class="btn btn-success btn-sm" onclick="startSession('${escapeHtml(profile.name)}')">Start</button>
                        <button class="btn btn-danger btn-sm" onclick="deleteProfile('${escapeHtml(profile.name)}')">Delete</button>
                    </div>
                </li>
            `).join('');
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
                const response = await fetch(`${API_BASE}/api/sessions/${encodeURIComponent(profileName)}/start`, {
                    method: 'POST'
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
            
            const tunnelParts = document.getElementById('profileTunnel').value.split(':');
            let tunnel;
            
            if (tunnelParts.length === 2) {
                tunnel = {
                    remote_bind: 'localhost',
                    remote_port: parseInt(tunnelParts[0]),
                    local_host: 'localhost',
                    local_port: parseInt(tunnelParts[1])
                };
            } else if (tunnelParts.length === 3) {
                tunnel = {
                    remote_bind: 'localhost',
                    remote_port: parseInt(tunnelParts[0]),
                    local_host: tunnelParts[1],
                    local_port: parseInt(tunnelParts[2])
                };
            } else {
                showToast('Invalid tunnel format', 'error');
                return;
            }
            
            const profile = {
                name: document.getElementById('profileName').value,
                host: document.getElementById('profileHost').value,
                user: document.getElementById('profileUser').value,
                port: parseInt(document.getElementById('profilePort').value) || 22,
                tunnels: [tunnel]
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
                } else {
                    const result = await response.json();
                    showToast(result.error || 'Failed to create profile', 'error');
                }
            } catch (error) {
                showToast('Failed to create profile', 'error');
            }
        }

        // Modal functions
        function showAddProfileModal() {
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

        // Close modal on escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                closeModal();
            }
        });

        // Close modal on overlay click
        document.getElementById('addProfileModal').addEventListener('click', (e) => {
            if (e.target === e.currentTarget) {
                closeModal();
            }
        });
    </script>
</body>
</html>
"##;
