# ClaudeCtl

[![CI](https://github.com/HoganMcDonald/claudectl/actions/workflows/ci.yml/badge.svg)](https://github.com/HoganMcDonald/claudectl/actions/workflows/ci.yml)

A powerful CLI tool for managing multiple parallel development sessions with Claude Code. ClaudeCtl creates isolated git worktrees for each feature or task, allowing you to seamlessly switch between different work contexts while maintaining clean, focused development environments.

## ✨ Features

- **🌿 Worktree Management**: Create isolated git worktrees for parallel development
- **🤖 Claude Integration**: Seamless integration with Claude Code sessions
- **📊 Interactive TUI**: Beautiful terminal interface for managing all your sessions
- **⚡ Tab Completion**: Full shell completion for all commands and session names
- **🎯 Session Tracking**: Keep track of active development sessions across projects
- **🔄 Idempotent Operations**: Safe, repeatable commands that won't break your workflow

## 🚀 Quick Start

### Installation

```bash
npm install -g claudectl
```

### Initialize a Project

```bash
cd your-git-project
claudectl init my-project
```

### Create a New Development Session

```bash
claudectl new feature-auth
# Creates a new worktree and starts Claude Code
```

### View All Sessions

```bash
claudectl list
```

### Switch to Existing Session

```bash
claudectl attach feature-auth
```

### Interactive Dashboard

```bash
claudectl tui
```

## 📋 Commands

| Command | Description |
|---------|-------------|
| `claudectl init [name]` | Initialize a new claudectl project |
| `claudectl new [name]` | Create a new worktree and Claude session |
| `claudectl list` | List all active sessions for current project |
| `claudectl attach <name>` | Attach to an existing session |
| `claudectl rm <name>` | Remove a session and its worktree |
| `claudectl tui` | Launch interactive terminal interface |

## 🎯 Use Cases

### Feature Development
```bash
claudectl new feature-user-auth
# Work on authentication feature in isolation
claudectl new feature-dashboard  
# Switch to dashboard feature without losing auth work
```

### Bug Fixes
```bash
claudectl new hotfix-login-bug
# Create dedicated environment for urgent fixes
```

### Experimentation
```bash
claudectl new experiment-new-api
# Try new approaches without affecting main development
```

## 🏗️ How It Works

ClaudeCtl leverages git worktrees to create separate working directories for each development session:

1. **Project Initialization**: Sets up a `.claudectl` directory structure
2. **Worktree Creation**: Creates isolated git worktrees from your main branch
3. **Session Management**: Tracks Claude Code sessions for each worktree
4. **State Persistence**: Maintains session information across restarts

```
~/.claudectl/projects/
└── your-project/
    ├── main/                 # Main repository
    ├── feature-auth/         # Feature worktree
    ├── feature-dashboard/    # Another feature worktree
    └── hotfix-login-bug/     # Hotfix worktree
```

## 🔧 Development

### Prerequisites

- Node.js 16+
- Git
- Claude Code CLI

### Local Development Setup

```bash
git clone https://github.com/your-username/claudectl
cd claudectl
npm install
npm run setup:local  # Builds and links globally
```

### Available Scripts

```bash
npm run build          # Build TypeScript
npm run dev            # Watch mode development
npm run test           # Run test suite
npm run lint           # Lint code
npm run format         # Format code
npm run typecheck      # Type checking
npm run setup:local    # Build and link globally
npm run setup:clean    # Unlink global installation
```

### Testing

```bash
npm test              # Run all tests
npm run test:ui       # Interactive test UI
npm run test:coverage # Coverage report
```

## 🏭 Architecture

ClaudeCtl is built with TypeScript and follows a modular architecture:

```
src/
├── commands/          # CLI command implementations
├── core/             # Core business logic
│   ├── handlers/     # Session and worktree handlers
│   ├── types/        # Type definitions
│   └── utils/        # Core utilities
├── tui/              # Terminal UI components (React/Ink)
│   ├── components/   # UI components
│   ├── hooks/        # Custom hooks
│   └── providers/    # Context providers
└── utils/            # Utility functions
```

### Key Technologies

- **TypeScript**: Type-safe development
- **Commander.js**: CLI framework
- **React/Ink**: Terminal UI framework
- **Tabtab**: Shell completion
- **Vitest**: Testing framework
- **Biome**: Linting and formatting

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Quick Contribution Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Install dependencies: `npm install`
4. Set up local development: `npm run setup:local`
5. Make your changes and add tests
6. Run the test suite: `npm test`
7. Commit your changes: `git commit -m 'feat: add amazing feature'`
8. Push to your branch: `git push origin feature/amazing-feature`
9. Open a Pull Request

### Development Standards

- Write tests for new features
- Follow conventional commit messages
- Ensure TypeScript types are properly defined
- Run `npm run check` before committing
- Update documentation for user-facing changes

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Claude AI for inspiring the integration
- The git worktree feature for making parallel development possible
- The open source community for the amazing tools this project builds upon

## 📞 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/your-username/claudectl/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/your-username/claudectl/discussions)
- 📖 **Documentation**: [Wiki](https://github.com/your-username/claudectl/wiki)

---

**Made with ❤️ for developers who love parallel workflows**