/* ============================================================================
   Reverse SSH Interface - GUI Application Logic
   ============================================================================ */

// Tauri API
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { open } = window.__TAURI__.shell;

// ============================================================================
// State Management
// ============================================================================

const state = {
    profiles: [],
    sessions: [],
    currentProfile: null,
    confirmCallback: null,
};

// ============================================================================
// Initialization
// ============================================================================

document.addEventListener('DOMContentLoaded', async () => {
    // Setup navigation
    setupNavigation();
    
    // Setup event listeners
    setupEventListeners();
    
    // Load initial data
    await loadProfiles();
    await loadSessions();
    await loadSettings();
    
    // Start auto-refresh
    setInterval(loadSessions, 5000);
    
    // Log startup
    addLog('info', 'Application initialized');
});

// ============================================================================
// Navigation
// ============================================================================

function setupNavigation() {
    const navItems = document.querySelectorAll('.nav-item');
    
    navItems.forEach(item => {
        item.addEventListener('click', () => {
            const tab = item.dataset.tab;
            switchTab(tab);
        });
    });
}

function switchTab(tabName) {
    // Update nav items
    document.querySelectorAll('.nav-item').forEach(item => {
        item.classList.toggle('active', item.dataset.tab === tabName);
    });
    
    // Update tab content
    document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.toggle('active', content.id === `tab-${tabName}`);
    });
    
    // Refresh data for specific tabs
    if (tabName === 'profiles') {
        loadProfiles();
    } else if (tabName === 'sessions') {
        loadSessions();
    }
}

// ============================================================================
// Event Listeners (Tauri Events)
// ============================================================================

function setupEventListeners() {
    // Session events from backend
    listen('session-started', (event) => {
        const session = event.payload;
        addLog('success', `Session started: ${session.profile_name}`);
        showToast('success', 'Session Started', `Connected to ${session.profile_name}`);
        loadSessions();
    });
    
    listen('session-connected', (event) => {
        addLog('success', `Session connected`);
        loadSessions();
    });
    
    listen('session-disconnected', (event) => {
        const data = event.payload;
        addLog('warning', `Session disconnected: ${data.reason || 'Unknown reason'}`);
        loadSessions();
    });
    
    listen('session-reconnecting', (event) => {
        const data = event.payload;
        addLog('warning', `Reconnecting... (attempt ${data.attempt})`);
        loadSessions();
    });
    
    listen('session-failed', (event) => {
        const data = event.payload;
        addLog('error', `Session failed: ${data.error}`);
        showToast('error', 'Session Failed', data.error);
        loadSessions();
    });
    
    listen('session-stopped', (event) => {
        addLog('info', `Session stopped: ${event.payload}`);
        loadSessions();
    });
    
    listen('session-output', (event) => {
        const data = event.payload;
        if (data.output) {
            addLog('debug', data.output);
        }
    });
    
    listen('all-sessions-stopped', () => {
        addLog('info', 'All sessions stopped');
        showToast('info', 'Sessions Stopped', 'All tunnel sessions have been stopped');
        loadSessions();
    });
}

// ============================================================================
// Profiles
// ============================================================================

async function loadProfiles() {
    try {
        state.profiles = await invoke('get_profiles');
        renderProfiles();
        renderQuickActions();
        renderQuickConnect();
        updateStats();
    } catch (error) {
        console.error('Failed to load profiles:', error);
        showToast('error', 'Error', 'Failed to load profiles');
    }
}

function renderProfiles() {
    const grid = document.getElementById('profilesGrid');
    
    if (state.profiles.length === 0) {
        grid.innerHTML = '<p class="empty-state">No profiles found. Create your first profile!</p>';
        return;
    }
    
    grid.innerHTML = state.profiles.map(profile => `
        <div class="profile-card" onclick="showProfileDetail('${profile.name}')">
            <div class="profile-card-header">
                <h3>${escapeHtml(profile.name)}</h3>
                <span class="text-muted">${escapeHtml(profile.host)}</span>
            </div>
            <div class="profile-card-body">
                <div class="profile-detail">
                    <span class="profile-detail-label">User</span>
                    <span class="profile-detail-value">${escapeHtml(profile.user)}</span>
                </div>
                <div class="profile-detail">
                    <span class="profile-detail-label">Port</span>
                    <span class="profile-detail-value">${profile.port}</span>
                </div>
                <div class="profile-detail">
                    <span class="profile-detail-label">Tunnels</span>
                    <span class="profile-detail-value">${profile.tunnels.length}</span>
                </div>
                <div class="profile-detail">
                    <span class="profile-detail-label">Auth</span>
                    <span class="profile-detail-value">${formatAuth(profile.auth)}</span>
                </div>
            </div>
            <div class="profile-card-footer">
                <button class="btn btn-primary btn-sm" onclick="event.stopPropagation(); startSession('${profile.name}')">
                    Connect
                </button>
                <button class="btn btn-ghost btn-sm" onclick="event.stopPropagation(); confirmDeleteProfile('${profile.name}')">
                    Delete
                </button>
            </div>
        </div>
    `).join('');
}

