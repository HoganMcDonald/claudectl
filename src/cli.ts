import { Command } from "commander";
import { version } from "../package.json";
import { initCommand } from "./commands/init";
import { newCommand } from "./commands/new";
import { listCommand } from "./commands/list";
import { rmCommand } from "./commands/rm";
import { handleCompletion, installCompletion, uninstallCompletion } from "./completion";

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
  .action((name?: string) => {
    newCommand(name);
  });

program
  .command("list")
  .description("List all active worktrees for the current project")
  .action(() => {
    listCommand();
  });

program
  .command("rm")
  .description("Remove a session/worktree by name")
  .argument("<name>", "Name of the session to remove")
  .option("-f, --force", "Force removal even if session has uncommitted changes")
  .action((name: string, options: { force?: boolean }) => {
    rmCommand(name, options);
  });

program
  .command("install-completion")
  .description("Install tab completion for your shell")
  .action(async () => {
    await installCompletion();
  });

program
  .command("uninstall-completion")
  .description("Uninstall tab completion for your shell")
  .action(async () => {
    await uninstallCompletion();
  });

// Handle tab completion before parsing
handleCompletion().then(() => {
  program.parse();
});
