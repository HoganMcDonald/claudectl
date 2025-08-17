#!/usr/bin/env node

import { Command } from "commander";
import { version } from "../package.json";
import { initCommand } from "./commands/init";
import { newCommand } from "./commands/new";
import { listCommand } from "./commands/list";
import { rmCommand } from "./commands/rm";
import { handleCompletion } from "./completion";

const program = new Command();

program
  .name("claudectl")
  .description("A CLI tool for orchestrating coding agents")
  .version(version);

program
  .command("init")
  .description("Initialize a new claudectl project in current directory")
  .argument("[name]", "Project name (defaults to current directory name)")
  .action((name?: string) => {
    initCommand(name);
  });

program
  .command("new")
  .description("Create a new worktree from the latest main/master branch")
  .argument("[name]", "Worktree name (defaults to auto-generated name)")
  .action(async (name?: string) => {
    await newCommand(name);
  });

program
  .command("list")
  .description("List all active worktrees for the current project")
  .action(async () => {
    await listCommand();
  });

program
  .command("rm")
  .description("Remove a session/worktree by name")
  .argument("<name>", "Name of the session to remove")
  .option("-f, --force", "Force removal even if session has uncommitted changes")
  .action(async (name: string, options: { force?: boolean }) => {
    await rmCommand(name, options);
  });

// Hidden completion command for tabtab
program
  .command("completion", { hidden: true })
  .description("Generate completion script")
  .action(() => {
    // When called normally (not for completion), show a helpful message
    // But when called for completion, this won't execute because
    // handleCompletion() will have already handled it and exited
    console.log('Tab completion is handled automatically by the shell.');
    console.log('To set up completion, use: tabtab install --name claudectl --completer claudectl');
  });

// Handle tab completion before parsing
handleCompletion().then(() => {
  program.parse();
});
