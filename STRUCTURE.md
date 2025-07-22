# Noto Project Structure

```
  noto/
  ├── Cargo.toml                    # Workspace configuration
  ├── README.md
  ├── docs/
  │   ├── architecture.md
  │   ├── protocol.md
  │   └── deployment.md
  │
  ├── crates/
  │   ├── vel-core/                 # 🧠 Core editor logic (platform-agnostic)
  │   │   ├── Cargo.toml
  │   │   └── src/
  │   │       ├── lib.rs
  │   │       ├── editor/           # Editor state management
  │   │       │   ├── mod.rs
  │   │       │   ├── document.rs   # Document/buffer management
  │   │       │   ├── cursor.rs     # Cursor and selection handling
  │   │       │   ├── history.rs    # Undo/redo system
  │   │       │   └── workspace.rs  # Multi-file workspace
  │   │       ├── syntax/           # Syntax highlighting & parsing
  │   │       │   ├── mod.rs
  │   │       │   ├── treesitter.rs # Tree-sitter integration
  │   │       │   └── themes.rs     # Color themes
  │   │       ├── search/           # Search functionality
  │   │       │   ├── mod.rs
  │   │       │   ├── local.rs      # In-file search
  │   │       │   └── query.rs      # Search query parsing
  │   │       ├── lsp/              # Language Server Protocol
  │   │       │   ├── mod.rs
  │   │       │   ├── client.rs     # LSP client interface
  │   │       │   └── types.rs      # LSP message types
  │   │       └── utils/
  │   │           ├── mod.rs
  │   │           ├── rope.rs       # Text rope data structure
  │   │           └── encoding.rs   # Text encoding handling
  │   │
  │   ├── vel-render/               # 🎨 Cross-platform rendering (wgpu)
  │   │   ├── Cargo.toml
  │   │   └── src/
  │   │       ├── lib.rs
  │   │       ├── renderer.rs       # Main renderer
  │   │       ├── text/             # Text rendering
  │   │       │   ├── mod.rs
  │   │       │   ├── glyph.rs      # Glyph management
  │   │       │   └── layout.rs     # Text layout
  │   │       ├── ui/               # UI rendering primitives
  │   │       │   ├── mod.rs
  │   │       │   ├── widgets.rs    # Basic UI widgets
  │   │       │   └── layout.rs     # Layout system
  │   │       └── shaders/          # WGSL shaders
  │   │           ├── text.wgsl
  │   │           └── ui.wgsl
  │   │
  │   ├── vel-gui/                  # 🖼️ Retained GUI framework
  │   │   ├── Cargo.toml
  │   │   └── src/
  │   │       ├── lib.rs
  │   │       ├── widget/           # Widget system
  │   │       │   ├── mod.rs
  │   │       │   ├── text_editor.rs
  │   │       │   ├── file_tree.rs
  │   │       │   ├── menu_bar.rs
  │   │       │   └── status_bar.rs
  │   │       ├── layout/           # Layout management
  │   │       │   ├── mod.rs
  │   │       │   └── flex.rs       # Flexbox-like layout
  │   │       └── event/            # Event handling
  │   │           ├── mod.rs
  │   │           └── keyboard.rs
  │   │
  │   ├── noto-protocol/            # 🔌 WebSocket protocol definitions
  │   │   ├── Cargo.toml
  │   │   └── src/
  │   │       ├── lib.rs
  │   │       ├── messages/         # Protocol messages
  │   │       │   ├── mod.rs
  │   │       │   ├── file_ops.rs   # File operations
  │   │       │   ├── project.rs    # Project management
  │   │       │   ├── lsp.rs        # LSP message forwarding
  │   │       │   ├── build.rs      # Build system messages
  │   │       │   └── search.rs     # Search messages
  │   │       └── codec.rs          # Message serialization
  │   │
  │   ├── vel-native/               # 🖥️ Native desktop application
  │   │   ├── Cargo.toml
  │   │   └── src/
  │   │       ├── main.rs
  │   │       ├── app.rs            # Main application
  │   │       ├── window.rs         # Window management
  │   │       ├── fs/               # Local filesystem operations
  │   │       │   ├── mod.rs
  │   │       │   ├── project.rs    # Local project management
  │   │       │   └── watcher.rs    # File system watching
  │   │       └── lsp/              # Local LSP management
  │   │           ├── mod.rs
  │   │           └── server.rs     # LSP server spawning
  │   │
  │   ├── vel-web/                  # 🌐 WebAssembly web client
  │   │   ├── Cargo.toml
  │   │   ├── index.html
  │   │   ├── style.css
  │   │   └── src/
  │   │       ├── lib.rs
  │   │       ├── app.rs            # Web application entry
  │   │       ├── websocket.rs      # WebSocket client
  │   │       └── bindings/         # Web API bindings
  │   │           ├── mod.rs
  │   │           └── dom.rs        # DOM manipulation
  │   │
  │   ├── noto-server/              # 🖧 Web backend server
  │   │   ├── Cargo.toml
  │   │   └── src/
  │   │       ├── main.rs
  │   │       ├── server.rs         # HTTP/WebSocket server
  │   │       ├── auth/             # Authentication
  │   │       │   ├── mod.rs
  │   │       │   ├── users.rs      # User management
  │   │       │   ├── sessions.rs   # Session handling
  │   │       │   └── crypto.rs     # Password hashing
  │   │       ├── projects/         # Project management
  │   │       │   ├── mod.rs
  │   │       │   ├── manager.rs    # Project lifecycle
  │   │       │   ├── fs.rs         # Server filesystem ops
  │   │       │   └── permissions.rs# Access control
  │   │       ├── lsp/              # Language Server management
  │   │       │   ├── mod.rs
  │   │       │   ├── manager.rs    # LSP server lifecycle
  │   │       │   └── proxy.rs      # LSP message proxying
  │   │       ├── build/            # Build system integration
  │   │       │   ├── mod.rs
  │   │       │   ├── sandbox.rs    # Sandboxed execution
  │   │       │   └── runners/      # Specific build runners
  │   │       │       ├── mod.rs
  │   │       │       ├── cargo.rs
  │   │       │       ├── npm.rs
  │   │       │       └── generic.rs
  │   │       ├── search/           # Server-side search
  │   │       │   ├── mod.rs
  │   │       │   └── indexer.rs    # File indexing
  │   │       └── websocket/        # WebSocket handling
  │   │           ├── mod.rs
  │   │           ├── handler.rs    # Message routing
  │   │           └── session.rs    # Connection management
  │   │
  │   └── noto-sandbox/             # 🔒 Build system sandboxing
  │       ├── Cargo.toml
  │       └── src/
  │           ├── lib.rs
  │           ├── container.rs      # Container/chroot management
  │           ├── limits.rs         # Resource limiting
  │           └── monitor.rs        # Process monitoring
  │
  ├── assets/                       # Static assets
  │   ├── fonts/
  │   ├── icons/
  │   └── themes/
  │       ├── dark.toml
  │       └── light.toml
  │
  ├── scripts/                      # Build and deployment scripts
  │   ├── build-web.sh
  │   ├── build-native.sh
  │   └── deploy.sh
  │
  └── tests/                        # Integration tests
      ├── protocol/
      ├── editor/
      └── e2e/
```

## Key Design Decisions

### 🔄 Shared Core Architecture

- **`vel-core`**: Contains all editor logic, completely platform-agnostic
- **`vel-render`**: Shared wgpu-based rendering for both native and web
- **`vel-gui`**: Retained GUI framework built on top of vel-render
- **`noto-protocol`**: Shared message definitions between client and server

### 🌐 Web Architecture

- **`vel-web`**: Thin WASM wrapper that connects vel-core to WebSocket
- **`noto-server`**: Handles authentication, project management, LSP proxying, and builds
- Auto-save implemented via WebSocket messages to server
- File tree mirrored on client for instant UI updates

### 🖥️ Native Architecture

- **`vel-native`**: Direct integration with vel-core, no network layer
- Uses same rendering and GUI code as web version
- Directly manages LSP servers and file system

### 🔒 Security & Sandboxing

- **`noto-sandbox`**: Handles safe execution of build scripts
- User isolation through filesystem permissions
- Salted password hashing in authentication layer

### 📦 Deployment

- Native: Single binary with embedded assets
- Server: Single binary that can serve static web assets
- Web client: WASM + assets served by noto-server

This structure allows maximum code reuse while maintaining clean separation between platforms. The editor logic is identical between native and web, with only the I/O layer differing.
