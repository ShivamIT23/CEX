import express from 'express';
import jwt from "jsonwebtoken";
import { authenticate, createToken } from './authenticate.js';
import USERS, { USER_INDEX, increment_index, type UserState, type AuthRequest } from './state.js';

const app = express();

const jwt_secret = process.env.JWT_SECRET || "cex2026";



app.get('/', (req, res) => {
  res.send('Hello, World!23');
});

app.post("/signup", (req, res) => {
  const { username, password } = req.body;
  if (!username || !password) {
    return res.status(400).send("Username and password are required");
  }
  if (USERS.find(user => user.username === username)) {
    return res.status(400).send("Username already exists");
  }
  const user = { id: USER_INDEX, username, password, usd: 0, eth: 0, sol: 0 };
  increment_index();
  USERS.push(user);
  res.send("User signed up successfully");
})

app.post("/signin", (req, res) => {
  const { username, password } = req.body;
  if (!username || !password) {
    return res.status(400).send("Username and password are required");
  }
  const user = USERS.find(user => user.username === username);
  if (user) {
    const payload = {
      userId: user.id,
    }
    const token = createToken(payload);
    return res.status(200).send({
      token
    })
  }
  res.status(400).send("UnAuthorized");
})

app.use(authenticate);

app.post("/balance/onramp", (req: AuthRequest, res) => {
  const { amount } = req.body;

  if (!amount || typeof amount !== 'number' || amount <= 0) {
    return res.status(400).send("Invalid amount");
  }

  if (!req.user?.id) {
    return res.status(401).send("UnAuthorized");
  }

  const user = USERS.find(u => u.id === req.user!.id);

  if (user) {
    // 4. Use the correct variable 'asset' to access the property
    user.usd += amount;
    return res.status(200).send({
      asset: user.usd,
      type: "usd"
    });
  }

  res.status(404).send("User not found");
});

app.post("/balance/deposit", (req: AuthRequest, res) => {
  const { assetType, quantity } = req.body;

  if (!quantity || typeof quantity !== 'number' || quantity <= 0) {
    return res.status(400).send("Invalid quantity");
  }

  if (!req.user?.id) {
    return res.status(401).send("UnAuthorized");
  }

  const allowedAssets = ["sol", "eth", "usd"] as const;

  type NumericAsset = typeof allowedAssets[number];

  if (!assetType || allowedAssets.includes(assetType as any)) {
    return res.status(400).send("Needs the token to get balance");
  }
  const asset = assetType as NumericAsset;

  const user = USERS.find(u => u.id === req.user!.id);

  if (user) {
     if (typeof user[asset] !== 'number') {
        return res.status(500).send("Internal error: Asset field is not a number");
    }

    user[asset] += quantity; // ✅ No more error: both sides are guaranteed to be numbers

    return res.status(200).send({
      asset: user[asset],
      type: asset
    });
  }

  res.status(404).send("User not found");
});

app.post("/order", (req: AuthRequest, res) => {
  const { market, action, qty, price, type } = req.body;

  // if (!price || typeof price !== 'number' || price <= 0) {
  //   return res.status(400).send("Invalid quantity");
  // }

  // if (!req.user?.id) {
  //   return res.status(401).send("UnAuthorized");
  // }

  // const allowedAssets = ["sol", "eth", "usd"] as const;

  // type NumericAsset = typeof allowedAssets[number];

  // if (!assetType || allowedAssets.includes(assetType as any)) {
  //   return res.status(400).send("Needs the token to get balance");
  // }
  // const asset = assetType as NumericAsset;

  // const user = USERS.find(u => u.id === req.user!.id);

  // if (user) {
  //    if (typeof user[asset] !== 'number') {
  //       return res.status(500).send("Internal error: Asset field is not a number");
  //   }

  //   user[asset] += quantity; // ✅ No more error: both sides are guaranteed to be numbers

  //   return res.status(200).send({
  //     asset: user[asset],
  //     type: asset
  //   });
  // }

  res.status(404).send("User not found");
});

app.get("/balance", (req: AuthRequest, res) => {
  const assetParam = req.query.asset as string;

  const allowedAssets = ["sol", "eth", "usd"] as const;

  if (!assetParam || allowedAssets.includes(assetParam as any)) {
    return res.status(400).send("Needs the token to get balance");
  }
  const asset = assetParam as keyof UserState;

  if (!req.user?.id) {
    return res.status(401).send("UnAuthorized");
  }

  const user = USERS.find(u => u.id === req.user!.id);

  if (user) {
    // 4. Use the correct variable 'asset' to access the property
    return res.status(200).send({
      asset: user[asset] ?? 0,
      type: asset
    });
  }

  res.status(404).send("User not found");
});

app.listen(3000, () => {
  console.log('Server is running on port 3000');
});