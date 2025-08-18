export interface ClaudeSessionOptions {
  workingDirectory: string;
  sessionName: string;
  useContainer?: boolean;
  dangerouslySkipPermissions?: boolean;
}

export interface ClaudeSessionInfo {
  pid: number;
  sessionName: string;
  workingDirectory: string;
  startTime: Date;
  lastAccessed: Date;
  useContainer: boolean;
}

export type ClaudeStatus = 'active' | 'waiting' | 'idle' | 'error';
export type GitStatus = 'clean' | 'dirty' | 'ahead' | 'behind';

export interface SessionInfo extends ClaudeSessionInfo {
  gitStatus: GitStatus;
  claudeStatus: ClaudeStatus;
  selected?: boolean;
  branch?: string;
  lastCommit?: string;
  isMain?: boolean;
}

export interface SessionStatus {
  claude: ClaudeStatus;
  git: GitStatus;
  lastActivity?: Date;
}

export interface SessionResult {
  success: boolean;
  session?: ClaudeSessionInfo;
  error?: string;
}

export interface TUIOptions {
  refreshRate?: string;
}