function renderQuickActions() {
    const container = document.getElementById('quickProfilesList');
    
    if (state.profiles.length === 0) {
        container.innerHTML = '<p class="empty-state">No profiles yet. Create one to get started!</p>';
        return;
    }
    
    container.innerHTML = state.profiles.slice(0, 6).map(profile => `
        <button class="quick-action-btn" onclick="startSession('${profile.name}')">
            <div class="profile-name">${escapeHtml(profile.name)}</div>
            <div class="profile-host">${escapeHtml(profile.user)}@${escapeHtml(profile.host)}</div>
        </button>
    `).join('');
}

function renderQuickConnect() {
    const list = document.getElementById('quickConnectList');
    
    if (state.profiles.length === 0) {
        list.innerHTML = '<p class="empty-state">No profiles available</p>';
        return;
    }
    
    list.innerHTML = state.profiles.map(profile => `
        <div class="quick-connect-item" onclick="startSession('${profile.name}'); closeModal('quickConnectModal');">
            <div>
                <div class="name">${escapeHtml(profile.name)}</div>
                <div class="host">${escapeHtml(profile.user)}@${escapeHtml(profile.host)}:${profile.port}</div>
            </div>
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
        </div>
    `).join('');
}

async function showProfileDetail(name) {
    try {
        const profile = await invoke('get_profile', { name });
        state.currentProfile = profile;
        
        document.getElementById('profileDetailTitle').textContent = profile.name;
        
        const body = document.getElementById('profileDetailBody');
        body.innerHTML = `
            <div class="profile-detail">
                <span class="profile-detail-label">Host</span>
                <span class="profile-detail-value">${escapeHtml(profile.host)}:${profile.port}</span>
            </div>
            <div class="profile-detail">
                <span class="profile-detail-label">User</span>
                <span class="profile-detail-value">${escapeHtml(profile.user)}</span>
            </div>
            <div class="profile-detail">
                <span class="profile-detail-label">Authentication</span>
                <span class="profile-detail-value">${formatAuth(profile.auth)}</span>
            </div>
            <div class="profile-detail">
                <span class="profile-detail-label">Auto Reconnect</span>
                <span class="profile-detail-value">${profile.auto_reconnect ? 'Yes' : 'No'}</span>
            </div>
            <div class="profile-detail">
                <span class="profile-detail-label">Keep-alive</span>
                <span class="profile-detail-value">${profile.keepalive_interval}s</span>
            </div>
            <h3 style="margin-top: 16px; margin-bottom: 8px;">Tunnels</h3>
            ${profile.tunnels.map(t => `
                <div class="profile-detail">
                    <span class="profile-detail-label">${t.remote_bind}:${t.remote_port}</span>
                    <span class="profile-detail-value">→ ${t.local_host}:${t.local_port}</span>
                </div>
            `).join('')}
        `;
        
        showModal('profileDetailModal');
    } catch (error) {
        showToast('error', 'Error', `Failed to load profile: ${error}`);
    }
}

function connectCurrentProfile() {
    if (state.currentProfile) {
        startSession(state.currentProfile.name);
        closeModal('profileDetailModal');
    }
}

function deleteCurrentProfile() {
    if (state.currentProfile) {
        confirmDeleteProfile(state.currentProfile.name);
        closeModal('profileDetailModal');
    }
}

function confirmDeleteProfile(name) {
    showConfirm(
        'Delete Profile',
        `Are you sure you want to delete the profile "${name}"?`,
        async () => {
            try {
                await invoke('delete_profile', { name });
                showToast('success', 'Deleted', `Profile "${name}" has been deleted`);
                loadProfiles();
            } catch (error) {
                showToast('error', 'Error', `Failed to delete profile: ${error}`);
            }
        }
    );
}

