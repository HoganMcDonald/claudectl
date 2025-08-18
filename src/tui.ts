#!/usr/bin/env node

import React from 'react';
import { render } from 'ink';
import { Dashboard } from './tui/components/Dashboard.js';
import { TUIOptions } from './core/types/session.js';

export async function startTUI(options: TUIOptions = {}): Promise<void> {
  const refreshRate = options.refreshRate ? parseInt(options.refreshRate) : 2000;
  
  render(React.createElement(Dashboard, { refreshRate }));
}

// Allow direct execution
if (import.meta.url === `file://${process.argv[1]}`) {
  startTUI().catch(console.error);
}