import { spawn, ChildProcess } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";
import { error, info, success, step } from "./output";

/**
 * Options for starting a Claude Code session
 */
export interface ClaudeSessionOptions {
  workingDirectory: string;
  sessionName: string;
  useContainer?: boolean;
  dangerouslySkipPermissions?: boolean;
}

/**
 * Information about a running Claude Code session
 */
export interface ClaudeSessionInfo {
  pid: number;
  sessionName: string;
  workingDirectory: string;
  startTime: Date;
  lastAccessed: Date;
  useContainer: boolean;
}

/**
 * Manager for Claude Code sessions
 */
export class ClaudeSessionManager {
  private static sessionsFile = path.join(os.homedir(), '.claudectl', 'sessions.json');

  /**
   * Ensure the sessions directory exists
   */
  private static ensureSessionsDir(): void {
    const sessionsDir = path.dirname(this.sessionsFile);
    if (!fs.existsSync(sessionsDir)) {
      fs.mkdirSync(sessionsDir, { recursive: true });
    }
  }

  /**
   * Load existing sessions from file
   */
  private static loadSessions(): Record<string, ClaudeSessionInfo> {
    this.ensureSessionsDir();
    
    if (!fs.existsSync(this.sessionsFile)) {
      return {};
    }
    
    try {
      const content = fs.readFileSync(this.sessionsFile, 'utf8');
      const sessions = JSON.parse(content);
      
      // Convert date strings back to Date objects
      Object.values(sessions).forEach((session: any) => {
        session.startTime = new Date(session.startTime);
        session.lastAccessed = new Date(session.lastAccessed || session.startTime);
      });
      
      return sessions;
    } catch (error) {
      console.warn('Failed to load sessions file, starting fresh');
      return {};
    }
  }

  /**
   * Save sessions to file
   */
  private static saveSessions(sessions: Record<string, ClaudeSessionInfo>): void {
    this.ensureSessionsDir();
    
    try {
      fs.writeFileSync(this.sessionsFile, JSON.stringify(sessions, null, 2));
    } catch (error) {
      console.warn('Failed to save sessions file:', error);
    }
  }

  /**
   * Check if Claude Code is available
   */
  static async isClaudeCodeAvailable(): Promise<boolean> {
    return new Promise((resolve) => {
      const child = spawn('claude', ['--version'], { stdio: 'pipe' });
      child.on('close', (code) => resolve(code === 0));
      child.on('error', () => resolve(false));
    });
  }

  /**
   * Start a new Claude Code session
   */
  static async startSession(options: ClaudeSessionOptions): Promise<ClaudeSessionInfo> {
    const { workingDirectory, sessionName, useContainer = true, dangerouslySkipPermissions = true } = options;

    // Check if Claude Code is available
    if (!(await this.isClaudeCodeAvailable())) {
      throw new Error('Claude Code is not available. Please install it first: https://docs.anthropic.com/en/docs/claude-code');
    }

    // Build command arguments
    const args: string[] = [];
    
    if (dangerouslySkipPermissions) {
      args.push('--dangerously-skip-permissions');
    }
    
    if (useContainer) {
      args.push('--container');
    }
    
    // Add the working directory
    args.push(workingDirectory);

    step(1, 1, `Starting Claude Code session for "${sessionName}"`);
    
    // Start Claude Code in background
    const child = spawn('claude', args, {
      detached: true,
      stdio: ['ignore', 'ignore', 'ignore'],
      cwd: workingDirectory
    });

    // Detach from parent process so it runs independently
    child.unref();

    const sessionInfo: ClaudeSessionInfo = {
      pid: child.pid!,
      sessionName,
      workingDirectory,
      startTime: new Date(),
      lastAccessed: new Date(),
      useContainer
    };

    // Save session info
    const sessions = this.loadSessions();
    sessions[sessionName] = sessionInfo;
    this.saveSessions(sessions);

    success(`Claude Code session started for "${sessionName}" (PID: ${child.pid})`);
    info(`Session running in ${useContainer ? 'container' : 'host'} mode`);
    if (dangerouslySkipPermissions) {
      info('Running with dangerously-skip-permissions flag');
    }

    return sessionInfo;
  }

  /**
   * Stop a Claude Code session
   */
  static async stopSession(sessionName: string): Promise<boolean> {
    const sessions = this.loadSessions();
    const session = sessions[sessionName];
    
    if (!session) {
      error(`No session found with name "${sessionName}"`);
      return false;
    }

    try {
      // Try to kill the process
      process.kill(session.pid, 'SIGTERM');
      
      // Remove from sessions
      delete sessions[sessionName];
      this.saveSessions(sessions);
      
      success(`Claude Code session "${sessionName}" stopped`);
      return true;
    } catch (err) {
      // Process might already be dead
      if (err instanceof Error && 'code' in err && err.code === 'ESRCH') {
        // Process doesn't exist, just remove from sessions
        delete sessions[sessionName];
        this.saveSessions(sessions);
        info(`Session "${sessionName}" was already stopped, removed from tracking`);
        return true;
      }
      
      error(`Failed to stop session "${sessionName}": ${err instanceof Error ? err.message : String(err)}`);
      return false;
    }
  }

  /**
   * List all tracked sessions
   */
  static listSessions(): ClaudeSessionInfo[] {
    const sessions = this.loadSessions();
    return Object.values(sessions);
  }

  /**
   * Get session info by name
   */
  static getSession(sessionName: string): ClaudeSessionInfo | null {
    const sessions = this.loadSessions();
    return sessions[sessionName] || null;
  }

  /**
   * Update the last accessed timestamp for a session
   */
  static updateLastAccessed(sessionName: string): boolean {
    const sessions = this.loadSessions();
    const session = sessions[sessionName];
    
    if (!session) {
      return false;
    }
    
    session.lastAccessed = new Date();
    sessions[sessionName] = session;
    this.saveSessions(sessions);
    
    return true;
  }

  /**
   * Clean up dead sessions from tracking
   */
  static cleanupSessions(): void {
    const sessions = this.loadSessions();
    const activeSessions: Record<string, ClaudeSessionInfo> = {};
    
    for (const [name, session] of Object.entries(sessions)) {
      try {
        // Check if process is still running
        process.kill(session.pid, 0);
        activeSessions[name] = session;
      } catch (err) {
        // Process is dead, don't include it
        info(`Cleaned up dead session: ${name}`);
      }
    }
    
    this.saveSessions(activeSessions);
  }
}