// ============================================================================
// Create Profile
// ============================================================================

function showCreateProfileModal() {
    // Reset form
    document.getElementById('createProfileForm').reset();
    document.getElementById('keyPathGroup').style.display = 'none';
    
    // Reset tunnels to single row
    const tunnelsEditor = document.getElementById('tunnelsEditor');
    tunnelsEditor.innerHTML = `
        <div class="tunnel-row">
            <input type="number" class="tunnel-remote" placeholder="Remote Port" min="1" max="65535">
            <span class="tunnel-arrow">→</span>
            <input type="text" class="tunnel-local-host" placeholder="localhost" value="localhost">
            <span>:</span>
            <input type="number" class="tunnel-local-port" placeholder="Local Port" min="1" max="65535">
            <button type="button" class="btn btn-ghost btn-sm" onclick="removeTunnelRow(this)">×</button>
        </div>
    `;
    
    showModal('createProfileModal');
}

function toggleKeyPath() {
    const auth = document.getElementById('profileAuth').value;
    const keyGroup = document.getElementById('keyPathGroup');
    keyGroup.style.display = auth === 'key' ? 'block' : 'none';
}

function addTunnelRow() {
    const editor = document.getElementById('tunnelsEditor');
    const row = document.createElement('div');
    row.className = 'tunnel-row';
    row.innerHTML = `
        <input type="number" class="tunnel-remote" placeholder="Remote Port" min="1" max="65535">
        <span class="tunnel-arrow">→</span>
        <input type="text" class="tunnel-local-host" placeholder="localhost" value="localhost">
        <span>:</span>
        <input type="number" class="tunnel-local-port" placeholder="Local Port" min="1" max="65535">
        <button type="button" class="btn btn-ghost btn-sm" onclick="removeTunnelRow(this)">×</button>
    `;
    editor.appendChild(row);
}

function removeTunnelRow(btn) {
    const editor = document.getElementById('tunnelsEditor');
    if (editor.children.length > 1) {
        btn.parentElement.remove();
    }
}

async function createProfile(event) {
    event.preventDefault();
    
    const name = document.getElementById('profileName').value.trim();
    const host = document.getElementById('profileHost').value.trim();
    const port = document.getElementById('profilePort').value || null;
    const user = document.getElementById('profileUser').value.trim();
    const authType = document.getElementById('profileAuth').value;
    const keyPath = document.getElementById('profileKeyPath').value.trim();
    const autoReconnect = document.getElementById('profileAutoReconnect').checked;
    
    // Get tunnels
    const tunnelRows = document.querySelectorAll('#tunnelsEditor .tunnel-row');
    const tunnels = [];
    
    for (const row of tunnelRows) {
        const remotePort = row.querySelector('.tunnel-remote').value;
        const localHost = row.querySelector('.tunnel-local-host').value || 'localhost';
        const localPort = row.querySelector('.tunnel-local-port').value;
        
        if (remotePort && localPort) {
            tunnels.push({
                remote_bind: 'localhost',
                remote_port: parseInt(remotePort),
                local_host: localHost,
                local_port: parseInt(localPort),
            });
        }
    }
    
    if (tunnels.length === 0) {
        showToast('warning', 'Warning', 'Please add at least one tunnel');
        return;
    }
    
    // Build auth
    let auth = 'agent';
    if (authType === 'key' && keyPath) {
        auth = `key:${keyPath}`;
    } else if (authType === 'password') {
        auth = 'password';
    }
    
    try {
        await invoke('create_profile', {
            request: {
                name,
                host,
                port: port ? parseInt(port) : null,
                user,
                auth,
                key_path: authType === 'key' ? keyPath : null,
                tunnels,
                auto_reconnect: autoReconnect,
            }
        });
        
        showToast('success', 'Profile Created', `Profile "${name}" has been created`);
        closeModal('createProfileModal');
        loadProfiles();
    } catch (error) {
        showToast('error', 'Error', `Failed to create profile: ${error}`);
    }
}

// ============================================================================
// Sessions
// ============================================================================

