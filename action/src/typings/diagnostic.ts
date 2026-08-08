import { z } from "zod";

const positionSchema = z.object({
  source: z.string(),
  line: z.number().min(0),
  character: z.number().min(0),
});

const rangeSchema = z.object({
  start: positionSchema,
  end: positionSchema,
});

export const diagnosticSchema = z.object({
    code: z.number().min(0),
    severity: z.string(),
    ranges: z.array(rangeSchema),
    description: z.string(),
})

export type Diagnostic = z.infer<typeof diagnosticSchema>