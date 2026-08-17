import express from 'express';

const app = express();

let USER_INDEX = 0;

const USERS:{id: number, username: string, password: string}[] = [];

app.get('/', (req, res) => {
  res.send('Hello, World!23');
});

app.post("/signup",(req,res)=>{
    const {username,password} = req.body;
    if (!username || !password) {
        return res.status(400).send("Username and password are required");
    }
    if (USERS.find(user => user.username === username)) {
        return res.status(400).send("Username already exists");
    }
    const user = {id: USER_INDEX++, username, password};
    USERS.push(user);
    res.send("User signed up successfully");
})

app.listen(3000, () => {
  console.log('Server is running on port 3000');
});