# Noto
Vel/Veltext is a fast and easy to use Text-Editor like Notepad++ written in Rust for native and web

## Vel? Why is the repo called Noto then?
"Vel" or "Valtext" is the editor itself. Noto is the project containing Vel and a web-server that can be used to run Vel (like codespaces but on the browser)

## Structure
Vel for web is run in the Browser which then communicates via websockets to the runtime, which actions it should take.
The runtime 'Noto' exists only when build for web.

Vel for native is solely run on the maschine without any "extra" backend like the web app.