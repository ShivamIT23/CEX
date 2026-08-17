import jwt, { type JwtPayload } from "jsonwebtoken";
import type { Request, Response, NextFunction } from "express";
import { type TokenPayload, type CustomJwtPayload, type AuthRequest } from "./state.js";

// 2. Enforce secret existence
const jwt_secret = process.env.JWT_SECRET;
if (!jwt_secret) {
  throw new Error("MISSING_ENV: JWT_SECRET is required");
}

// 3. Type the payload argument strictly
export const createToken = (payload: TokenPayload): string => {
  // Options: set expiration (e.g., '1h')
  return jwt.sign(payload, jwt_secret, { expiresIn: '1h' });
};

export const authenticate = (req: AuthRequest, res: Response, next: NextFunction) => {
  const authHeader = req.headers['authorization'];
  
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return res.status(401).json({ error: 'No token provided' });
  }

  const token = authHeader.split(' ')[1]??"";

  try {
    // 4. Use the same constant 'jwt_secret' here
    const decoded = jwt.verify(token, jwt_secret) as unknown as CustomJwtPayload;
  
    req.user = decoded;
    next();
  } catch (error) {
    return res.status(403).json({ error: 'Invalid token' });
  }
};   