# Railway Deployment Guide for SSH Portfolio

This guide will help you deploy your Rust-based SSH portfolio to Railway with the "No Credit Card" trial option.

## Prerequisites

1. **GitHub Account** - Your project must be connected to GitHub
2. **Railway Account** - Sign up at [railway.app](https://railway.app)
3. **GitHub Repository** - Your SSH portfolio code must be pushed to GitHub

## Phase 1: Prepare Your GitHub Repository

### 1. Ensure Your Repository is Ready

Make sure your repository contains:
- `Cargo.toml` and `Cargo.lock`
- `src/` directory with all Rust source files
- `Dockerfile` (already optimized for Railway)
- `railway.json` configuration file

### 2. Push to GitHub

```bash
# If not already done, initialize and push to GitHub
git init
git add .
git commit -m "Initial commit: SSH Portfolio with Railway deployment"
git branch -M main
git remote add origin https://github.com/your-username/your-repo-name.git
git push -u origin main
```

## Phase 2: Deploy to Railway

### 1. Connect GitHub Repository

1. Go to [railway.app](https://railway.app) and sign in with GitHub
2. Click "New Project" → "Deploy from GitHub repo"
3. Select your SSH portfolio repository
4. Click "Deploy"

### 2. Configure Environment Variables

After deployment starts, go to your project settings:

1. Click on your project in Railway dashboard
2. Go to "Settings" → "Variables"
3. Add the following environment variable:
   ```
   PORT = 2222
   ```

### 3. Configure Build Settings

1. Go to "Settings" → "Builds"
2. Ensure the build command is set to:
   ```
   cargo build --release
   ```
3. Ensure the start command is set to:
   ```
   /usr/local/bin/ssh-portfolio
   ```

## Phase 3: Access Your SSH Portfolio

### 1. Get Your Railway URL

Once deployment is complete, Railway will provide you with a URL like:
```
your-project-name-production.up.railway.app
```

### 2. Connect via SSH

Since Railway uses port 2222 for SSH (not the standard port 22), connect with:

```bash
ssh your-project-name-production.up.railway.app -p 2222
```

**Note:** No username or password required - the connection is completely open for portfolio access.

## Enhanced Features Added

### 1. Custom MOTD (Message of the Day)

Your portfolio now displays an industrial-themed welcome banner when users connect:

```
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║  ██████╗ ██████╗  ██████╗ ██╗  ██╗██╗   ██╗███████╗██████╗  ║
║  ██╔══██╗██╔══██╗██╔═══██╗╚██╗██╔╝╚██╗ ██╔╝██╔════╝██╔══██╗ ║
║  ██████╔╝██████╔╝██║   ██║ ╚███╔╝  ╚████╔╝ █████╗  ██████╔╝ ║
║  ██╔═══╝ ██╔══██╗██║   ██║ ██╔██╗   ╚██╔╝  ██╔══╝  ██╔══██╗ ║
║  ██║     ██║  ██║╚██████╔╝██╔╝ ██╗   ██║   ███████╗██║  ██║ ║
║  ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝ ║
║                                                              ║
║              Redwan's Industrial Terminal                    ║
║              Authorized Access Only                          ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

System Status: ONLINE
Connection Type: SSH
Security Level: PUBLIC ACCESS
Last Login: 2024-01-01 12:00:00 UTC
```

### 2. Exit Animation

When users quit with 'q', they see a system shutdown animation:

```
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║  ██████╗ ███████╗███╗   ██╗██████╗  ██████╗ ██╗   ██╗       ║
║  ██╔══██╗██╔════╝████╗  ██║██╔══██╗██╔═══██╗╚██╗ ██╔╝       ║
║  ██████╔╝█████╗  ██╔██╗ ██║██║  ██║██║   ██║ ╚████╔╝        ║
║  ██╔══██╗██╔══╝  ██║╚██╗██║██║  ██║██║   ██║  ╚██╔╝         ║
║  ██║  ██║███████╗██║ ╚████║██████╔╝╚██████╔╝   ██║          ║
║  ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝╚═════╝  ╚═════╝    ╚═╝          ║
║                                                              ║
║              System Shutdown Initiated                       ║
║              Connection Terminating                          ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

Logging out...
Session ended.
```

### 3. Security Enhancements

- **Non-root user**: Application runs as a dedicated `portfolio` user
- **Minimal dependencies**: Only essential packages installed
- **Secure SSH**: Uses Ed25519 keys with aws-lc-rs for cryptography

## Troubleshooting

### Connection Issues

**Problem**: "Connection refused" or "Connection timed out"
**Solution**: 
1. Check that your Railway project is running (green status)
2. Verify the PORT environment variable is set to 2222
3. Ensure you're using the correct Railway URL

**Problem**: "Permission denied" errors
**Solution**: The application runs as non-root user - this is intentional for security

### Build Issues

**Problem**: Docker build fails
**Solution**:
1. Ensure `Dockerfile` is in repository root
2. Check that `Cargo.toml` has all dependencies
3. Verify Rust version compatibility

**Problem**: Application crashes on startup
**Solution**:
1. Check Railway logs for error details
2. Verify PORT environment variable
3. Ensure all required dependencies are in `Cargo.toml`

### SSH Connection Issues

**Problem**: SSH connection drops immediately
**Solution**:
1. Check that PTY is properly requested by your SSH client
2. Verify terminal dimensions are supported
3. Try with a different SSH client

## Railway-Specific Optimizations

### 1. Dockerfile Optimizations

Your `Dockerfile` includes:
- Multi-stage build for smaller images
- Non-root user for security
- Minimal package installation
- Proper file permissions

### 2. Environment Configuration

- **PORT**: Set to 2222 to match your application
- **Build Command**: `cargo build --release`
- **Start Command**: `/usr/local/bin/ssh-portfolio`

### 3. Resource Usage

- **Memory**: Optimized for Railway's free tier
- **CPU**: Efficient Rust binary with minimal overhead
- **Storage**: Small Docker image (~50MB)

## Cost and Limits

### Railway Free Tier
- **$5 Trial Credit**: No credit card required for new accounts
- **Resource Limits**: Shared CPU, 512MB RAM
- **Deployment Limits**: 1 project on free tier
- **Build Time**: Included in trial credit

### After Trial
- **Pricing**: $5/month for basic resources
- **Scaling**: Can scale up as needed
- **Multiple Projects**: Available on paid plans

## Next Steps

1. **Custom Domain**: Set up a custom domain for easier access
2. **Monitoring**: Add logging and monitoring
3. **Backups**: Set up automated backups
4. **Scaling**: Configure auto-scaling for high traffic

## Support

- **Railway Documentation**: https://docs.railway.app
- **SSH Portfolio Source**: This repository
- **Contact**: reahs302444@gmail.com

## Quick Reference

```bash
# Connect to your deployed SSH portfolio
ssh your-project-name-production.up.railway.app -p 2222

# Check Railway logs
railway logs

# View deployment status
railway status

# Set environment variables
railway variables set PORT=2222