import { z } from "zod";

const positionSchema = z.object({
  line: z.number().min(0),
  character: z.number().min(0),
});

export const fragmentSchema = z.object({
  source: z.string(),
  start: positionSchema,
  end: positionSchema,
});

export const diagnosticSchema = z.object({
    code: z.number().min(0),
    severity: z.string(),
    ranges: z.array(fragmentSchema),
    description: z.string(),
})

export type Fragment = z.infer<typeof fragmentSchema>
export type Diagnostic = z.infer<typeof diagnosticSchema>