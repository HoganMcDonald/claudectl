import { Box, Text } from "ink";
import type React from "react";
import type { SessionInfo } from "../../core/types/session.js";

interface SessionListProps {
  sessions: SessionInfo[];
  selectedIndex: number;
}

const getStatusIcon = (claudeStatus: string): string => {
  switch (claudeStatus) {
    case "active":
      return "🟢";
    case "waiting":
      return "🟡";
    case "error":
      return "⚠️";
    default:
      return "🔴";
  }
};

const getGitIcon = (gitStatus: string): string => {
  switch (gitStatus) {
    case "clean":
      return "✅";
    case "dirty":
      return "📝";
    case "ahead":
      return "🔀";
    case "behind":
      return "⬇️";
    default:
      return "❓";
  }
};

const formatLastAccessed = (date: Date): string => {
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / (1000 * 60));

  if (diffMins < 1) return "now";
  if (diffMins < 60) return `${diffMins}m ago`;

  const diffHours = Math.floor(diffMins / 60);
  if (diffHours < 24) return `${diffHours}h ago`;

  const diffDays = Math.floor(diffHours / 24);
  return `${diffDays}d ago`;
};

export const SessionList: React.FC<SessionListProps> = ({
  sessions,
  selectedIndex,
}) => {
  if (sessions.length === 0) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="gray">No sessions found</Text>
        <Text color="gray">Press 'n' to create a new session</Text>
      </Box>
    );
  }

  return (
    <Box flexDirection="column">
      {sessions.map((session, index) => {
        const isSelected = index === selectedIndex;
        const statusIcon = getStatusIcon(session.claudeStatus);
        const gitIcon = getGitIcon(session.gitStatus);
        const lastAccessed = formatLastAccessed(session.lastAccessed);

        return (
          <Box key={session.sessionName} paddingX={1}>
            <Text color={isSelected ? "cyan" : "white"}>
              {isSelected ? "●" : "○"} {session.sessionName}
            </Text>
            <Box marginLeft={2}>
              <Text color="gray">[{session.branch || "main"}]</Text>
            </Box>
            <Box marginLeft={2}>
              <Text>
                {statusIcon} {session.claudeStatus}
              </Text>
            </Box>
            <Box marginLeft={2}>
              <Text>
                {gitIcon} {session.gitStatus}
              </Text>
            </Box>
            <Box marginLeft={2}>
              <Text color="gray">{lastAccessed}</Text>
            </Box>
          </Box>
        );
      })}
    </Box>
  );
};
