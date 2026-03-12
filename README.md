# ssh.redwan.dev

```
  _____          _                                _                                  _ 
 |  __ \        | |                         /\   | |                                | |
 | |__) |___  __| |_      ____ _ _ __      /  \  | |__  _ __ ___  _ __ ___   ___  __| |
 |  _  // _ \/ _` \ \ /\ / / _` | '_ \    / /\ \ | '_ \| '_ ` _ \| '_ ` _ \ / _ \/ _` |
 | | \ \  __/ (_| |\ V  V / (_| | | | |  / ____ \| | | | | | | | | | | | | |  __/ (_| |
 |_|  \_\___|\__,_| \_/\_/ \__,_|_| |_| /_/    \_\_| |_|_| |_| |_|_| |_| |_|\___|\__,_|
                                                                                       
                                                                                
```

A personal portfolio served entirely over SSH. No browser needed -- just open a terminal and connect.

```bash
ssh ballast.proxy.rlwy.net -p 29436
```

## What is this?

Instead of a traditional web portfolio, this project runs an SSH server that renders a fully interactive terminal UI (TUI) to anyone who connects. Built in Rust with [ratatui](https://github.com/ratatui/ratatui) and [russh](https://github.com/warp-tech/russh), it streams a rich interface directly to your terminal.

## Features

- **Typewriter intro animation** -- ASCII art banner revealed character by character with a blinking cursor
- **4 navigable tabs** -- About, Projects, Skills, Contact
- **Telescope-style project browser** -- split-pane layout with categories, descriptions, and tech stacks
- **Vim-style keybindings** -- `h`/`j`/`k`/`l`, `g`/`G`, `1`-`4` for direct tab jumps
- **Responsive** -- adapts to terminal resizing in real-time
- **Zero authentication** -- connect instantly, no login required
- **Per-client isolation** -- each connection gets its own independent session

## Keybindings

| Key | Action |
|---|---|
| `h` `l` / `Left` `Right` | Switch tabs |
| `j` `k` / `Up` `Down` | Scroll / select |
| `g` / `G` | Jump to top / bottom |
| `1` `2` `3` `4` | Jump to tab directly |
| `Tab` / `Shift-Tab` | Next / previous tab |
| `q` / `Ctrl-C` | Quit |

## Tech Stack

- **Rust** -- async runtime with [Tokio](https://tokio.rs)
- **russh** -- SSH protocol server with Ed25519 host keys
- **ratatui** + **crossterm** -- terminal UI framework
- **Docker** -- multi-stage build for minimal runtime image
- **Fly.io** -- deployed to the `cdg` (Paris) region

## Project Structure

```
src/
  main.rs       SSH server bootstrap and key generation
  app.rs        Application state (tabs, scroll, animation)
  handler.rs    SSH session management and keypress routing
  ui.rs         Ratatui rendering for all layouts
  content.rs    Static portfolio data (bio, projects, skills, contacts)
  theme.rs      Centralized color and style definitions
  terminal.rs   Bridge between ratatui and SSH channel via mpsc
```

## Running Locally

```bash
# Build and start the server (defaults to port 2222)
cargo run

# Connect from another terminal
ssh localhost -p 2222
```

Set a custom port with the `PORT` environment variable:

```bash
PORT=3333 cargo run
```

## Docker

```bash
docker build -t ssh-portfolio .
docker run -p 2222:22 ssh-portfolio
ssh localhost -p 2222
```

## Self-Hosting (Free)

If you want to host this for free without a credit card, you can self-host it on your own machine using a TCP tunnel.

### Option 1: Bore.pub (Easiest)
1. Run the portfolio:
   ```bash
   docker run -d --name ssh-portfolio -e PORT=2222 -p 2222:2222 redwanahmmed-portfolio:latest
   ```
2. Start the tunnel:
   ```bash
   docker run -it --init --rm ekzhang/bore local 2222 --to bore.pub --local-host host.docker.internal
   ```
3. Connect using the port provided by Bore:
   ```bash
   ssh -p [PORT] bore.pub
   ```

### Option 2: Cloudflare Tunnel (Professional)
If you own a domain, you can use Cloudflare Zero Trust to create a permanent tunnel.
1. Use the provided `docker-compose.yml`.
2. Set your `TUNNEL_TOKEN` in the environment.
3. Connect via `cloudflared access`.

## Deployment

## Author

**Redwan Ahmmed** -- Robotics-Hardware Specialist (R&D) & Backend Developer

- [Medium](https://medium.com/@reahs302444)
- [GitHub](https://github.com/redwan2003-bot)
- [reahs302444@gmail.com](mailto:reahs302444@gmail.com)
