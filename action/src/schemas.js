import { z } from "zod";

const positionSchema = z.object({
  line: z.number().min(0),
  character: z.number().min(0),
});

const rangeSchema = z.object({
  start: positionSchema,
  end: positionSchema,
});

export const diagnosticSchema = z.object({
    source: z.string(),
    severity: z.string(),
    code: z.number().min(0) ,
    ranges: z.array(rangeSchema),
    codeDescription: z.string().optional(),
})
