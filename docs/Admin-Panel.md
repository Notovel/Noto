# Noto Server Admin Panel Architecture

## 🔐 Admin Authentication & Access

### Admin User Setup

```rust
// First-time setup creates default admin
// Password must be changed on first login
pub struct AdminSetup {
    pub default_admin: String,        // "admin"
    pub default_password: String,     // Generated random password
    pub force_password_change: bool,  // true
}

// Admin user in database
pub struct AdminUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: AdminRole,              // SuperAdmin, UserManager, ServerManager
    pub last_login: Option<DateTime<Utc>>,
    pub password_changed: DateTime<Utc>,
    pub force_password_change: bool,
}

pub enum AdminRole {
    SuperAdmin,    // Full access
    UserManager,   // User management only
    ServerManager, // Server settings only
}
```

### Admin Routes (`noto-server/src/admin/handlers.rs`)

```rust
use axum::{Router, middleware, routing::{get, post, put, delete}};

pub fn admin_routes() -> Router {
    Router::new()
        // Admin panel UI
        .route("/admin", get(serve_admin_panel))
        .route("/admin/login", get(admin_login_page))
        .route("/admin/login", post(admin_login))
        
        // Dashboard & Overview
        .route("/admin/api/dashboard", get(dashboard_stats))
        .route("/admin/api/system-info", get(system_info))
        .route("/admin/api/activity-log", get(activity_log))
        
        // User Management
        .route("/admin/api/users", get(list_users))
        .route("/admin/api/users", post(create_user))
        .route("/admin/api/users/:id", get(get_user))
        .route("/admin/api/users/:id", put(update_user))
        .route("/admin/api/users/:id", delete(delete_user))
        .route("/admin/api/users/:id/reset-password", post(reset_user_password))
        .route("/admin/api/users/:id/projects", get(user_projects))
        
        // Project Management  
        .route("/admin/api/projects", get(list_all_projects))
        .route("/admin/api/projects/:id", get(get_project_details))
        .route("/admin/api/projects/:id", delete(delete_project))
        .route("/admin/api/projects/:id/transfer", post(transfer_project))
        .route("/admin/api/projects/cleanup", post(cleanup_orphaned_projects))
        
        // Server Configuration
        .route("/admin/api/config", get(get_server_config))
        .route("/admin/api/config", put(update_server_config))
        .route("/admin/api/config/validate", post(validate_config))
        .route("/admin/api/config/backup", get(backup_config))
        .route("/admin/api/config/restore", post(restore_config))
        
        // Language Server Management
        .route("/admin/api/lsp/servers", get(list_lsp_servers))
        .route("/admin/api/lsp/servers", post(add_lsp_server))
        .route("/admin/api/lsp/servers/:id", put(update_lsp_server))
        .route("/admin/api/lsp/servers/:id", delete(remove_lsp_server))
        .route("/admin/api/lsp/active", get(active_lsp_instances))
        
        // Build System Management
        .route("/admin/api/build/runners", get(list_build_runners))
        .route("/admin/api/build/runners", post(add_build_runner))
        .route("/admin/api/build/active", get(active_builds))
        .route("/admin/api/build/history", get(build_history))
        .route("/admin/api/build/limits", get(get_build_limits))
        .route("/admin/api/build/limits", put(update_build_limits))
        
        // System Logs & Monitoring
        .route("/admin/api/logs", get(system_logs))
        .route("/admin/api/logs/export", get(export_logs))
        .route("/admin/api/metrics", get(system_metrics))
        .route("/admin/api/health", get(health_check))
        
        // Security & Maintenance
        .route("/admin/api/security/sessions", get(active_sessions))
        .route("/admin/api/security/sessions/:id", delete(terminate_session))
        .route("/admin/api/maintenance/cleanup", post(cleanup_temp_files))
        .route("/admin/api/maintenance/backup", post(create_backup))
        .route("/admin/api/maintenance/restart", post(restart_server))
        
        // Middleware for admin authentication
        .layer(middleware::from_fn(require_admin_auth))
}
```

## 🖥️ Admin Panel UI Structure

### Admin Panel HTML (`assets/admin/admin.html`)

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Noto Server Administration</title>
    <link rel="stylesheet" href="/admin/static/admin.css">
    <script src="/admin/static/charts.js"></script>
</head>
<body>
    <div id="admin-app">
        <nav class="sidebar">
            <h2>Noto Admin</h2>
            <ul>
                <li><a href="#dashboard" class="nav-link active">Dashboard</a></li>
                <li><a href="#users" class="nav-link">Users</a></li>
                <li><a href="#projects" class="nav-link">Projects</a></li>
                <li><a href="#settings" class="nav-link">Server Settings</a></li>
                <li><a href="#lsp" class="nav-link">Language Servers</a></li>
                <li><a href="#builds" class="nav-link">Build System</a></li>
                <li><a href="#logs" class="nav-link">Logs & Monitoring</a></li>
                <li><a href="#security" class="nav-link">Security</a></li>
                <li><a href="#maintenance" class="nav-link">Maintenance</a></li>
            </ul>
            <div class="user-info">
                <span id="admin-user"></span>
                <button id="logout">Logout</button>
            </div>
        </nav>
        
        <main class="main-content">
            <div id="page-content">
                <!-- Dynamic content loaded here -->
            </div>
        </main>
    </div>
    
    <script src="/admin/static/admin.js"></script>
