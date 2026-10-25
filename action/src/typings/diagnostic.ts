import { z } from "zod";

const rangeSchema = z.object({
    start: z.number().int().min(0),
    end: z.number().int().min(0),
  })
  .refine((range) => range.end >= range.start);

export const fragmentSchema = z.object({
  source: z.string(),
  tokens: rangeSchema,
  chars: rangeSchema,
  lines: rangeSchema.nullable(),
});

export const diagnosticSchema = z.object({
  ranges: z.array(fragmentSchema).min(2),
});

export type Fragment = z.infer<typeof fragmentSchema>
export type Diagnostic = z.infer<typeof diagnosticSchema>
