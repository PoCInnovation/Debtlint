import { z } from "zod";

/** 
* @typedef {z.infer<typeof positionSchema>} Position
*/ 
const positionSchema = z.object({
  line: z.number().min(0),
  character: z.number().min(0),
});

/** 
* @typedef {z.infer<typeof rangeSchema>} Range
*/ 
const rangeSchema = z.object({
  start: positionSchema,
  end: positionSchema,
});

/** 
* @typedef {z.infer<typeof diagnosticSchema>} Diagnostic
*/ 
export const diagnosticSchema = z.object({
    source: z.string(),
    severity: z.string(),
    code: z.number().min(0) ,
    ranges: z.array(rangeSchema),
    codeDescription: z.string().optional(),
})
