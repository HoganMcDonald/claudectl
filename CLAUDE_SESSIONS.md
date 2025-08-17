# Claude Code Integration

claudectl now automatically starts Claude Code sessions when creating new worktrees, providing seamless integration between worktree management and coding assistance.

## Automatic Session Startup

When you create a new worktree with `claudectl new`, it will:

1. **Create the git worktree** with a fresh branch
2. **Start Claude Code** in the worktree directory
3. **Run with security flags** for immediate productivity
4. **Use container isolation** for safety

### Default Configuration

Claude Code sessions are started with these settings:
- `--dangerously-skip-permissions` - Skip permission dialogs for faster startup
- `--container` - Run in container mode for isolation
- **Background process** - Runs independently of the terminal

## Session Management Commands

### `claudectl new [name]`
Creates a new worktree and starts a Claude Code session.

```bash
claudectl new feature-auth    # Named session
claudectl new                 # Auto-generated name (e.g., "brave-penguin")
```

### `claudectl list`
Lists all created sessions and shows which ones have active Claude Code sessions.

```bash
claudectl list
```

Example output:
```
Sessions for project "myapp":

Name           Branch         Commit   Status         Last Commit                    
─────────────  ─────────────  ───────  ─────────────  ───────────────────────────────
brave-penguin  brave-penguin  a1b2c3d  clean, claude  fix: resolve login issue       
swift-fox      swift-fox      a1b2c3d  current, dirty wip: dashboard improvements    

Currently in session: swift-fox

Switch to a session:
  cd ~/.claudectl/projects/myapp/brave-penguin  # brave-penguin
```

**Status meanings:**
- `claude` - Claude Code session is running for this session
- `clean` - No uncommitted changes
- `dirty` - Has uncommitted changes  
- `current` - You are currently in this session

Note: The main repository is not shown in the list - only created sessions appear.

### `claudectl rm <name>`
Removes both the worktree and stops any associated Claude Code session.

```bash
claudectl rm brave-penguin              # Normal removal
claudectl rm brave-penguin --force      # Force removal with uncommitted changes
```

## Session Persistence

- Session information is stored in `~/.claudectl/sessions.json`
- Sessions are automatically cleaned up if processes die
- Sessions persist across system restarts (as long as Claude Code is running)

## Manual Claude Code Usage

If you prefer to start Claude Code manually or if the automatic session stops:

1. Navigate to the worktree: `cd ~/.claudectl/projects/myproject/session-name`
2. Start Claude Code manually: `claude .`

Note: Manual sessions won't be tracked by claudectl.

## Troubleshooting

### Claude Code Not Found
If you see "Claude Code is not available", install it first:
```bash
# Install Claude Code
curl -L https://claude.ai/install.sh | bash
```

### Session Appears Dead
Sessions are automatically cleaned up when processes die. If a session shows as "cleaned up dead session", the process has stopped.

### Permission Issues
Claude Code runs with `--dangerously-skip-permissions` by default. If you prefer to review permissions:
1. Stop the session: `claudectl stop-session <name>`
2. Start manually: `cd <worktree-path> && claude .`

## Security Considerations

- **Container Mode**: Provides isolation from your host system
- **Skip Permissions**: Enables faster startup but bypasses permission dialogs
- **Background Process**: Runs independently and won't block your terminal

For maximum security, you can disable automatic startup and start Claude Code manually with your preferred flags.