async function loadSessions() {
    try {
        state.sessions = await invoke('get_sessions');
        renderSessions();
        renderDashboardSessions();
        updateStats();
    } catch (error) {
        console.error('Failed to load sessions:', error);
    }
}

function refreshSessions() {
    loadSessions();
    showToast('info', 'Refreshed', 'Sessions list updated');
}

function renderSessions() {
    const tbody = document.getElementById('sessionsTableBody');
    
    if (state.sessions.length === 0) {
        tbody.innerHTML = '<tr><td colspan="7" class="empty-state">No active sessions</td></tr>';
        return;
    }
    
    tbody.innerHTML = state.sessions.map(session => `
        <tr>
            <td>
                <span class="status-badge ${getStatusClass(session.status)}">
                    <span class="session-status ${getStatusClass(session.status)}"></span>
                    ${session.status}
                </span>
            </td>
            <td>${escapeHtml(session.profile_name)}</td>
            <td><code>${session.id.substring(0, 8)}...</code></td>
            <td>${formatTime(session.started_at)}</td>
            <td>${session.pid || '-'}</td>
            <td>${session.reconnect_count}</td>
            <td>
                <button class="btn btn-danger btn-sm" onclick="stopSession('${session.id}')">
                    Stop
                </button>
            </td>
        </tr>
    `).join('');
}

function renderDashboardSessions() {
    const container = document.getElementById('dashboardSessionsList');
    const stopAllBtn = document.getElementById('stopAllBtn');
    
    if (state.sessions.length === 0) {
        container.innerHTML = '<p class="empty-state">No active sessions</p>';
        stopAllBtn.style.display = 'none';
        return;
    }
    
    stopAllBtn.style.display = 'block';
    
    container.innerHTML = state.sessions.map(session => `
        <div class="session-item">
            <div class="session-info">
                <span class="session-status ${getStatusClass(session.status)}"></span>
                <div class="session-details">
                    <h3>${escapeHtml(session.profile_name)}</h3>
                    <p>${session.status} • Started ${formatTime(session.started_at)}</p>
                </div>
            </div>
            <button class="btn btn-danger btn-sm" onclick="stopSession('${session.id}')">
                Stop
            </button>
        </div>
    `).join('');
}

async function startSession(profileName) {
    try {
        addLog('info', `Starting session for profile: ${profileName}`);
        await invoke('start_session', { name: profileName });
    } catch (error) {
        showToast('error', 'Error', `Failed to start session: ${error}`);
        addLog('error', `Failed to start session: ${error}`);
    }
}

async function stopSession(sessionId) {
    try {
        await invoke('stop_session', { sessionId });
        showToast('success', 'Session Stopped', 'The tunnel session has been stopped');
    } catch (error) {
        showToast('error', 'Error', `Failed to stop session: ${error}`);
    }
}

async function stopAllSessions() {
    showConfirm(
        'Stop All Sessions',
        'Are you sure you want to stop all active tunnel sessions?',
        async () => {
            try {
                await invoke('stop_all_sessions');
            } catch (error) {
                showToast('error', 'Error', `Failed to stop sessions: ${error}`);
            }
        }
    );
}

// ============================================================================
// Settings
// ============================================================================

async function loadSettings() {
    try {
        // Get profiles path
        const path = await invoke('get_profiles_path');
        document.getElementById('profilesPath').textContent = path;
        
        // Get config
        const config = await invoke('get_config');
        
        if (config.general) {
            document.getElementById('startMinimized').checked = config.general.start_minimized || false;
        }
        
        if (config.ssh) {
            document.getElementById('keepaliveInterval').value = config.ssh.default_keepalive_interval || 20;
            document.getElementById('hostKeyChecking').value = config.ssh.strict_host_key_checking || 'accept_new';
        }
    } catch (error) {
        console.error('Failed to load settings:', error);
    }
}

async function openProfilesFolder() {
    try {
        const path = await invoke('get_profiles_path');
        await open(path);
    } catch (error) {
        showToast('error', 'Error', `Failed to open folder: ${error}`);
    }
}

async function openExternal(url) {
    try {
        await open(url);
    } catch (error) {
        console.error('Failed to open URL:', error);
    }
}

// ============================================================================
// Logs
// ============================================================================

