import React, { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import { SessionInfo } from '../../core/types/session.js';
import { SessionHandler } from '../../core/handlers/session-handler.js';

interface SessionContextType {
  sessions: SessionInfo[];
  selectedIndex: number;
  setSelectedIndex: (index: number) => void;
  refreshSessions: () => Promise<void>;
  createSession: (name?: string) => Promise<void>;
  removeSession: (name: string, force?: boolean) => Promise<void>;
  attachSession: (name: string) => Promise<void>;
  loading: boolean;
}

const SessionContext = createContext<SessionContextType | null>(null);

interface SessionProviderProps {
  children: ReactNode;
  refreshRate?: number;
}

export const SessionProvider: React.FC<SessionProviderProps> = ({ 
  children, 
  refreshRate = 2000 
}) => {
  const [sessions, setSessions] = useState<SessionInfo[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [loading, setLoading] = useState(true);

  const refreshSessions = useCallback(async () => {
    try {
      const sessionList = await SessionHandler.listSessions();
      setSessions(sessionList);
      
      // Adjust selected index if needed
      if (selectedIndex >= sessionList.length && sessionList.length > 0) {
        setSelectedIndex(sessionList.length - 1);
      }
    } catch (error) {
      console.error('Failed to refresh sessions:', error);
    } finally {
      setLoading(false);
    }
  }, [selectedIndex]);

  const createSession = useCallback(async (name?: string) => {
    setLoading(true);
    try {
      await SessionHandler.createSession(name);
      await refreshSessions();
    } catch (error) {
      console.error('Failed to create session:', error);
    }
  }, [refreshSessions]);

  const removeSession = useCallback(async (name: string, force?: boolean) => {
    setLoading(true);
    try {
      await SessionHandler.removeSession(name, force);
      await refreshSessions();
    } catch (error) {
      console.error('Failed to remove session:', error);
    }
  }, [refreshSessions]);

  const attachSession = useCallback(async (name: string) => {
    try {
      await SessionHandler.attachSession(name);
      await refreshSessions();
    } catch (error) {
      console.error('Failed to attach to session:', error);
    }
  }, [refreshSessions]);

  // Auto-refresh sessions
  useEffect(() => {
    refreshSessions(); // Initial load
    
    const interval = setInterval(refreshSessions, refreshRate);
    return () => clearInterval(interval);
  }, [refreshSessions, refreshRate]);

  return (
    <SessionContext.Provider value={{
      sessions,
      selectedIndex,
      setSelectedIndex,
      refreshSessions,
      createSession,
      removeSession,
      attachSession,
      loading
    }}>
      {children}
    </SessionContext.Provider>
  );
};

export const useSession = (): SessionContextType => {
  const context = useContext(SessionContext);
  if (!context) {
    throw new Error('useSession must be used within a SessionProvider');
  }
  return context;
};