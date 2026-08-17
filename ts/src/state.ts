import { type JwtPayload } from "jsonwebtoken";
import type { Request } from "express";


export interface UserState {
    id: number,
    username: string,
    password: string,
    usd: number,
    sol: number,
    eth: number,
    [key: string]: number | string | undefined; 
};

const USERS: UserState[] = [];

export interface TokenPayload {
  userId: number;
  // Add other fields you need, e.g., role?: string;
}

export interface CustomJwtPayload extends JwtPayload, TokenPayload {}

export interface AuthRequest extends Request {
  user?: CustomJwtPayload;
}

export let USER_INDEX = 0;

export const increment_index = () => {
    USER_INDEX++;
}

export default USERS;