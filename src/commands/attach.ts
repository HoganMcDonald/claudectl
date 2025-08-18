import { spawn } from "node:child_process";
import { ClaudeSessionManager } from "../claude-session.js";
import {
  error,
  info,
  success,
  instruction,
} from "../output.js";

/**
 * Attaches to an existing session by starting Claude Code in the session's directory.
 * This is idempotent - it will always result in a foreground Claude session.
 *
 * @param sessionName - Name of the session to attach to.
 */
export const attachCommand = async (sessionName: string): Promise<void> => {
  // Validate session name is provided
  if (!sessionName || sessionName.trim().length === 0) {
    error("session name is required");
    instruction(
      "Specify the name of the session to attach to:",
      ["claudectl attach brave-penguin", "claudectl attach feature-auth"]
    );
    process.exit(1);
  }

  // Check if session exists
  const session = ClaudeSessionManager.getSession(sessionName);
  if (!session) {
    error(`Session "${sessionName}" not found`);
    instruction(
      "Create the session first or choose an existing one:",
      [`claudectl new ${sessionName}`, "claudectl list"]
    );
    process.exit(1);
  }

  // Update last accessed timestamp
  ClaudeSessionManager.updateLastAccessed(sessionName);

  // Start Claude in foreground in the session directory
  info(`Attaching to session "${sessionName}"`);
  
  try {
    const child = spawn('claude', ['.'], { 
      cwd: session.workingDirectory,
      stdio: 'inherit' // User controls the session directly
    });
    
    // Wait for user to exit Claude
    await new Promise<void>((resolve, reject) => {
      child.on('close', (code) => {
        resolve();
      });
      
      child.on('error', (err) => {
        reject(err);
      });
    });
    
    success(`Detached from session "${sessionName}"`);
    
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    error(`Failed to start Claude Code: ${errorMessage}`);
    
    if (errorMessage.includes('ENOENT')) {
      instruction(
        "Claude Code is not installed or not in PATH:",
        ["Install Claude Code from https://claude.ai/code"]
      );
    }
    
    process.exit(1);
  }
};