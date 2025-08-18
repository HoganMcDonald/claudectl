import React from 'react';
import { Box, Text } from 'ink';

export const ActionBar: React.FC = () => {
  return (
    <Box borderStyle="single" borderColor="gray" padding={1}>
      <Box flexDirection="column" width="100%">
        <Box justifyContent="space-between">
          <Text color="green">[n] new session</Text>
          <Text color="blue">[a] attach</Text>
          <Text color="red">[d] delete</Text>
          <Text color="yellow">[r] refresh</Text>
        </Box>
        <Box justifyContent="space-between" marginTop={1}>
          <Text color="cyan">[enter] attach to selected</Text>
          <Text color="gray">[↑↓] navigate</Text>
          <Text color="magenta">[q] quit</Text>
        </Box>
      </Box>
    </Box>
  );
};