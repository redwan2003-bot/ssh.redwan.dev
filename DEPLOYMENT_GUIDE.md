# SSH Portfolio Deployment Guide

This guide will help you deploy your SSH portfolio to Fly.io.

## Prerequisites

1. **Fly.io Account**: Sign up at [fly.io](https://fly.io)
2. **Payment Method**: Add a payment method to your Fly.io account (required for deployment)
3. **Fly CLI**: Already installed on your system

## Deployment Steps

### Step 1: Add Payment Method
1. Visit: https://fly.io/dashboard/personal/billing
2. Add a payment method to your account
3. Return to this terminal

### Step 2: Deploy to Fly.io
Run the following command to deploy your SSH portfolio:

```bash
& "$env:USERPROFILE\.fly\bin\flyctl.exe" launch --name ssh-redwan-dev --region cdg
```

**Note**: The app name `ssh-redwan-dev` is already configured in your `fly.toml` file.

### Step 3: Test Your Deployment
Once deployed, test your SSH portfolio:

```bash
ssh ssh-redwan-dev.fly.dev
```

## Alternative: Manual Deployment

If the launch command doesn't work, you can deploy manually:

```bash
# Deploy the app
& "$env:USERPROFILE\.fly\bin\flyctl.exe" deploy

# Check app status
& "$env:USERPROFILE\.fly\bin\flyctl.exe" status

# View logs
& "$env:USERPROFILE\.fly\bin\flyctl.exe" logs
```

## Automated Deployment with GitHub Actions

The `.github/workflows/deploy.yml` file is already configured for automated deployment. To use it:

1. **Generate Fly.io API Token**:
   ```bash
   & "$env:USERPROFILE\.fly\bin\flyctl.exe" auth token
   ```

2. **Add Token to GitHub**:
   - Go to your GitHub repository settings
   - Navigate to "Secrets and variables" > "Actions"
   - Add a new repository secret named `FLY_API_TOKEN` with the token value

3. **Push to Main Branch**:
   ```bash
   git add .
   git commit -m "Setup automated deployment"
   git push origin main
   ```

## Troubleshooting

### Port Issues
- Your app runs on port 2222 internally
- Fly.io maps external port 22 to internal port 2222
- No SSH authentication is required

### Docker Build Issues
If Docker build fails:
1. Ensure Docker is running
2. Check Dockerfile syntax
3. Verify Rust dependencies in `Cargo.toml`

### SSH Connection Issues
- Ensure port 22 is not blocked by firewall
- Check Fly.io app status: `fly status`
- View logs for errors: `fly logs`

## Security Notes

- The SSH server generates ephemeral keys for each deployment
- No authentication is required (public portfolio)
- Each connection gets an isolated session
- The app runs with restricted permissions

## Performance

- App deployed to Paris region (CDG) for optimal performance in Europe
- Shared CPU with 1GB RAM (sufficient for terminal UI)
- Multiple concurrent connections supported

## Next Steps

1. Add your custom SSH host key (optional):
   ```bash
   # Generate a key
   ssh-keygen -t ed25519 -f ssh_host_key
   
   # Add to Fly.io secrets
   & "$env:USERPROFILE\.fly\bin\flyctl.exe" secrets set SSH_HOST_KEY="$(cat ssh_host_key)"
   ```

2. Monitor your app:
   - View metrics: `fly metrics`
   - Check health: `fly status`
   - View logs: `fly logs`

3. Scale if needed:
   ```bash
   # Add more instances
   & "$env:USERPROFILE\.fly\bin\flyctl.exe" scale count 2
   ```

## Support

- Fly.io Documentation: https://fly.io/docs/
- SSH Portfolio Source: This repository
- Contact: reahs302444@gmail.com