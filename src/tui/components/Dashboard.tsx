import React, { useState } from 'react';
import { Box, Text, useApp } from 'ink';
import { SessionProvider, useSession } from '../providers/SessionProvider.js';
import { SessionList } from './SessionList.js';
import { ActionBar } from './ActionBar.js';
import { Details } from './Details.js';
import { useKeyBindings } from '../hooks/useKeyBindings.js';

interface DashboardProps {
  refreshRate?: number;
}

const DashboardContent: React.FC = () => {
  const { exit } = useApp();
  const { sessions, selectedIndex, loading } = useSession();
  const [projectName] = useState('claudectl'); // TODO: Get from config
  
  const selectedSession = sessions.length > 0 ? sessions[selectedIndex] : null;

  useKeyBindings({
    onQuit: () => exit(),
    onNewSession: () => {
      // TODO: Implement new session modal
      console.log('New session requested');
    },
    onDeleteSession: () => {
      // TODO: Implement delete confirmation modal
      console.log('Delete session requested');
    },
    onAttachSession: () => {
      // TODO: Implement attach prompt modal
      console.log('Attach session requested');
    },
    onRefresh: () => {
      console.log('Refresh requested');
    }
  });

  return (
    <Box flexDirection="column" width="100%" height="100%">
      {/* Header */}
      <Box justifyContent="space-between" paddingX={1} paddingY={1}>
        <Text color="cyan" bold>claudectl TUI</Text>
        <Text color="gray">[q] quit [?] help</Text>
      </Box>

      {/* Project Info */}
      <Box paddingX={1}>
        <Text>
          <Text color="gray">Sessions for project: </Text>
          <Text color="white" bold>{projectName}</Text>
          {loading && <Text color="yellow"> (loading...)</Text>}
        </Text>
      </Box>

      {/* Main Content */}
      <Box flexDirection="column" paddingX={1} paddingY={1} flexGrow={1}>
        {/* Sessions List */}
        <Box borderStyle="single" borderColor="cyan" padding={1} marginBottom={1}>
          <Box flexDirection="column" width="100%">
            <Text color="cyan" bold>Active Sessions</Text>
            <SessionList sessions={sessions} selectedIndex={selectedIndex} />
          </Box>
        </Box>

        {/* Session Details */}
        <Details session={selectedSession} projectName={projectName} />
      </Box>

      {/* Action Bar */}
      <ActionBar />
    </Box>
  );
};

export const Dashboard: React.FC<DashboardProps> = ({ refreshRate = 2000 }) => {
  return (
    <SessionProvider refreshRate={refreshRate}>
      <DashboardContent />
    </SessionProvider>
  );
};