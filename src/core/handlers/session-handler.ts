import * as path from "node:path";
import {
  isGitRepository,
  hasClaudectlConfig,
  loadProjectConfig,
  getProjectDir,
  createWorktree,
  getDefaultBranch,
  generateRandomName,
  getProjectWorktrees,
  getWorktreeName,
  type WorktreeInfo,
} from "../../utils.js";
import { ClaudeSessionManager, type ClaudeSessionInfo } from "../../claude-session.js";
import { SessionInfo, SessionResult, ClaudeStatus, GitStatus } from "../types/session.js";
import { spawn } from "node:child_process";

export class SessionHandler {
  /**
   * Create a new session with worktree and Claude Code
   */
  static async createSession(name?: string): Promise<SessionResult> {
    const currentDir = process.cwd();

    // Validation
    if (!isGitRepository(currentDir)) {
      return { success: false, error: "Current directory is not a git repository" };
    }

    if (!hasClaudectlConfig(currentDir)) {
      return { success: false, error: "Current directory is not a claudectl project" };
    }

    // Load project configuration
    let projectConfig: { name: string };
    try {
      projectConfig = loadProjectConfig(currentDir);
    } catch (_err) {
      return { success: false, error: "Failed to load project configuration" };
    }

    // Generate session name if not provided
    const sessionName = name || generateRandomName();
    
    // Get project directory path
    const projectDir = getProjectDir(projectConfig.name);
    const worktreePath = path.join(projectDir, sessionName);

    // Get default branch
    let defaultBranch: string;
    try {
      defaultBranch = getDefaultBranch(currentDir);
    } catch (err) {
      return { success: false, error: `Failed to determine default branch: ${err instanceof Error ? err.message : String(err)}` };
    }

    try {
      // Create the worktree
      createWorktree(worktreePath, sessionName, defaultBranch, currentDir);
      
      // Start Claude Code session
      const session = await ClaudeSessionManager.startSession({
        workingDirectory: worktreePath,
        sessionName,
        useContainer: true,
        dangerouslySkipPermissions: true
      });

      return { success: true, session };
    } catch (err) {
      return { success: false, error: err instanceof Error ? err.message : String(err) };
    }
  }

  /**
   * List all sessions with status information
   */
  static async listSessions(): Promise<SessionInfo[]> {
    const currentDir = process.cwd();

    // Validation
    if (!isGitRepository(currentDir) || !hasClaudectlConfig(currentDir)) {
      return [];
    }

    try {
      const projectConfig = loadProjectConfig(currentDir);
      
      // Get all worktrees (excluding main)
      const allWorktrees = getProjectWorktrees(projectConfig.name, currentDir);
      const worktrees = allWorktrees.filter(w => !w.isMain);

      // Clean up dead sessions and get active ones
      ClaudeSessionManager.cleanupSessions();
      const sessions = ClaudeSessionManager.listSessions();
      const sessionMap = new Map<string, ClaudeSessionInfo>();
      
      // Map sessions by name
      for (const session of sessions) {
        sessionMap.set(session.sessionName, session);
      }

      // Combine worktree and session information
      const sessionInfos: SessionInfo[] = [];
      
      for (const worktree of worktrees) {
        const name = getWorktreeName(worktree.path, projectConfig.name);
        if (!name) continue;

        const session = sessionMap.get(name);
        const claudeStatus = session ? await this.getClaudeStatus(session) : 'idle';
        const gitStatus = await this.getGitStatus(worktree.path);

        sessionInfos.push({
          pid: session?.pid || 0,
          sessionName: name,
          workingDirectory: worktree.path,
          startTime: session?.startTime || new Date(),
          lastAccessed: session?.lastAccessed || new Date(),
          useContainer: session?.useContainer || true,
          gitStatus,
          claudeStatus,
          branch: worktree.branch,
          lastCommit: worktree.commitMessage,
          isMain: worktree.isMain
        });
      }

      return sessionInfos;
    } catch (err) {
      console.warn('Failed to list sessions:', err);
      return [];
    }
  }

  /**
   * Remove a session
   */
  static async removeSession(name: string, force?: boolean): Promise<boolean> {
    try {
      // Check if session exists
      const session = ClaudeSessionManager.getSession(name);
      
      // Stop Claude session if it exists
      if (session) {
        await ClaudeSessionManager.stopSession(name);
      }

      // TODO: Add worktree removal logic here
      // This would need to be extracted from the rm command
      
      return true;
    } catch (err) {
      console.error('Failed to remove session:', err);
      return false;
    }
  }

  /**
   * Attach to a session
   */
  static async attachSession(name: string): Promise<void> {
    const session = ClaudeSessionManager.getSession(name);
    if (!session) {
      throw new Error(`Session "${name}" not found`);
    }

    // Update last accessed timestamp
    ClaudeSessionManager.updateLastAccessed(name);

    // Start Claude in foreground
    const child = spawn('claude', ['.'], { 
      cwd: session.workingDirectory,
      stdio: 'inherit'
    });
    
    // Wait for user to exit Claude
    await new Promise<void>((resolve, reject) => {
      child.on('close', () => resolve());
      child.on('error', (err) => reject(err));
    });
  }

  /**
   * Get Claude status for a session
   */
  private static async getClaudeStatus(session: ClaudeSessionInfo): Promise<ClaudeStatus> {
    try {
      // Check if process is still running
      process.kill(session.pid, 0);
      return 'active';
    } catch {
      return 'idle';
    }
  }

  /**
   * Get git status for a worktree
   */
  private static async getGitStatus(workingDirectory: string): Promise<GitStatus> {
    try {
      // Check working tree status
      const statusResult = await new Promise<boolean>((resolve) => {
        const child = spawn('git', ['diff-index', '--quiet', 'HEAD'], { 
          cwd: workingDirectory,
          stdio: 'pipe'
        });
        child.on('close', (code) => resolve(code === 0));
        child.on('error', () => resolve(true)); // Assume clean on error
      });

      return statusResult ? 'clean' : 'dirty';
    } catch {
      return 'clean';
    }
  }
}