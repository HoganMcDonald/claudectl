import { Box, Text } from "ink";
import type React from "react";
import type { SessionInfo } from "../../core/types/session.js";

interface DetailsProps {
  session: SessionInfo | null;
  projectName?: string;
}

const formatPath = (path: string): string => {
  const home = process.env.HOME || "";
  return path.replace(home, "~");
};

export const Details: React.FC<DetailsProps> = ({ session, projectName: _projectName }) => {
  if (!session) {
    return (
      <Box borderStyle="single" borderColor="gray" padding={1} height={6}>
        <Text color="gray">Select a session to view details</Text>
      </Box>
    );
  }

  return (
    <Box
      borderStyle="single"
      borderColor="gray"
      padding={1}
      flexDirection="column"
      height={6}
    >
      <Text color="cyan">Session Details</Text>
      <Text>
        <Text color="gray">Session: </Text>
        <Text color="white">{session.sessionName}</Text>
      </Text>
      <Text>
        <Text color="gray">Branch: </Text>
        <Text color="yellow">{session.branch || "main"}</Text>
      </Text>
      <Text>
        <Text color="gray">Path: </Text>
        <Text color="blue">{formatPath(session.workingDirectory)}</Text>
      </Text>
      {session.lastCommit && (
        <Text>
          <Text color="gray">Last commit: </Text>
          <Text color="white">
            {session.lastCommit.slice(0, 50)}
            {session.lastCommit.length > 50 ? "..." : ""}
          </Text>
        </Text>
      )}
    </Box>
  );
};