</body>
</html>
```

## 📊 Dashboard & Overview

### System Dashboard Data

```rust
#[derive(Serialize)]
pub struct DashboardStats {
    pub server_info: ServerInfo,
    pub user_stats: UserStats,
    pub project_stats: ProjectStats,
    pub system_resources: SystemResources,
    pub recent_activity: Vec<ActivityLogEntry>,
    pub active_sessions: u32,
    pub build_queue_size: u32,
    pub lsp_instances: u32,
}

#[derive(Serialize)]
pub struct ServerInfo {
    pub version: String,
    pub uptime: Duration,
    pub start_time: DateTime<Utc>,
    pub config_file: String,
    pub data_directory: String,
}

#[derive(Serialize)]
pub struct UserStats {
    pub total_users: u32,
    pub active_today: u32,
    pub active_this_week: u32,
    pub new_this_month: u32,
}

#[derive(Serialize)]
pub struct ProjectStats {
    pub total_projects: u32,
    pub projects_by_language: HashMap<String, u32>,
    pub disk_usage: u64,
    pub largest_projects: Vec<ProjectSummary>,
}

#[derive(Serialize)]
pub struct SystemResources {
    pub cpu_usage: f32,
    pub memory_usage: MemoryUsage,
    pub disk_usage: DiskUsage,
    pub network_stats: NetworkStats,
}
```

## ⚙️ Server Configuration Management

### Configuration Structure (`noto-server/src/config/settings.rs`)

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub server: ServerSettings,
    pub security: SecuritySettings,
    pub users: UserSettings,
    pub projects: ProjectSettings,
    pub build: BuildSettings,
    pub lsp: LSPSettings,
    pub logging: LoggingSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,           // "0.0.0.0"
    pub port: u16,             // 8080
    pub max_connections: u32,   // 1000
    pub request_timeout: u64,   // 30 seconds
    pub websocket_timeout: u64, // 300 seconds
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecuritySettings {
    pub session_duration: u64,        // 24 hours
    pub max_login_attempts: u32,      // 5
    pub lockout_duration: u64,        // 15 minutes
    pub password_min_length: usize,   // 8
    pub require_special_chars: bool,  // true
    pub admin_session_duration: u64,  // 4 hours
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSettings {
    pub max_projects_per_user: Option<u32>, // None = unlimited
    pub max_project_size: u64,              // 1GB
    pub auto_cleanup_days: Option<u32>,     // 90 days
    pub backup_enabled: bool,               // true
    pub backup_interval: u64,               // 24 hours
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildSettings {
    pub enabled: bool,                    // true
    pub timeout: u64,                    // 10 minutes
    pub max_concurrent_builds: u32,      // 4
    pub max_cpu_percent: f32,           // 80.0
    pub max_memory_mb: u32,             // 512
    pub allowed_commands: Vec<String>,   // ["cargo", "npm", "make", etc.]
    pub sandbox_enabled: bool,           // true
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LSPSettings {
    pub enabled: bool,
    pub servers: HashMap<String, LSPServerConfig>, // language -> config
    pub timeout: u64,        // 30 seconds
    pub max_instances: u32,  // 10
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LSPServerConfig {
    pub command: String,
    pub args: Vec<String>,
    pub file_extensions: Vec<String>,
    pub enabled: bool,
}
```

## 🔧 Admin Panel Features

### 1. Dashboard

- **Server Overview**: Uptime, version, system resources
- **Real-time Metrics**: CPU, memory, disk usage charts
- **User Activity**: Login statistics, active sessions
- **Recent Activity**: Last 50 system events

### 2. User Management

- **User List**: Searchable, sortable table
- **User Details**: Projects, disk usage, login history
- **User Actions**: Create, edit, delete, reset password
- **Bulk Operations**: Mass user management

### 3. Project Management

- **Project Browser**: All projects across all users
- **Project Details**: Size, last activity, file count
- **Project Actions**: Transfer ownership, delete, backup
- **Cleanup Tools**: Remove orphaned projects

### 4. Server Configuration

- **Live Config Editor**: Modify settings with validation
- **Configuration Backup/Restore**: Save and restore configs
- **Language Server Management**: Add/remove/configure LSP servers
- **Build Runner Configuration**: Manage build tools and limits

### 5. Monitoring & Logs

- **System Logs**: Real-time log viewer with filtering
- **Build History**: Past builds with outputs and errors
- **Performance Metrics**: Historical charts and graphs
- **Health Checks**: System health monitoring

### 6. Security & Maintenance

- **Active Sessions**: View and terminate user sessions
- **Security Events**: Failed logins, suspicious activity
- **System Maintenance**: Cleanup tools, restart server
- **Backup Management**: Create and restore system backups

## 🚀 Admin Panel Access

### URL Structure

```
https://your-server:8080/admin          # Admin panel
https://your-server:8080/admin/login    # Admin login
https://your-server:8080/admin/api/*    # Admin API endpoints
```

### Security Features

- **Separate Admin Authentication**: Independent from user auth
- **Role-Based Access Control**: Different admin permission levels
- **Session Management**: Shorter sessions, automatic logout
- **Audit Logging**: All admin actions logged
- **Two-Factor Authentication**: Optional 2FA for admins

This admin panel gives you complete control over your Noto server, allowing you to manage users, monitor system health, configure settings, and maintain the server efficiently.