import { useInput } from 'ink';
import { useSession } from '../providers/SessionProvider.js';

interface KeyBindingOptions {
  onQuit: () => void;
  onNewSession: () => void;
  onDeleteSession: () => void;
  onAttachSession: () => void;
  onRefresh: () => void;
}

export const useKeyBindings = (options: KeyBindingOptions) => {
  const { 
    sessions, 
    selectedIndex, 
    setSelectedIndex, 
    attachSession, 
    refreshSessions 
  } = useSession();

  useInput((input: string, key: any) => {
    // Quit
    if (input === 'q' || (key.ctrl && input === 'c')) {
      options.onQuit();
      return;
    }

    // Navigation
    if (key.downArrow || input === 'j') {
      if (sessions.length > 0) {
        setSelectedIndex((selectedIndex + 1) % sessions.length);
      }
      return;
    }

    if (key.upArrow || input === 'k') {
      if (sessions.length > 0) {
        setSelectedIndex(selectedIndex === 0 ? sessions.length - 1 : selectedIndex - 1);
      }
      return;
    }

    // Actions
    if (key.return) {
      if (sessions.length > 0 && sessions[selectedIndex]) {
        attachSession(sessions[selectedIndex].sessionName).catch(console.error);
      }
      return;
    }

    if (input === 'n') {
      options.onNewSession();
      return;
    }

    if (input === 'd' || key.delete) {
      options.onDeleteSession();
      return;
    }

    if (input === 'a') {
      options.onAttachSession();
      return;
    }

    if (input === 'r') {
      options.onRefresh();
      refreshSessions();
      return;
    }
  });
};