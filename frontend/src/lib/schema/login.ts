import { z } from "zod";

export const loginSchema = z
  .object({
    nameOrEmail: z
      .string()
      .min(2, "Name or Email must be at least 2 characters")
      .max(100, "Name or Email must be less than 100 characters"),
    password: z
      .string()
      .min(8, "Password must be 8 - 128 characters long")
      .max(128, "Password is too long")
      .regex(
        /[A-Z]/,
        "Password must contain at least one uppercase letter",
      )
      .regex(
        /[a-z]/,
        "Password must contain at least one lowercase letter",
      )
      .regex(/[0-9]/, "Password must contain at least one number")
      .regex(
        /[^A-Za-z0-9]/,
        "Password must contain at least one special character",
      ),
  });