function addLog(level, message) {
    const output = document.getElementById('logsOutput');
    const autoScroll = document.getElementById('autoScrollLogs').checked;
    
    const time = new Date().toLocaleTimeString();
    const entry = document.createElement('div');
    entry.className = `log-entry ${level}`;
    entry.innerHTML = `
        <span class="log-time">[${time}]</span>
        <span class="log-message">${escapeHtml(message)}</span>
    `;
    
    output.appendChild(entry);
    
    // Keep only last 500 entries
    while (output.children.length > 500) {
        output.removeChild(output.firstChild);
    }
    
    if (autoScroll) {
        output.scrollTop = output.scrollHeight;
    }
}

function clearLogs() {
    const output = document.getElementById('logsOutput');
    output.innerHTML = `
        <div class="log-entry info">
            <span class="log-time">[${new Date().toLocaleTimeString()}]</span>
            <span class="log-message">Logs cleared</span>
        </div>
    `;
}

// ============================================================================
// Statistics
// ============================================================================

function updateStats() {
    document.getElementById('profilesCount').textContent = state.profiles.length;
    document.getElementById('activeSessionsCount').textContent = state.sessions.filter(s => 
        s.status === 'Connected' || s.status === 'Starting' || s.status === 'Reconnecting'
    ).length;
    
    // Count active tunnels
    let tunnelCount = 0;
    for (const session of state.sessions) {
        if (session.status === 'Connected') {
            const profile = state.profiles.find(p => p.name === session.profile_name);
            if (profile) {
                tunnelCount += profile.tunnels.length;
            }
        }
    }
    document.getElementById('tunnelsCount').textContent = tunnelCount;
}

// ============================================================================
// Modals
// ============================================================================

function showModal(id) {
    document.getElementById(id).classList.add('active');
}

function closeModal(id) {
    document.getElementById(id).classList.remove('active');
}

function showQuickConnect() {
    showModal('quickConnectModal');
}

function showConfirm(title, message, callback) {
    document.getElementById('confirmTitle').textContent = title;
    document.getElementById('confirmMessage').textContent = message;
    state.confirmCallback = callback;
    showModal('confirmModal');
}

function confirmAction() {
    if (state.confirmCallback) {
        state.confirmCallback();
        state.confirmCallback = null;
    }
    closeModal('confirmModal');
}

// ============================================================================
// Toast Notifications
// ============================================================================

function showToast(type, title, message) {
    const container = document.getElementById('toastContainer');
    
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.innerHTML = `
        <div class="toast-icon">
            ${getToastIcon(type)}
        </div>
        <div class="toast-content">
            <div class="toast-title">${escapeHtml(title)}</div>
            <div class="toast-message">${escapeHtml(message)}</div>
        </div>
    `;
    
    container.appendChild(toast);
    
    // Remove after 5 seconds
    setTimeout(() => {
        toast.style.animation = 'slideIn 0.3s ease reverse';
        setTimeout(() => toast.remove(), 300);
    }, 5000);
}

function getToastIcon(type) {
    switch (type) {
        case 'success':
            return '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 11-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>';
        case 'error':
            return '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>';
        case 'warning':
            return '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>';
        case 'info':
        default:
            return '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>';
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function formatAuth(auth) {
    if (auth === 'agent') return 'SSH Agent';
    if (auth === 'password') return 'Password';
    if (auth.startsWith('key:')) return 'Key File';
    return auth;
}

function formatTime(isoString) {
    try {
        const date = new Date(isoString);
        return date.toLocaleTimeString();
    } catch {
        return isoString;
    }
}

function getStatusClass(status) {
    const s = status.toLowerCase();
    if (s === 'connected') return 'connected';
    if (s === 'starting') return 'starting';
    if (s === 'reconnecting') return 'reconnecting';
    if (s === 'disconnected') return 'disconnected';
    if (s === 'failed') return 'failed';
    return '';
}

// ============================================================================
// Keyboard Shortcuts
// ============================================================================

document.addEventListener('keydown', (e) => {
    // Escape to close modals
    if (e.key === 'Escape') {
        document.querySelectorAll('.modal.active').forEach(modal => {
            modal.classList.remove('active');
        });
    }
    
    // Ctrl+N for new profile
    if (e.ctrlKey && e.key === 'n') {
        e.preventDefault();
        showCreateProfileModal();
    }
    
    // Ctrl+K for quick connect
    if (e.ctrlKey && e.key === 'k') {
        e.preventDefault();
        showQuickConnect();
    }
});
