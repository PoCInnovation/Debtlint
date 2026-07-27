import { z } from "zod";

/** 
* @typedef {z.infer<typeof positionSchema>} Position
*/ 
const positionSchema = z.object({
  source: z.string(),
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
    severity: z.string(),
    code: z.number().min(0) ,
    ranges: z.array(rangeSchema),
    code_description: z.string(),
})
