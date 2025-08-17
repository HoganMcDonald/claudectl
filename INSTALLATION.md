# Installation Guide

## NPM Global Installation (Recommended)

Install claudectl globally to get automatic tab completion setup:

```bash
npm install -g claudectl
```

**What happens during installation:**
- 📦 claudectl CLI is installed globally
- 🔧 Tab completion is automatically configured for your shell
- ✅ Ready to use immediately after restarting your shell

**Supported shells:** Bash, Zsh, Fish

## Manual Tab Completion Setup

If you installed locally or want to reinstall completion:

```bash
# Install tab completion
claudectl install-completion

# Remove tab completion
claudectl uninstall-completion
```

## Verification

Test that tab completion is working:

```bash
claudectl <TAB>     # Should show: init, new, list, rm, install-completion, uninstall-completion
claudectl rm <TAB>  # Should show available session names
```

## Troubleshooting

### Tab completion not working

1. **Restart your shell** or source your config:
   ```bash
   # Bash
   source ~/.bashrc
   
   # Zsh
   source ~/.zshrc
   
   # Fish
   source ~/.config/fish/config.fish
   ```

2. **Reinstall completion**:
   ```bash
   claudectl uninstall-completion
   claudectl install-completion
   ```

3. **Check if completion is installed**:
   ```bash
   # Look for claudectl completion in your shell config
   grep -n "claudectl" ~/.bashrc ~/.zshrc ~/.config/fish/config.fish 2>/dev/null
   ```

### Installation fails

If automatic installation fails during `npm install -g`, you can manually install completion:

```bash
claudectl install-completion
```

## Uninstallation

When you uninstall claudectl globally, tab completion is automatically removed:

```bash
npm uninstall -g claudectl
```

The uninstall process will:
- 🧹 Remove tab completion from your shell configuration  
- 🗑️ Clean up the claudectl binary

## Local Development

For local development, install dependencies and build:

```bash
npm install
npm run build
```

Tab completion is only automatically installed with global installations. For local development, use:

```bash
node dist/cli.js install-completion
```