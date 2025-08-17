import { Command } from "commander";
import { version } from "../package.json";
import { initCommand } from "./commands/init";
import { newCommand } from "./commands/new";

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

program.parse();
