import tabtab from "tabtab";
import {
  getProjectWorktrees,
  getWorktreeName,
  hasClaudectlConfig,
  loadProjectConfig,
} from "./utils.js";

/**
 * Handle tab completion for claudectl commands.
 * This provides completions through tabtab when completion environment is detected.
 */
export async function handleCompletion(): Promise<void> {
  const env = tabtab.parseEnv(process.env);

  if (!env.complete) {
    return;
  }

  let completions: string[] = [];

  // Parse the command line to get words
  const words = (env.line || "").trim().split(/\s+/);
  const wordCount = words.length;

  // Main commands completion (when typing just "claudectl " or partial command)
  if (wordCount === 1 || (wordCount === 2 && !(env.line || "").endsWith(" "))) {
    completions = ["init", "new", "list", "rm", "attach", "tui"];
  }
  // Arguments and options completion
  else if (wordCount >= 2) {
    const command = words[1];

    switch (command) {
      case "rm":
        if (wordCount === 2 || (wordCount === 3 && !(env.line || "").endsWith(" "))) {
          // Complete with available session names
          completions = getAvailableSessions();
        } else if (wordCount >= 3) {
          // Complete with flags
          completions = ["--force", "-f"];
        }
        break;

      case "attach":
        if (wordCount === 2 || (wordCount === 3 && !(env.line || "").endsWith(" "))) {
          // Complete with available session names
          completions = getAvailableSessions();
        }
        break;

      case "init":
      case "new":
      case "list":
      case "tui":
      case "completion":
        // These commands don't have specific completions
        completions = [];
        break;
    }
  }

  // Filter completions based on current partial input
  if (env.last?.trim()) {
    completions = completions.filter((completion) =>
      completion.startsWith((env.last || "").trim())
    );
  }

  tabtab.log(completions);

  // Exit immediately to prevent command execution (skip in tests)
  if (process.env.NODE_ENV !== 'test') {
    process.exit(0);
  }
}

/**
 * Get available session names for completion
 */
function getAvailableSessions(): string[] {
  try {
    const currentDir = process.cwd();
    if (!hasClaudectlConfig(currentDir)) {
      return [];
    }

    const projectConfig = loadProjectConfig(currentDir);
    const worktrees = getProjectWorktrees(projectConfig.name, currentDir);

    return worktrees
      .filter((w) => !w.isMain) // Don't suggest main repo for removal
      .map((w) => getWorktreeName(w.path, projectConfig.name))
      .filter((name): name is string => name !== null);
  } catch {
    return [];
  }
}

/**
 * Install tab completion for claudectl
 */
export async function installCompletion(): Promise<void> {
  try {
    await tabtab.install({
      name: 'claudectl',
      completer: 'claudectl',
    });
    
    if (!process.env.CLAUDECTL_AUTO_INSTALL) {
      console.log('Tab completion installed successfully');
    }
  } catch (error) {
    if (process.env.CLAUDECTL_AUTO_INSTALL) {
      throw new Error('Installation failed');
    }
    
    console.error('Failed to install tab completion:', error);
    process.exit(1);
  }
}

/**
 * Uninstall tab completion for claudectl
 */
export async function uninstallCompletion(): Promise<void> {
  try {
    await tabtab.uninstall({
      name: 'claudectl',
    });
    
    console.log('Tab completion uninstalled successfully');
  } catch (error) {
    console.error('Failed to uninstall tab completion:', error);
    process.exit(1);
  }
}
