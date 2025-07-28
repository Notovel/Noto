#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::content;
use std::env;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"

<!DOCTYPE html>

<html>
<head>
    <meta charset="utf-8">
    <title>Rust WASM App with Rocket + TLS</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }
        .secure { color: green; }
        .insecure { color: orange; }
        button {
            background: #007acc;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 5px;
            cursor: pointer;
            font-size: 16px;
        }
        button:hover {
            background: #005a9e;
        }
        #output {
            margin-top: 20px;
            padding: 10px;
            background: #f5f5f5;
            border-radius: 5px;
            min-height: 50px;
        }
        .security-info {
            background: #e8f4fd;
            border: 1px solid #bee5eb;
            border-radius: 5px;
            padding: 10px;
            margin: 10px 0;
        }
    </style>
</head>
<body>
    <h1>🚀 Rust WASM + Rocket Server</h1>


<div class="security-info">
    <h3 id="security-status">🔍 Checking connection security...</h3>
    <p id="security-details"></p>
</div>

<p>This WASM app was automatically built by the Rust build script!</p>

<button id="greet-button">Say Hello</button>
<button id="add-button">Add Numbers (5 + 3)</button>

<div id="output"></div>

<script type="module">
    import init, { greet, add } from '/static/vel-web.js'; 

    const isSecure = location.protocol === 'https:';
    const statusElement = document.getElementById('security-status');
    const detailsElement = document.getElementById('security-details');
    
    if (isSecure) {
        statusElement.innerHTML = '🔒 <span class="secure">Secure HTTPS Connection</span>';
        detailsElement.textContent = 'Your connection is encrypted and secure.';
    } else {
        statusElement.innerHTML = '⚠️ <span class="insecure">HTTP Connection</span>';
        detailsElement.textContent = 'Running in development mode. In production, use HTTPS for security.';
    }
    
    
    async function run() {
        try {
            await init();
            console.log('WASM module loaded successfully!');
            
            const output = document.getElementById('output');
            output.innerHTML = '<p>✅ WASM module loaded and ready!</p>';
            
            document.getElementById('greet-button').addEventListener('click', () => {
                const result = greet('Rocket + WebAssembly + TLS');
                output.innerHTML = '<p>Greeting: ${result}</p>';
            });
            
            document.getElementById('add-button').addEventListener('click', () => {
                const result = add(5, 3);
                output.innerHTML = '<p>5 + 3 = ${result}</p>';
            });
            
        } catch (error) {
            console.error('Failed to load WASM module:', error);
            document.getElementById('output').innerHTML = 
                '<p style="color: red;">❌ Failed to load WASM: ${error}</p>';
        }
    }
    
    run();
</script>


</body>
</html>
    "#,
    )
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

fn find_cert_files() -> Option<(String, String)> {
    // Try multiple common locations for certificates
    let cert_locations = [
        ("cert.pem", "key.pem"),
        ("server.crt", "server.key"),
        ("certificate.pem", "private.key"),
        ("ssl/cert.pem", "ssl/key.pem"),
        ("tls/cert.pem", "tls/key.pem"),
        ("/etc/ssl/certs/server.crt", "/etc/ssl/private/server.key"),
    ];

    // Also check environment variables
    if let (Ok(cert), Ok(key)) = (env::var("TLS_CERT_PATH"), env::var("TLS_KEY_PATH")) {
        if Path::new(&cert).exists() && Path::new(&key).exists() {
            return Some((cert, key));
        }
    }

    // Check common locations
    for (cert_path, key_path) in &cert_locations {
        if Path::new(cert_path).exists() && Path::new(key_path).exists() {
            return Some((cert_path.to_string(), key_path.to_string()));
        }
    }

    None
}

fn get_exec_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    Ok(dir)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let exec_dir = get_exec_path().expect("Server should have a path!");
    let mut wasm_pkg_dir = exec_dir;
    wasm_pkg_dir.push("static");
    if !wasm_pkg_dir.exists() {
        wasm_pkg_dir = PathBuf::from(option_env!("WASM_PKG_DIR").unwrap_or("wasm-app/pkg"));
        if !wasm_pkg_dir.exists() {
            panic!("❌ Did not find any folder for the static files!");
        }
    }

    println!("🚀 Starting Rocket server...");
    println!("📦 WASM files served from: {}", wasm_pkg_dir.display());

    let mut rocket_config = rocket::Config::default();

    match find_cert_files() {
        Some((cert_path, key_path)) => {
            println!("🔒 TLS certificates found - enabling HTTPS");
            println!("   Certificate: {}", cert_path);
            println!("   Private key: {}", key_path);

            rocket_config.tls = Some(rocket::config::TlsConfig::from_paths(cert_path, key_path));

            // Use environment variable for port, default to 8443
            rocket_config.port = env::var("PORT")
                .unwrap_or_else(|_| "8443".to_string())
                .parse()
                .unwrap_or(8443);

            // Use environment variable for address, default to 0.0.0.0 for production
            rocket_config.address = env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string())
                .parse()
                .unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

            println!(
                "🌐 Server will run on https://{}:{}",
                rocket_config.address, rocket_config.port
            );
        }
        None => {
            println!("⚠️  No TLS certificates found");
            println!("   Looked for: cert.pem/key.pem, server.crt/server.key, etc.");
            println!(
                "   Set TLS_CERT_PATH and TLS_KEY_PATH environment variables to specify custom locations"
            );
            println!("   Running in HTTP mode");

            rocket_config.port = env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000);

            rocket_config.address = env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string())
                .parse()
                .unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

            println!(
                "🌐 Server will run on http://{}:{}",
                rocket_config.address, rocket_config.port
            );
        }
    }

    rocket::custom(rocket_config)
        .mount("/", routes![index, health])
        .mount("/static", FileServer::from(wasm_pkg_dir))
}
