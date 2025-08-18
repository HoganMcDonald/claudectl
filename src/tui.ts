#!/usr/bin/env node

import { render } from "ink";
import React from "react";
import type { TUIOptions } from "./core/types/session.js";
import { Dashboard } from "./tui/components/Dashboard.js";

export async function startTUI(options: TUIOptions = {}): Promise<void> {
  const refreshRate = options.refreshRate
    ? parseInt(options.refreshRate, 10)
    : 2000;

  render(React.createElement(Dashboard, { refreshRate }));
}

// Allow direct execution
if (import.meta.url === `file://${process.argv[1]}`) {
  startTUI().catch(console.error);
}
