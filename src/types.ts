import { z } from "zod";

/****************************************
 * Config
 *  This file defines the configuration schema for the project.
 ****************************************/
export const ProjectConfigSchema = z.object({
  name: z.string().min(1, "Project name cannot be empty"),
});

export type ProjectConfig = z.infer<typeof ProjectConfigSchema>;
