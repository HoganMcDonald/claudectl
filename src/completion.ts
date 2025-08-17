import tabtab from "tabtab";
import {
  hasClaudectlConfig,
  loadProjectConfig,
  getProjectWorktrees,
  getWorktreeName,
} from "./utils";

/**
 * Handle tab completion for claudectl commands.
 */
export async function handleCompletion(): Promise<void> {
  const env = tabtab.parseEnv(process.env);
  
  if (!env.complete) {
    return;
  }

  let completions: string[] = [];

  // Handle completion based on command context
  const words = Array.isArray(env.words) ? env.words : [];
  switch (words.length) {
    case 1:
      // Completing main commands
      completions = ['init', 'new', 'list', 'rm', 'install-completion', 'uninstall-completion'];
      break;
      
    case 2: {
      // Completing subcommands/options
      const command = words[1];
      
      switch (command) {
        case 'init':
          // Project name completion - no specific suggestions
          completions = [];
          break;
          
        case 'new':
          // Worktree name completion - no specific suggestions (auto-generated)
          completions = [];
          break;
          
        case 'rm':
          // Complete with available session names
          try {
            const currentDir = process.cwd();
            if (hasClaudectlConfig(currentDir)) {
              const projectConfig = loadProjectConfig(currentDir);
              const worktrees = getProjectWorktrees(projectConfig.name, currentDir);
              
              completions = worktrees
                .filter(w => !w.isMain) // Don't suggest main repo for removal
                .map(w => getWorktreeName(w.path, projectConfig.name))
                .filter((name): name is string => name !== null);
            }
          } catch {
            // If there's an error, just return empty completions
            completions = [];
          }
          break;
          
        case 'list':
          // No arguments for list command
          completions = [];
          break;
          
        default:
          completions = [];
      }
      break;
    }
      
    case 3: {
      // Handle flags and additional arguments
      const thirdCommand = words[1];
      
      if (thirdCommand === 'rm') {
        // Complete with --force flag
        completions = ['--force', '-f'];
      }
      break;
    }
      
    default:
      completions = [];
  }

  // Filter completions based on current partial input
  if (env.partial) {
    completions = completions.filter(completion => 
      completion.startsWith(env.partial)
    );
  }

  tabtab.log(completions);
}

/**
 * Install tab completion for the current shell.
 */
export async function installCompletion(): Promise<void> {
  try {
    await tabtab.install({
      name: 'claudectl',
      completer: 'claudectl',
    });
    console.log('✓ Tab completion installed successfully!');
    console.log('  Restart your shell or run: source ~/.bashrc (or ~/.zshrc)');
  } catch (error) {
    console.error('✗ Failed to install tab completion:', error);
    process.exit(1);
  }
}

/**
 * Uninstall tab completion for the current shell.
 */
export async function uninstallCompletion(): Promise<void> {
  try {
    await tabtab.uninstall({
      name: 'claudectl',
    });
    console.log('✓ Tab completion uninstalled successfully!');
  } catch (error) {
    console.error('✗ Failed to uninstall tab completion:', error);
    process.exit(1);
  }
}